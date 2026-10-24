//! The client against a scripted keyboard: one request at a time, timeouts, correlation,
//! notifications, errors, validation, and losing the connection.
//!
//! The tests run on a paused clock, so a two-second timeout takes no time. Every wait for
//! the client goes through [`within`], so a client that never sends or answers fails a test
//! instead of hanging it.

mod replay;

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use tokio::sync::{broadcast, mpsc};
use tokio::time::Instant;

use super::*;
use crate::message::{KeyPhysicalAttrs, KeymapNotification, PhysicalLayout};

/// Waits for `future`, or fails the test after a minute. On the paused clock, that minute
/// passes as soon as nothing else can happen.
async fn within<T>(future: impl Future<Output = T>) -> T {
    tokio::time::timeout(Duration::from_secs(60), future)
        .await
        .expect("the test waited for something that never happened")
}

/// The client's end of an in-memory connection.
struct TestTransport {
    to_keyboard: mpsc::UnboundedSender<Vec<u8>>,
    from_keyboard: mpsc::UnboundedReceiver<Vec<u8>>,
}

impl Transport for TestTransport {
    fn send(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send {
        let sent = self
            .to_keyboard
            .send(bytes.to_vec())
            .map_err(|_| TransportError::Closed);
        std::future::ready(sent)
    }

    async fn recv(&mut self) -> Result<Vec<u8>, TransportError> {
        self.from_keyboard
            .recv()
            .await
            .ok_or(TransportError::Closed)
    }

    fn kind(&self) -> TransportKind {
        TransportKind::InMemory
    }
}

/// The keyboard's end, which the test drives.
struct Keyboard {
    from_client: mpsc::UnboundedReceiver<Vec<u8>>,
    to_client: mpsc::UnboundedSender<Vec<u8>>,
    decoder: FrameDecoder,
}

impl Keyboard {
    /// Waits for the client's next request.
    async fn next_request(&mut self) -> ClientMessage {
        loop {
            let chunk = within(self.from_client.recv())
                .await
                .expect("the client hung up");
            // Lone `EOF`s after a timeout come out as skipped bytes.
            let payload = self.decoder.decode(&chunk).into_iter().find_map(Result::ok);
            if let Some(payload) = payload {
                return ClientMessage::decode(&payload).expect("the client sent garbage");
            }
        }
    }

    /// Waits for the next chunk of bytes the client sends, as it is.
    async fn next_chunk(&mut self) -> Vec<u8> {
        within(self.from_client.recv())
            .await
            .expect("the client hung up")
    }

    /// Expects the client's next request to be `expected`, and answers it.
    async fn answer(&mut self, expected: Request, response: Response) {
        let request = self.next_request().await;
        assert_eq!(request.request, expected);
        self.respond(request.request_id, response);
    }

    /// Checks that the client has sent nothing more.
    fn assert_quiet(&mut self) {
        assert!(
            self.from_client.try_recv().is_err(),
            "the client sent bytes while it should wait"
        );
    }

    fn respond(&self, request_id: u32, response: Response) {
        self.send(&message(&DeviceMessage::Response {
            request_id,
            response,
        }));
    }

    fn notify(&self, notification: Notification) {
        self.send(&message(&DeviceMessage::Notification(notification)));
    }

    fn send(&self, bytes: &[u8]) {
        self.to_client
            .send(bytes.to_vec())
            .expect("the client hung up");
    }
}

/// A device message, framed.
fn message(message: &DeviceMessage) -> Vec<u8> {
    frame::encode(&message.encode())
}

/// Connects a client to a keyboard the test drives.
///
/// Dropping the keyboard closes the connection, so tests keep it until they're done: a
/// task that returns it is held as `_keyboard`, which keeps its output alive.
fn connect_at(first_request_id: u32) -> (Client, Keyboard) {
    let (transport, keyboard) = pipe();
    (Client::starting_at(transport, first_request_id), keyboard)
}

/// Connects through [`Client::new`], whose first request has ID 0.
fn connect() -> (Client, Keyboard) {
    let (transport, keyboard) = pipe();
    (Client::new(transport), keyboard)
}

/// The two ends of an in-memory connection.
fn pipe() -> (TestTransport, Keyboard) {
    let (to_keyboard, from_client) = mpsc::unbounded_channel();
    let (to_client, from_keyboard) = mpsc::unbounded_channel();
    let transport = TestTransport {
        to_keyboard,
        from_keyboard,
    };
    let keyboard = Keyboard {
        from_client,
        to_client,
        decoder: FrameDecoder::new(),
    };
    (transport, keyboard)
}

fn device_info() -> DeviceInfo {
    DeviceInfo {
        name: "Sofle Choc BT".to_owned(),
        serial_number: vec![0xDE, 0xAD],
    }
}

fn lock_state(state: LockState) -> Response {
    Response::Core(CoreResponse::GetLockState(state))
}

fn unlocked() -> Notification {
    Notification::Core(crate::message::CoreNotification::LockStateChanged(
        LockState::Unlocked,
    ))
}

/// A keymap with layers 0 and 3, of 4 keys each, and a 20-byte name buffer.
fn keymap() -> Keymap {
    let layer = |id, name: &str| Layer {
        id: LayerId(id),
        name: name.to_owned(),
        bindings: vec![
            Binding {
                behavior_id: BehaviorId(1),
                param1: 0x0007_0004,
                param2: 0,
            };
            4
        ],
    };
    Keymap {
        layers: vec![layer(0, "Base"), layer(3, "Raise")],
        available_layers: 2,
        max_layer_name_length: 20,
    }
}

mod requests {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn a_request_gets_its_response() {
        let (client, mut keyboard) = connect();
        let keyboard = tokio::spawn(async move {
            let request = keyboard.next_request().await;
            assert_eq!(request.request, Request::Core(CoreRequest::GetDeviceInfo));
            keyboard.respond(
                request.request_id,
                Response::Core(CoreResponse::GetDeviceInfo(device_info())),
            );
            request.request_id
        });
        assert_eq!(client.get_device_info().await.unwrap(), device_info());
        assert_eq!(keyboard.await.unwrap(), 0, "the first request has ID 0");
        assert_eq!(client.kind(), TransportKind::InMemory);
    }

    #[tokio::test(start_paused = true)]
    async fn request_ids_count_up_and_wrap_around() {
        let (client, mut keyboard) = connect_at(126);
        let keyboard = tokio::spawn(async move {
            let mut ids = Vec::new();
            for _ in 0..3 {
                let request = keyboard.next_request().await;
                ids.push(request.request_id);
                keyboard.respond(request.request_id, lock_state(LockState::Locked));
            }
            ids
        });
        for _ in 0..3 {
            assert_eq!(client.get_lock_state().await.unwrap(), LockState::Locked);
        }
        // At most 127, so the ID takes one byte (spike S1).
        assert_eq!(keyboard.await.unwrap(), [126, 127, 0]);
    }

    /// The Done-when: no request is sent while another is in flight, and the queue is FIFO.
    #[tokio::test(start_paused = true)]
    async fn requests_go_one_at_a_time_in_the_order_they_were_made() {
        let (client, mut keyboard) = connect();
        let keyboard = tokio::spawn(async move {
            let mut order = Vec::new();
            for _ in 0..5 {
                let request = keyboard.next_request().await;
                // Take a while to answer: the client must not send anything meanwhile.
                tokio::time::sleep(Duration::from_millis(500)).await;
                keyboard.assert_quiet();
                let Request::Behaviors(BehaviorsRequest::GetBehaviorDetails { behavior_id }) =
                    request.request
                else {
                    panic!("unexpected request {request:?}");
                };
                order.push(behavior_id.0);
                let details = BehaviorDetails {
                    id: behavior_id,
                    display_name: format!("Behavior {}", behavior_id.0),
                    metadata: vec![],
                };
                keyboard.respond(
                    request.request_id,
                    Response::Behaviors(BehaviorsResponse::GetBehaviorDetails(details)),
                );
            }
            order
        });

        let details = |id| client.get_behavior_details(BehaviorId(id));
        let answers = tokio::join!(
            details(10),
            details(11),
            details(12),
            details(13),
            details(14)
        );
        let answers = [answers.0, answers.1, answers.2, answers.3, answers.4];
        for (result, id) in answers.into_iter().zip(10..) {
            assert_eq!(
                result.unwrap().id,
                BehaviorId(id),
                "each caller gets its own answer"
            );
        }
        assert_eq!(keyboard.await.unwrap(), [10, 11, 12, 13, 14]);
    }

    #[tokio::test(start_paused = true)]
    async fn a_caller_that_stops_waiting_before_its_turn_is_skipped() {
        let (client, mut keyboard) = connect();

        let first = tokio::spawn({
            let client = client.clone();
            async move { client.get_lock_state().await }
        });
        let request = keyboard.next_request().await;

        let abandoned = tokio::spawn({
            let client = client.clone();
            async move { client.get_device_info().await }
        });
        tokio::task::yield_now().await;
        abandoned.abort();
        let third = tokio::spawn({
            let client = client.clone();
            async move { client.check_unsaved_changes().await }
        });
        tokio::task::yield_now().await;

        keyboard.respond(request.request_id, lock_state(LockState::Unlocked));
        assert_eq!(first.await.unwrap().unwrap(), LockState::Unlocked);

        // The abandoned request was never sent: the next one is the third.
        let request = keyboard.next_request().await;
        assert_eq!(
            request.request,
            Request::Keymap(KeymapRequest::CheckUnsavedChanges)
        );
        assert_eq!(request.request_id, 1);
        keyboard.respond(
            request.request_id,
            Response::Keymap(KeymapResponse::CheckUnsavedChanges {
                unsaved_changes: true,
            }),
        );
        assert!(third.await.unwrap().unwrap());
    }

    fn keymap_request(request: KeymapRequest) -> Request {
        Request::Keymap(request)
    }

    fn keymap_response(response: KeymapResponse) -> Response {
        Response::Keymap(response)
    }

    #[tokio::test(start_paused = true)]
    async fn the_typed_methods_that_read_send_their_requests_and_read_the_answers() {
        let (client, mut keyboard) = connect();
        let layouts = PhysicalLayouts {
            active_layout_index: 0,
            layouts: vec![PhysicalLayout {
                name: "Default".to_owned(),
                keys: vec![KeyPhysicalAttrs {
                    width: 100,
                    height: 100,
                    x: 0,
                    y: 0,
                    r: 0,
                    rx: 0,
                    ry: 0,
                }],
            }],
        };

        let (behaviors, ()) = tokio::join!(
            client.list_all_behaviors(),
            keyboard.answer(
                Request::Behaviors(BehaviorsRequest::ListAllBehaviors),
                Response::Behaviors(BehaviorsResponse::ListAllBehaviors(vec![BehaviorId(1)])),
            ),
        );
        assert_eq!(behaviors.unwrap(), [BehaviorId(1)]);

        let (read, ()) = tokio::join!(
            client.get_keymap(),
            keyboard.answer(
                keymap_request(KeymapRequest::GetKeymap),
                keymap_response(KeymapResponse::GetKeymap(keymap())),
            ),
        );
        assert_eq!(read.unwrap(), keymap());

        let (read, ()) = tokio::join!(
            client.get_physical_layouts(),
            keyboard.answer(
                keymap_request(KeymapRequest::GetPhysicalLayouts),
                keymap_response(KeymapResponse::GetPhysicalLayouts(layouts.clone())),
            ),
        );
        assert_eq!(read.unwrap(), layouts);
    }

    #[tokio::test(start_paused = true)]
    async fn the_typed_methods_that_change_the_keymap_send_their_requests() {
        let (client, mut keyboard) = connect();
        let binding = keymap().layers[0].bindings[0];
        let layer = keymap().layers[1].clone();
        let added = AddedLayer {
            index: 2,
            layer: layer.clone(),
        };

        let (result, ()) = tokio::join!(
            client.set_layer_binding(LayerId(3), 3, binding),
            keyboard.answer(
                keymap_request(KeymapRequest::SetLayerBinding {
                    layer_id: LayerId(3),
                    key_position: 3,
                    binding,
                }),
                keymap_response(KeymapResponse::SetLayerBinding(Ok(()))),
            ),
        );
        result.unwrap();

        let (result, ()) = tokio::join!(
            client.set_active_physical_layout(0),
            keyboard.answer(
                keymap_request(KeymapRequest::SetActivePhysicalLayout { index: 0 }),
                keymap_response(KeymapResponse::SetActivePhysicalLayout(Ok(keymap()))),
            ),
        );
        assert_eq!(result.unwrap(), keymap());

        let (result, ()) = tokio::join!(
            client.move_layer(1, 0),
            keyboard.answer(
                keymap_request(KeymapRequest::MoveLayer {
                    start_index: 1,
                    dest_index: 0,
                }),
                keymap_response(KeymapResponse::MoveLayer(Ok(keymap()))),
            ),
        );
        assert_eq!(result.unwrap(), keymap());

        let (result, ()) = tokio::join!(
            client.add_layer(),
            keyboard.answer(
                keymap_request(KeymapRequest::AddLayer),
                keymap_response(KeymapResponse::AddLayer(Ok(added.clone()))),
            ),
        );
        assert_eq!(result.unwrap(), added);

        let (result, ()) = tokio::join!(
            client.remove_layer(2),
            keyboard.answer(
                keymap_request(KeymapRequest::RemoveLayer { layer_index: 2 }),
                keymap_response(KeymapResponse::RemoveLayer(Ok(()))),
            ),
        );
        result.unwrap();

        let (result, ()) = tokio::join!(
            client.restore_layer(LayerId(3), 1),
            keyboard.answer(
                keymap_request(KeymapRequest::RestoreLayer {
                    layer_id: LayerId(3),
                    at_index: 1,
                }),
                keymap_response(KeymapResponse::RestoreLayer(Ok(layer.clone()))),
            ),
        );
        assert_eq!(result.unwrap(), layer);

        let (result, ()) = tokio::join!(
            client.set_layer_props(LayerId(0), "Main".to_owned()),
            keyboard.answer(
                keymap_request(KeymapRequest::SetLayerProps {
                    layer_id: LayerId(0),
                    name: "Main".to_owned(),
                }),
                keymap_response(KeymapResponse::SetLayerProps(Ok(()))),
            ),
        );
        result.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn the_typed_methods_for_settings_and_saving_send_their_requests() {
        let (client, mut keyboard) = connect();

        let (result, ()) = tokio::join!(
            client.reset_settings(),
            keyboard.answer(
                Request::Core(CoreRequest::ResetSettings),
                Response::Core(CoreResponse::ResetSettings { succeeded: true }),
            ),
        );
        assert!(result.unwrap());

        let (result, ()) = tokio::join!(
            client.save_changes(),
            keyboard.answer(
                keymap_request(KeymapRequest::SaveChanges),
                keymap_response(KeymapResponse::SaveChanges(Ok(()))),
            ),
        );
        result.unwrap();

        let (result, ()) = tokio::join!(
            client.discard_changes(),
            keyboard.answer(
                keymap_request(KeymapRequest::DiscardChanges),
                keymap_response(KeymapResponse::DiscardChanges),
            ),
        );
        result.unwrap();
    }
}

mod timeouts {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn a_request_times_out_after_two_seconds_and_the_next_one_goes_out() {
        let (client, mut keyboard) = connect();
        let start = Instant::now();
        let keyboard = tokio::spawn(async move {
            let ignored = keyboard.next_request().await;
            // No answer: the next request only comes once the first has timed out, and
            // the lone `EOF` after it has had time to arrive on its own.
            let request = keyboard.next_request().await;
            assert_eq!(Instant::now() - start, REQUEST_TIMEOUT + RESYNC_PAUSE);
            assert_eq!(request.request_id, ignored.request_id + 1);
            keyboard.respond(request.request_id, lock_state(LockState::Locked));
        });

        let error = client.get_lock_state().await.unwrap_err();
        assert!(matches!(error, RpcError::Timeout(timeout) if timeout == REQUEST_TIMEOUT));
        assert_eq!(Instant::now() - start, REQUEST_TIMEOUT);
        assert_eq!(client.get_lock_state().await.unwrap(), LockState::Locked);
        keyboard.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn the_probe_waits_one_second() {
        let (client, mut keyboard) = connect();
        let start = Instant::now();
        let keyboard = tokio::spawn(async move {
            let probe = keyboard.next_request().await;
            // Keep the keyboard: dropping it would close the connection.
            (probe, keyboard)
        });

        let error = client.probe().await.unwrap_err();
        assert!(matches!(error, RpcError::Timeout(timeout) if timeout == PROBE_TIMEOUT));
        assert_eq!(Instant::now() - start, PROBE_TIMEOUT);
        let (probe, _keyboard) = keyboard.await.unwrap();
        assert_eq!(probe.request, Request::Core(CoreRequest::GetDeviceInfo));
    }

    #[tokio::test(start_paused = true)]
    async fn a_probe_that_gets_an_answer_returns_the_device_info() {
        let (client, mut keyboard) = connect();
        let _keyboard = tokio::spawn(async move {
            let request = keyboard.next_request().await;
            keyboard.respond(
                request.request_id,
                Response::Core(CoreResponse::GetDeviceInfo(device_info())),
            );
            keyboard
        });
        assert_eq!(client.probe().await.unwrap(), device_info());
    }

    #[tokio::test(start_paused = true)]
    async fn a_custom_timeout_applies() {
        let (client, mut keyboard) = connect();
        let start = Instant::now();
        let _keyboard = tokio::spawn(async move {
            keyboard.next_request().await;
            keyboard
        });
        let timeout = Duration::from_millis(250);
        let request = Request::Core(CoreRequest::GetLockState);
        let error = client
            .request_with_timeout(request, timeout)
            .await
            .unwrap_err();
        assert!(matches!(error, RpcError::Timeout(t) if t == timeout));
        assert_eq!(Instant::now() - start, timeout);
    }

    /// A transport whose writes never finish, and which never receives anything.
    struct Stuck;

    impl Transport for Stuck {
        fn send(&mut self, _: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send {
            std::future::pending()
        }

        fn recv(&mut self) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send {
            std::future::pending()
        }

        fn kind(&self) -> TransportKind {
            TransportKind::Usb
        }
    }

    #[tokio::test(start_paused = true)]
    async fn a_send_that_never_finishes_times_out_too() {
        let client = Client::new(Stuck);
        let start = Instant::now();
        assert!(matches!(
            client.get_lock_state().await,
            Err(RpcError::Timeout(_))
        ));
        assert_eq!(Instant::now() - start, REQUEST_TIMEOUT);
        assert_eq!(client.kind(), TransportKind::Usb);
    }

    #[tokio::test(start_paused = true)]
    async fn a_lone_eof_follows_a_timeout_so_the_next_frame_starts_clean() {
        let (client, mut keyboard) = connect();
        let task = tokio::spawn(async move {
            let request = keyboard.next_chunk().await;
            let after = keyboard.next_chunk().await;
            let next = keyboard.next_request().await;
            keyboard.respond(next.request_id, lock_state(LockState::Unlocked));
            (request, after)
        });
        assert!(matches!(
            client.get_lock_state().await,
            Err(RpcError::Timeout(_))
        ));
        assert_eq!(client.get_lock_state().await.unwrap(), LockState::Unlocked);
        let (request, after) = task.await.unwrap();
        assert_eq!(request.first(), Some(&0xAB), "a frame");
        assert_eq!(after, [frame::RESYNC]);
    }

    /// The retry's name starts the name's length further along ZMK's 30-byte ring buffer,
    /// past the end the first one ran over (spike S1).
    #[tokio::test(start_paused = true)]
    async fn a_rename_that_times_out_is_sent_again_further_along_the_buffer() {
        let (client, mut keyboard) = connect();
        let task = tokio::spawn(async move {
            let first = keyboard.next_chunk().await;
            let filler = keyboard.next_chunk().await;
            let retry = keyboard.next_request().await;
            let renamed = Response::Keymap(KeymapResponse::SetLayerProps(Ok(())));
            keyboard.respond(retry.request_id, renamed);
            (first.len(), filler, retry)
        });
        client
            .set_layer_props(LayerId(1), "Numbers".to_owned())
            .await
            .unwrap();
        let (first, filler, retry) = task.await.unwrap();
        assert!(filler.iter().all(|byte| *byte == frame::RESYNC));
        assert_eq!((first + filler.len()) % MAX_FRAME_LEN, "Numbers".len());
        assert_eq!(retry.request_id, 1);
        assert_eq!(
            retry.request,
            Request::Keymap(KeymapRequest::SetLayerProps {
                layer_id: LayerId(1),
                name: "Numbers".to_owned(),
            })
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_rename_is_sent_four_times_at_most() {
        let (client, mut keyboard) = connect();
        let task = tokio::spawn(async move {
            let mut ids = Vec::new();
            for _ in 0..4 {
                ids.push(keyboard.next_request().await.request_id);
            }
            (ids, keyboard)
        });
        let start = Instant::now();
        assert!(matches!(
            client.set_layer_props(LayerId(1), "Nav".to_owned()).await,
            Err(RpcError::Timeout(REQUEST_TIMEOUT))
        ));
        assert_eq!(
            Instant::now() - start,
            REQUEST_TIMEOUT * 4 + RESYNC_PAUSE * 3
        );
        let (ids, mut keyboard) = task.await.unwrap();
        assert_eq!(ids, [0, 1, 2, 3]);
        assert_eq!(keyboard.next_chunk().await, [frame::RESYNC]);
        keyboard.assert_quiet();
    }

    #[tokio::test(start_paused = true)]
    async fn a_late_response_is_dropped_and_the_next_request_gets_its_own() {
        let (client, mut keyboard) = connect();
        let keyboard = tokio::spawn(async move {
            let late = keyboard.next_request().await;
            let next = keyboard.next_request().await;
            // The answer to the request that timed out arrives first.
            keyboard.respond(late.request_id, lock_state(LockState::Locked));
            keyboard.respond(next.request_id, lock_state(LockState::Unlocked));
        });

        assert!(matches!(
            client.get_lock_state().await,
            Err(RpcError::Timeout(_))
        ));
        assert_eq!(client.get_lock_state().await.unwrap(), LockState::Unlocked);
        keyboard.await.unwrap();
    }
}

mod responses {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn responses_with_other_ids_are_dropped() {
        let (client, mut keyboard) = connect();
        let _keyboard = tokio::spawn(async move {
            let request = keyboard.next_request().await;
            keyboard.respond(request.request_id + 7, lock_state(LockState::Locked));
            keyboard.respond(request.request_id, lock_state(LockState::Unlocked));
            keyboard
        });
        assert_eq!(client.get_lock_state().await.unwrap(), LockState::Unlocked);
    }

    #[tokio::test(start_paused = true)]
    async fn frames_can_span_chunks_and_share_them() {
        let (client, mut keyboard) = connect();
        let mut notifications = client.notifications();
        let _keyboard = tokio::spawn(async move {
            let request = keyboard.next_request().await;
            // A notification and the response in one chunk, sent a byte at a time.
            let mut bytes = message(&DeviceMessage::Notification(unlocked()));
            bytes.extend(message(&DeviceMessage::Response {
                request_id: request.request_id,
                response: lock_state(LockState::Unlocked),
            }));
            for byte in bytes {
                keyboard.send(&[byte]);
            }
            keyboard
        });
        assert_eq!(client.get_lock_state().await.unwrap(), LockState::Unlocked);
        assert_eq!(within(notifications.recv()).await.unwrap(), unlocked());
    }

    #[tokio::test(start_paused = true)]
    async fn garbage_and_undecodable_messages_are_skipped() {
        let (client, mut keyboard) = connect();
        let _keyboard = tokio::spawn(async move {
            let request = keyboard.next_request().await;
            // Noise outside a frame, a frame that isn't protobuf, an empty frame (a
            // response with no type), then the answer.
            keyboard.send(&[0x01, 0x02, 0x03]);
            keyboard.send(&frame::encode(&[0xFF, 0xFF, 0xFF]));
            keyboard.send(&frame::encode(&[]));
            keyboard.respond(request.request_id, lock_state(LockState::Locked));
            keyboard
        });
        assert_eq!(client.get_lock_state().await.unwrap(), LockState::Locked);
    }

    #[tokio::test(start_paused = true)]
    async fn keyboard_errors_become_typed_errors() {
        let (client, mut keyboard) = connect();
        let _keyboard = tokio::spawn(async move {
            let replies = [
                Response::Error(ErrorCondition::UnlockRequired),
                Response::Error(ErrorCondition::RpcNotFound),
                Response::Keymap(KeymapResponse::SaveChanges(Err(SaveChangesError::NoSpace))),
                Response::Keymap(KeymapResponse::AddLayer(Err(AddLayerError::NoSpace))),
            ];
            for reply in replies {
                let request = keyboard.next_request().await;
                keyboard.respond(request.request_id, reply);
            }
            keyboard
        });

        assert!(matches!(
            client.get_keymap().await,
            Err(RpcError::Keyboard(ErrorCondition::UnlockRequired))
        ));
        assert!(matches!(
            client.request(Request::Core(CoreRequest::Lock)).await,
            Err(RpcError::Keyboard(ErrorCondition::RpcNotFound))
        ));
        assert!(matches!(
            client.save_changes().await,
            Err(RpcError::SaveChanges(SaveChangesError::NoSpace))
        ));
        assert!(matches!(
            client.add_layer().await,
            Err(RpcError::AddLayer(AddLayerError::NoSpace))
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn a_response_for_another_request_is_unexpected() {
        let (client, mut keyboard) = connect();
        let _keyboard = tokio::spawn(async move {
            for _ in 0..2 {
                let request = keyboard.next_request().await;
                keyboard.respond(request.request_id, lock_state(LockState::Locked));
            }
            keyboard
        });
        assert!(matches!(
            client.get_keymap().await,
            Err(RpcError::UnexpectedResponse)
        ));
        assert!(matches!(
            client.probe().await,
            Err(RpcError::UnexpectedResponse)
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn every_typed_method_refuses_a_response_for_another_request() {
        type Call<'a> = Pin<Box<dyn Future<Output = Result<(), RpcError>> + 'a>>;

        let (client, mut keyboard) = connect();
        let binding = keymap().layers[0].bindings[0];
        let calls: Vec<(&str, Call<'_>)> = vec![
            ("probe", Box::pin(async { client.probe().await.map(drop) })),
            (
                "get_device_info",
                Box::pin(async { client.get_device_info().await.map(drop) }),
            ),
            (
                "get_lock_state",
                Box::pin(async { client.get_lock_state().await.map(drop) }),
            ),
            (
                "reset_settings",
                Box::pin(async { client.reset_settings().await.map(drop) }),
            ),
            (
                "list_all_behaviors",
                Box::pin(async { client.list_all_behaviors().await.map(drop) }),
            ),
            (
                "get_behavior_details",
                Box::pin(async { client.get_behavior_details(BehaviorId(1)).await.map(drop) }),
            ),
            (
                "get_keymap",
                Box::pin(async { client.get_keymap().await.map(drop) }),
            ),
            (
                "set_layer_binding",
                Box::pin(async { client.set_layer_binding(LayerId(0), 0, binding).await }),
            ),
            (
                "check_unsaved_changes",
                Box::pin(async { client.check_unsaved_changes().await.map(drop) }),
            ),
            (
                "save_changes",
                Box::pin(async { client.save_changes().await }),
            ),
            (
                "discard_changes",
                Box::pin(async { client.discard_changes().await }),
            ),
            (
                "get_physical_layouts",
                Box::pin(async { client.get_physical_layouts().await.map(drop) }),
            ),
            (
                "set_active_physical_layout",
                Box::pin(async { client.set_active_physical_layout(0).await.map(drop) }),
            ),
            (
                "move_layer",
                Box::pin(async { client.move_layer(0, 1).await.map(drop) }),
            ),
            (
                "add_layer",
                Box::pin(async { client.add_layer().await.map(drop) }),
            ),
            (
                "remove_layer",
                Box::pin(async { client.remove_layer(0).await }),
            ),
            (
                "restore_layer",
                Box::pin(async { client.restore_layer(LayerId(1), 0).await.map(drop) }),
            ),
            (
                "set_layer_props",
                Box::pin(async { client.set_layer_props(LayerId(0), "Nav".to_owned()).await }),
            ),
        ];
        for (method, call) in calls {
            // The keyboard answers whatever it's asked with "done, nothing to report".
            let answer = async {
                let request = keyboard.next_request().await;
                keyboard.respond(request.request_id, Response::Empty);
            };
            let (result, ()) = tokio::join!(call, answer);
            assert!(
                matches!(result, Err(RpcError::UnexpectedResponse)),
                "{method}: {result:?}"
            );
        }
    }

    #[tokio::test(start_paused = true)]
    async fn the_raw_request_returns_the_response_as_it_is() {
        let (client, mut keyboard) = connect();
        let _keyboard = tokio::spawn(async move {
            let request = keyboard.next_request().await;
            keyboard.respond(request.request_id, Response::Empty);
            keyboard
        });
        let response = client.request(Request::Core(CoreRequest::Lock)).await;
        assert_eq!(response.unwrap(), Response::Empty);
    }
}

mod notifications {
    use super::*;

    fn saved() -> Notification {
        Notification::Keymap(KeymapNotification::UnsavedChangesStatusChanged {
            unsaved_changes: false,
        })
    }

    #[tokio::test(start_paused = true)]
    async fn notifications_reach_every_receiver_between_and_during_requests() {
        let (client, mut keyboard) = connect();
        let mut first = client.notifications();
        let mut second = client.notifications();

        keyboard.notify(unlocked());
        assert_eq!(within(first.recv()).await.unwrap(), unlocked());
        assert_eq!(within(second.recv()).await.unwrap(), unlocked());

        let _keyboard = tokio::spawn(async move {
            let request = keyboard.next_request().await;
            keyboard.notify(saved());
            keyboard.respond(
                request.request_id,
                Response::Keymap(KeymapResponse::SaveChanges(Ok(()))),
            );
            keyboard
        });
        client.save_changes().await.unwrap();
        assert_eq!(within(first.recv()).await.unwrap(), saved());
        assert_eq!(within(second.recv()).await.unwrap(), saved());
    }
}

mod disconnection {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn when_the_transport_fails_every_waiting_request_fails() {
        let (client, mut keyboard) = connect();
        let mut notifications = client.notifications();

        let in_flight = tokio::spawn({
            let client = client.clone();
            async move { client.get_keymap().await }
        });
        keyboard.next_request().await;
        let queued = tokio::spawn({
            let client = client.clone();
            async move { client.get_lock_state().await }
        });
        tokio::task::yield_now().await;

        // The keyboard goes away.
        drop(keyboard);

        assert!(matches!(
            in_flight.await.unwrap(),
            Err(RpcError::Disconnected(_))
        ));
        assert!(matches!(
            queued.await.unwrap(),
            Err(RpcError::Disconnected(_))
        ));
        assert!(matches!(
            *within(client.closed()).await,
            TransportError::Closed
        ));
        assert!(matches!(
            client.get_device_info().await,
            Err(RpcError::Disconnected(_))
        ));
        assert_eq!(
            within(notifications.recv()).await,
            Err(broadcast::error::RecvError::Closed)
        );
        let mut late = client.notifications();
        assert_eq!(
            within(late.recv()).await,
            Err(broadcast::error::RecvError::Closed)
        );
    }

    #[tokio::test(start_paused = true)]
    async fn the_transport_failing_between_requests_closes_the_client() {
        let (client, keyboard) = connect();
        drop(keyboard);
        assert!(matches!(
            *within(client.closed()).await,
            TransportError::Closed
        ));
        let error = client.get_lock_state().await.unwrap_err();
        assert_eq!(error.to_string(), "the connection to the keyboard is lost");
    }

    #[tokio::test(start_paused = true)]
    async fn dropping_every_handle_closes_the_connection() {
        let (client, mut keyboard) = connect();
        let other = client.clone();
        drop(client);
        drop(other);
        // The task ends and drops the transport, so the keyboard sees the client hang up.
        assert_eq!(within(keyboard.from_client.recv()).await, None);
    }
}

mod validation {
    use super::*;

    /// A client that has read [`keymap`].
    async fn loaded() -> (Client, Keyboard) {
        let (client, mut keyboard) = connect();
        let task = tokio::spawn(async move {
            let request = keyboard.next_request().await;
            keyboard.respond(
                request.request_id,
                Response::Keymap(KeymapResponse::GetKeymap(keymap())),
            );
            keyboard
        });
        client.get_keymap().await.unwrap();
        (client, task.await.unwrap())
    }

    fn binding() -> Binding {
        keymap().layers[0].bindings[0]
    }

    #[tokio::test(start_paused = true)]
    async fn invalid_requests_are_refused_without_being_sent() {
        let (client, mut keyboard) = loaded().await;

        let refused = [
            (
                client.set_layer_binding(LayerId(1), 0, binding()).await,
                InvalidRequest::UnknownLayer(LayerId(1)),
            ),
            (
                client.set_layer_binding(LayerId(3), 4, binding()).await,
                InvalidRequest::KeyPositionOutOfRange(4),
            ),
            (
                client.set_layer_binding(LayerId(0), -1, binding()).await,
                InvalidRequest::KeyPositionOutOfRange(-1),
            ),
            (
                // The firmware stores 19 bytes, but a rename has to fit 30 (spike S1).
                client.set_layer_props(LayerId(0), "x".repeat(19)).await,
                InvalidRequest::LayerNameTooLong {
                    length: 19,
                    max: 18,
                },
            ),
            (
                // Ten two-byte characters: the limit is in bytes.
                client.set_layer_props(LayerId(0), "ä".repeat(10)).await,
                InvalidRequest::LayerNameTooLong {
                    length: 20,
                    max: 18,
                },
            ),
            (
                // `Э` is D0 AD in UTF-8, and AD is escaped.
                client.set_layer_props(LayerId(0), "Эхо".to_owned()).await,
                InvalidRequest::LayerNameCharacter('Э'),
            ),
            (
                client.set_layer_props(LayerId(9), "Nav".to_owned()).await,
                InvalidRequest::UnknownLayer(LayerId(9)),
            ),
            (
                client.move_layer(2, 0).await.map(drop),
                InvalidRequest::LayerIndexOutOfRange {
                    index: 2,
                    layers: 2,
                },
            ),
            (
                client.move_layer(0, 2).await.map(drop),
                InvalidRequest::LayerIndexOutOfRange {
                    index: 2,
                    layers: 2,
                },
            ),
            (
                client.remove_layer(2).await,
                InvalidRequest::LayerIndexOutOfRange {
                    index: 2,
                    layers: 2,
                },
            ),
            (
                client.restore_layer(LayerId(5), 3).await.map(drop),
                InvalidRequest::LayerIndexOutOfRange {
                    index: 3,
                    layers: 2,
                },
            ),
        ];
        for (result, expected) in refused {
            match result {
                Err(RpcError::Invalid(invalid)) => assert_eq!(invalid, expected),
                other => panic!("expected {expected:?}, got {other:?}"),
            }
        }
        keyboard.assert_quiet();
    }

    #[tokio::test(start_paused = true)]
    async fn requests_at_the_limits_are_sent() {
        let (client, mut keyboard) = loaded().await;
        let task = tokio::spawn(async move {
            let mut sent = Vec::new();
            for _ in 0..3 {
                let request = keyboard.next_request().await;
                let response = match &request.request {
                    Request::Keymap(KeymapRequest::SetLayerBinding { .. }) => {
                        KeymapResponse::SetLayerBinding(Ok(()))
                    }
                    Request::Keymap(KeymapRequest::SetLayerProps { .. }) => {
                        KeymapResponse::SetLayerProps(Ok(()))
                    }
                    _ => KeymapResponse::RestoreLayer(Ok(keymap().layers[1].clone())),
                };
                keyboard.respond(request.request_id, Response::Keymap(response));
                sent.push(request.request);
            }
            sent
        });
        client
            .set_layer_binding(LayerId(3), 3, binding())
            .await
            .unwrap();
        client
            .set_layer_props(LayerId(0), "x".repeat(18))
            .await
            .unwrap();
        // A restored layer can go after the last one.
        client.restore_layer(LayerId(5), 2).await.unwrap();
        assert_eq!(task.await.unwrap().len(), 3);
    }

    /// The Done-when: no frame over 30 bytes goes out. A rename is the only request that
    /// long, and its name is checked first, but a layer ID with an escaped byte still
    /// makes one too long.
    #[tokio::test(start_paused = true)]
    async fn a_frame_too_long_for_the_keyboard_is_refused_unsent() {
        let (client, mut keyboard) = connect();
        let rename = KeymapRequest::SetLayerProps {
            layer_id: LayerId(0xABAB),
            name: "x".repeat(18),
        };
        let message = ClientMessage {
            request_id: 0,
            request: Request::Keymap(rename.clone()),
        };
        let length = frame::encode(&message.encode()).len();
        assert_eq!(length, 31);
        assert!(matches!(
            client.request(Request::Keymap(rename)).await,
            Err(RpcError::Invalid(InvalidRequest::FrameTooLong {
                length: 31,
                max: MAX_FRAME_LEN
            }))
        ));
        keyboard.assert_quiet();
    }

    #[tokio::test(start_paused = true)]
    async fn a_negative_key_position_is_refused_before_any_keymap_is_known() {
        let (client, mut keyboard) = connect();
        let error = client.set_layer_binding(LayerId(0), -5, binding()).await;
        assert!(matches!(
            error,
            Err(RpcError::Invalid(InvalidRequest::KeyPositionOutOfRange(-5)))
        ));
        keyboard.assert_quiet();
    }

    #[test]
    fn errors_say_what_is_wrong() {
        let cases = [
            (
                InvalidRequest::LayerNameTooLong {
                    length: 21,
                    max: 18,
                },
                "a layer name can be at most 18 bytes long, and this one has 21",
            ),
            (
                InvalidRequest::LayerNameCharacter('«'),
                "ZMK v0.3 can't receive `«` in a layer name",
            ),
            (
                InvalidRequest::FrameTooLong {
                    length: 31,
                    max: 30,
                },
                "the request takes 31 bytes, and the keyboard receives at most 30",
            ),
            (
                InvalidRequest::UnknownLayer(LayerId(4)),
                "the keymap has no layer with ID 4",
            ),
            (
                InvalidRequest::KeyPositionOutOfRange(60),
                "key position 60 isn't on the layer",
            ),
            (
                InvalidRequest::LayerIndexOutOfRange {
                    index: 5,
                    layers: 4,
                },
                "layer index 5 is past the keymap's 4 layers",
            ),
            (
                InvalidRequest::LayoutIndexOutOfRange {
                    index: 1,
                    layouts: 1,
                },
                "physical layout index 1 is past the keyboard's 1 layouts",
            ),
        ];
        for (error, message) in cases {
            assert_eq!(error.to_string(), message);
        }
    }

    mod shape {
        use super::*;

        fn shape() -> KeymapShape {
            let mut shape = KeymapShape::default();
            update(
                &mut shape,
                KeymapRequest::GetKeymap,
                KeymapResponse::GetKeymap(keymap()),
            );
            shape
        }

        fn update(shape: &mut KeymapShape, request: KeymapRequest, response: KeymapResponse) {
            shape.update(&Request::Keymap(request), &Response::Keymap(response));
        }

        fn check(shape: &KeymapShape, request: KeymapRequest) -> Result<(), InvalidRequest> {
            shape.check(&Request::Keymap(request))
        }

        fn remove(layer_index: u32) -> KeymapRequest {
            KeymapRequest::RemoveLayer { layer_index }
        }

        fn layer(id: u32, keys: usize) -> Layer {
            Layer {
                id: LayerId(id),
                name: String::new(),
                bindings: vec![binding(); keys],
            }
        }

        #[test]
        fn nothing_is_checked_against_a_keymap_the_client_hasnt_seen() {
            let shape = KeymapShape::default();
            let rename = |length| KeymapRequest::SetLayerProps {
                layer_id: LayerId(7),
                name: "x".repeat(length),
            };
            assert_eq!(check(&shape, remove(40)), Ok(()));
            assert_eq!(check(&shape, rename(18)), Ok(()));
            // What the keyboard can receive doesn't depend on its keymap.
            assert_eq!(
                check(&shape, rename(19)),
                Err(InvalidRequest::LayerNameTooLong {
                    length: 19,
                    max: 18
                })
            );
            assert_eq!(
                check(&shape, KeymapRequest::SetActivePhysicalLayout { index: 9 }),
                Ok(())
            );
            assert_eq!(shape.check(&Request::Core(CoreRequest::Lock)), Ok(()));
        }

        #[test]
        fn added_restored_and_removed_layers_are_followed() {
            let mut shape = shape();
            let added = AddedLayer {
                index: 2,
                layer: layer(4, 6),
            };
            update(
                &mut shape,
                KeymapRequest::AddLayer,
                KeymapResponse::AddLayer(Ok(added)),
            );
            assert_eq!(check(&shape, remove(2)), Ok(()));
            let set_key = |key_position| KeymapRequest::SetLayerBinding {
                layer_id: LayerId(4),
                key_position,
                binding: binding(),
            };
            assert_eq!(check(&shape, set_key(5)), Ok(()));
            assert_eq!(
                check(&shape, set_key(6)),
                Err(InvalidRequest::KeyPositionOutOfRange(6))
            );

            update(&mut shape, remove(0), KeymapResponse::RemoveLayer(Ok(())));
            // Layers 3 and 4 are left.
            assert_eq!(
                check(&shape, remove(2)),
                Err(InvalidRequest::LayerIndexOutOfRange {
                    index: 2,
                    layers: 2,
                })
            );
            let restore = KeymapRequest::RestoreLayer {
                layer_id: LayerId(0),
                at_index: 0,
            };
            update(
                &mut shape,
                restore,
                KeymapResponse::RestoreLayer(Ok(layer(0, 4))),
            );
            assert_eq!(check(&shape, remove(2)), Ok(()));
        }

        #[test]
        fn failed_changes_change_nothing() {
            let mut shape = shape();
            update(
                &mut shape,
                KeymapRequest::AddLayer,
                KeymapResponse::AddLayer(Err(AddLayerError::NoSpace)),
            );
            update(
                &mut shape,
                remove(0),
                KeymapResponse::RemoveLayer(Err(RemoveLayerError::InvalidIndex)),
            );
            assert_eq!(check(&shape, remove(1)), Ok(()));
            assert!(check(&shape, remove(2)).is_err());
        }

        #[test]
        fn the_keymap_is_forgotten_when_the_client_cant_follow_a_change() {
            let forgets: [(Request, Response); 5] = [
                (
                    Request::Keymap(KeymapRequest::DiscardChanges),
                    Response::Keymap(KeymapResponse::DiscardChanges),
                ),
                (
                    Request::Core(CoreRequest::ResetSettings),
                    Response::Core(CoreResponse::ResetSettings { succeeded: false }),
                ),
                // A response that puts a layer past the end can't be right.
                (
                    Request::Keymap(KeymapRequest::AddLayer),
                    Response::Keymap(KeymapResponse::AddLayer(Ok(AddedLayer {
                        index: 9,
                        layer: layer(5, 4),
                    }))),
                ),
                (
                    Request::Keymap(remove(9)),
                    Response::Keymap(KeymapResponse::RemoveLayer(Ok(()))),
                ),
                // The keymap has two layers, so index 2 is just past the end.
                (
                    Request::Keymap(remove(2)),
                    Response::Keymap(KeymapResponse::RemoveLayer(Ok(()))),
                ),
            ];
            for (request, response) in forgets {
                let mut shape = shape();
                shape.update(&request, &response);
                assert_eq!(check(&shape, remove(40)), Ok(()), "after {response:?}");
            }
        }

        #[test]
        fn a_new_keymap_replaces_the_old_one() {
            let mut shape = shape();
            let mut bigger = keymap();
            bigger.layers.push(layer(6, 4));
            update(
                &mut shape,
                KeymapRequest::MoveLayer {
                    start_index: 0,
                    dest_index: 1,
                },
                KeymapResponse::MoveLayer(Ok(bigger.clone())),
            );
            assert_eq!(check(&shape, remove(2)), Ok(()));
            update(
                &mut shape,
                KeymapRequest::SetActivePhysicalLayout { index: 0 },
                KeymapResponse::SetActivePhysicalLayout(Ok(keymap())),
            );
            assert!(check(&shape, remove(2)).is_err());
        }

        #[test]
        fn physical_layout_indices_are_checked_once_the_layouts_are_known() {
            let mut shape = KeymapShape::default();
            let layouts = PhysicalLayouts {
                active_layout_index: 0,
                layouts: vec![PhysicalLayout {
                    name: "Default".to_owned(),
                    keys: vec![],
                }],
            };
            update(
                &mut shape,
                KeymapRequest::GetPhysicalLayouts,
                KeymapResponse::GetPhysicalLayouts(layouts),
            );
            let set = |index| KeymapRequest::SetActivePhysicalLayout { index };
            assert_eq!(check(&shape, set(0)), Ok(()));
            assert_eq!(
                check(&shape, set(1)),
                Err(InvalidRequest::LayoutIndexOutOfRange {
                    index: 1,
                    layouts: 1,
                })
            );
        }

        #[test]
        fn a_name_is_checked_on_its_own_against_what_zmk_v0_3_receives() {
            assert_eq!(check_layer_name("Navigation"), Ok(()));
            assert_eq!(check_layer_name(&"x".repeat(18)), Ok(()));
            assert_eq!(
                check_layer_name(&"x".repeat(19)),
                Err(InvalidRequest::LayerNameTooLong {
                    length: 19,
                    max: 18
                })
            );
            // Э is 0xD0 0xAD in UTF-8, and 0xAD is a byte the frame escapes.
            assert_eq!(
                check_layer_name("Эхо"),
                Err(InvalidRequest::LayerNameCharacter('Э'))
            );
            assert_eq!(check_layer_name("Навигация"), Ok(()));
        }

        #[test]
        fn a_firmware_that_reports_no_name_limit_gets_names_that_fit_a_frame() {
            let mut shape = KeymapShape::default();
            let mut unlimited = keymap();
            unlimited.max_layer_name_length = 0;
            update(
                &mut shape,
                KeymapRequest::GetKeymap,
                KeymapResponse::GetKeymap(unlimited),
            );
            let rename = |length| KeymapRequest::SetLayerProps {
                layer_id: LayerId(0),
                name: "x".repeat(length),
            };
            assert_eq!(check(&shape, rename(18)), Ok(()));
            assert_eq!(
                check(&shape, rename(19)),
                Err(InvalidRequest::LayerNameTooLong {
                    length: 19,
                    max: 18
                })
            );
        }
    }
}
