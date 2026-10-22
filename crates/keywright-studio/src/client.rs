//! The RPC client: requests, responses and notifications over a [`Transport`] (tech spec
//! §5.3).
//!
//! [`Client::new`] starts a task that owns the transport. Requests wait in a FIFO queue,
//! and the task sends them **one at a time**: the next one goes out only after the
//! keyboard has answered the previous one, or its timeout has passed. ZMK v0.3 corrupts
//! requests that arrive back to back, and its receive buffer holds 30 bytes.
//!
//! - Each request gets the next `request_id`, a `u32` that wraps around. A response with
//!   any other ID, such as a late answer to a request that timed out, is logged and
//!   dropped.
//! - Every request has a timeout: [`REQUEST_TIMEOUT`], or [`PROBE_TIMEOUT`] for
//!   [`Client::probe`]. ZMK drops requests it can't decode without answering.
//! - Notifications go to every receiver from [`Client::notifications`].
//! - An error the keyboard reports for any request (`meta.simple_error`) becomes
//!   [`RpcError::Keyboard`]. One that only a certain request has, such as running out of
//!   space while saving, becomes that request's own variant, such as
//!   [`RpcError::SaveChanges`].
//! - Requests the keyboard would choke on are refused before they're sent, with
//!   [`RpcError::Invalid`].
//! - When the transport fails, every waiting request fails with
//!   [`RpcError::Disconnected`], and [`Client::closed`] returns. Dropping every handle to
//!   the client closes the connection.
//!
//! The client never sends anything unasked, so it never keeps the keyboard from locking
//! itself again after 10 idle minutes (ADR-0006).

mod validation;

#[cfg(test)]
mod tests;

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::time::{Instant, sleep_until, timeout_at};

pub use self::validation::InvalidRequest;
use self::validation::KeymapShape;
use crate::frame::{self, FrameDecoder};
use crate::message::{
    AddLayerError, AddedLayer, BehaviorDetails, BehaviorId, BehaviorsRequest, BehaviorsResponse,
    Binding, ClientMessage, CoreRequest, CoreResponse, DeviceInfo, DeviceMessage, ErrorCondition,
    Keymap, KeymapRequest, KeymapResponse, Layer, LayerId, LockState, MoveLayerError, Notification,
    PhysicalLayouts, RemoveLayerError, Request, Response, RestoreLayerError, SaveChangesError,
    SetActivePhysicalLayoutError, SetLayerBindingError, SetLayerPropsError,
};
use crate::transport::{Transport, TransportError, TransportKind};

/// How long a request waits for its response, unless it says otherwise.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(2);

/// How long [`Client::probe`] waits. A keyboard that serves Studio on this transport
/// answers quickly. A silent one may be the half that isn't connected to the computer, or
/// have no live editing (tech spec §5.4).
pub const PROBE_TIMEOUT: Duration = Duration::from_secs(1);

/// How many requests can wait in the queue before sending one more waits for room.
const QUEUE_CAPACITY: usize = 32;

/// How many notifications a receiver can fall behind by before it misses some.
const NOTIFICATION_CAPACITY: usize = 16;

/// A handle to a connection to a keyboard.
///
/// Clones share the connection and its queue. The connection closes when the transport
/// fails, or when the last handle is dropped.
#[derive(Debug, Clone)]
pub struct Client {
    queue: mpsc::Sender<Command>,
    notifications: broadcast::WeakSender<Notification>,
    closed_by: Arc<OnceLock<Arc<TransportError>>>,
    kind: TransportKind,
}

/// Why a request failed.
#[derive(Debug, Clone, thiserror::Error)]
pub enum RpcError {
    /// The keyboard didn't answer in time.
    #[error("the keyboard didn't respond within {0:?}")]
    Timeout(Duration),
    /// The keyboard reported an error that any request can get, such as being locked
    /// ([`ErrorCondition::UnlockRequired`]) or not knowing the request
    /// ([`ErrorCondition::RpcNotFound`], so the firmware may be incompatible).
    #[error(transparent)]
    Keyboard(#[from] ErrorCondition),
    /// The keyboard didn't set the binding.
    #[error(transparent)]
    SetLayerBinding(#[from] SetLayerBindingError),
    /// The keyboard didn't save the changes.
    #[error(transparent)]
    SaveChanges(#[from] SaveChangesError),
    /// The keyboard didn't change the physical layout.
    #[error(transparent)]
    SetActivePhysicalLayout(#[from] SetActivePhysicalLayoutError),
    /// The keyboard didn't move the layer.
    #[error(transparent)]
    MoveLayer(#[from] MoveLayerError),
    /// The keyboard didn't add a layer.
    #[error(transparent)]
    AddLayer(#[from] AddLayerError),
    /// The keyboard didn't remove the layer.
    #[error(transparent)]
    RemoveLayer(#[from] RemoveLayerError),
    /// The keyboard didn't restore the layer.
    #[error(transparent)]
    RestoreLayer(#[from] RestoreLayerError),
    /// The keyboard didn't rename the layer.
    #[error(transparent)]
    SetLayerProps(#[from] SetLayerPropsError),
    /// The keyboard answered with a response for another kind of request.
    #[error("the keyboard's response doesn't fit the request")]
    UnexpectedResponse,
    /// The client refused to send the request.
    #[error(transparent)]
    Invalid(#[from] InvalidRequest),
    /// The connection is gone.
    #[error("the connection to the keyboard is lost")]
    Disconnected(#[source] Arc<TransportError>),
}

impl Client {
    /// Starts a client on the transport, in a task on the current Tokio runtime.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    #[must_use]
    pub fn new(transport: impl Transport) -> Self {
        Self::starting_at(transport, 0)
    }

    /// Starts a client whose first request has the ID `first_request_id`. Replay tests use
    /// it to match the IDs in a capture.
    pub(crate) fn starting_at(transport: impl Transport, first_request_id: u32) -> Self {
        let kind = transport.kind();
        let (queue, requests) = mpsc::channel(QUEUE_CAPACITY);
        // Only the task holds the sender itself, so receivers see the channel close when the
        // connection does.
        let (notifications, _) = broadcast::channel(NOTIFICATION_CAPACITY);
        let closed_by = Arc::new(OnceLock::new());
        let client = Self {
            queue,
            notifications: notifications.downgrade(),
            closed_by: Arc::clone(&closed_by),
            kind,
        };
        let connection = Connection {
            transport,
            requests,
            notifications,
            closed_by,
            decoder: FrameDecoder::new(),
            next_request_id: first_request_id,
            shape: KeymapShape::default(),
        };
        tokio::spawn(connection.run());
        client
    }

    /// How the client reaches the keyboard.
    #[must_use]
    pub fn kind(&self) -> TransportKind {
        self.kind
    }

    /// A receiver for the notifications the keyboard sends from now on. It reports the
    /// channel closed when the connection closes.
    #[must_use]
    pub fn notifications(&self) -> broadcast::Receiver<Notification> {
        match self.notifications.upgrade() {
            Some(sender) => sender.subscribe(),
            // The connection is already closed: hand out a receiver that says so.
            None => broadcast::channel(1).1,
        }
    }

    /// Waits until the connection closes, and returns why.
    pub async fn closed(&self) -> Arc<TransportError> {
        self.queue.closed().await;
        self.closed_by()
    }

    /// Sends a request, and waits up to [`REQUEST_TIMEOUT`] for its response.
    ///
    /// # Errors
    ///
    /// Fails if the request is invalid, times out, or the connection is lost, or if the
    /// keyboard reports an error ([`RpcError::Keyboard`]). The typed methods also turn the
    /// errors of a request's own reply into errors; this one returns the reply as it is.
    pub async fn request(&self, request: Request) -> Result<Response, RpcError> {
        self.request_with_timeout(request, REQUEST_TIMEOUT).await
    }

    /// Sends a request, and waits up to `timeout` for its response.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn request_with_timeout(
        &self,
        request: Request,
        timeout: Duration,
    ) -> Result<Response, RpcError> {
        let (reply, response) = oneshot::channel();
        let command = Command {
            request,
            timeout,
            reply,
        };
        if self.queue.send(command).await.is_err() {
            return Err(RpcError::Disconnected(self.closed_by()));
        }
        response
            .await
            .unwrap_or_else(|_| Err(RpcError::Disconnected(self.closed_by())))
    }

    /// Checks that the keyboard serves Studio on this transport: asks for its device info,
    /// and waits only [`PROBE_TIMEOUT`] (tech spec §5.4).
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request). A timeout means the keyboard doesn't serve Studio
    /// here.
    pub async fn probe(&self) -> Result<DeviceInfo, RpcError> {
        let response = self
            .request_with_timeout(Request::Core(CoreRequest::GetDeviceInfo), PROBE_TIMEOUT)
            .await?;
        match response {
            Response::Core(CoreResponse::GetDeviceInfo(info)) => Ok(info),
            _ => Err(RpcError::UnexpectedResponse),
        }
    }

    /// The keyboard's name and serial number. The keyboard answers while locked.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn get_device_info(&self) -> Result<DeviceInfo, RpcError> {
        self.call(
            Request::Core(CoreRequest::GetDeviceInfo),
            |response| match response {
                Response::Core(CoreResponse::GetDeviceInfo(info)) => Some(Ok(info)),
                _ => None,
            },
        )
        .await
    }

    /// Whether the keyboard is locked. The keyboard answers while locked.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn get_lock_state(&self) -> Result<LockState, RpcError> {
        self.call(
            Request::Core(CoreRequest::GetLockState),
            |response| match response {
                Response::Core(CoreResponse::GetLockState(state)) => Some(Ok(state)),
                _ => None,
            },
        )
        .await
    }

    /// Resets the keyboard's saved settings, the keymap included, to the firmware's
    /// defaults (CHG-5). Returns whether every part of the firmware reset its settings.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn reset_settings(&self) -> Result<bool, RpcError> {
        self.call(
            Request::Core(CoreRequest::ResetSettings),
            |response| match response {
                Response::Core(CoreResponse::ResetSettings { succeeded }) => Some(Ok(succeeded)),
                _ => None,
            },
        )
        .await
    }

    /// The IDs of the keyboard's behaviors. The keyboard answers while locked.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn list_all_behaviors(&self) -> Result<Vec<BehaviorId>, RpcError> {
        self.call(
            Request::Behaviors(BehaviorsRequest::ListAllBehaviors),
            |response| match response {
                Response::Behaviors(BehaviorsResponse::ListAllBehaviors(ids)) => Some(Ok(ids)),
                _ => None,
            },
        )
        .await
    }

    /// A behavior's name and the parameters it takes.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn get_behavior_details(
        &self,
        behavior_id: BehaviorId,
    ) -> Result<BehaviorDetails, RpcError> {
        let request = Request::Behaviors(BehaviorsRequest::GetBehaviorDetails { behavior_id });
        self.call(request, |response| match response {
            Response::Behaviors(BehaviorsResponse::GetBehaviorDetails(details)) => {
                Some(Ok(details))
            }
            _ => None,
        })
        .await
    }

    /// Every layer and its bindings.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn get_keymap(&self) -> Result<Keymap, RpcError> {
        self.keymap_call(KeymapRequest::GetKeymap, |response| match response {
            KeymapResponse::GetKeymap(keymap) => Some(Ok(keymap)),
            _ => None,
        })
        .await
    }

    /// Assigns a binding to a key on a layer. The change is unsaved until
    /// [`save_changes`](Self::save_changes).
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request), and [`RpcError::SetLayerBinding`] if the keyboard
    /// refuses the binding.
    pub async fn set_layer_binding(
        &self,
        layer_id: LayerId,
        key_position: i32,
        binding: Binding,
    ) -> Result<(), RpcError> {
        let request = KeymapRequest::SetLayerBinding {
            layer_id,
            key_position,
            binding,
        };
        self.keymap_call(request, |response| match response {
            KeymapResponse::SetLayerBinding(result) => Some(result.map_err(Into::into)),
            _ => None,
        })
        .await
    }

    /// Whether the keymap or the physical layout has unsaved changes.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn check_unsaved_changes(&self) -> Result<bool, RpcError> {
        self.keymap_call(
            KeymapRequest::CheckUnsavedChanges,
            |response| match response {
                KeymapResponse::CheckUnsavedChanges { unsaved_changes } => {
                    Some(Ok(unsaved_changes))
                }
                _ => None,
            },
        )
        .await
    }

    /// Saves the changes to the keyboard's flash.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request), and [`RpcError::SaveChanges`] if the keyboard
    /// doesn't save them.
    pub async fn save_changes(&self) -> Result<(), RpcError> {
        self.keymap_call(KeymapRequest::SaveChanges, |response| match response {
            KeymapResponse::SaveChanges(result) => Some(result.map_err(Into::into)),
            _ => None,
        })
        .await
    }

    /// Reverts the changes made since the last save.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn discard_changes(&self) -> Result<(), RpcError> {
        self.keymap_call(KeymapRequest::DiscardChanges, |response| match response {
            KeymapResponse::DiscardChanges => Some(Ok(())),
            _ => None,
        })
        .await
    }

    /// The physical layouts the keyboard supports, and which one is active.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request).
    pub async fn get_physical_layouts(&self) -> Result<PhysicalLayouts, RpcError> {
        self.keymap_call(
            KeymapRequest::GetPhysicalLayouts,
            |response| match response {
                KeymapResponse::GetPhysicalLayouts(layouts) => Some(Ok(layouts)),
                _ => None,
            },
        )
        .await
    }

    /// Makes a physical layout active, and returns the keymap for it.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request), and [`RpcError::SetActivePhysicalLayout`] if the
    /// keyboard doesn't change it.
    pub async fn set_active_physical_layout(&self, index: u32) -> Result<Keymap, RpcError> {
        let request = KeymapRequest::SetActivePhysicalLayout { index };
        self.keymap_call(request, |response| match response {
            KeymapResponse::SetActivePhysicalLayout(result) => Some(result.map_err(Into::into)),
            _ => None,
        })
        .await
    }

    /// Moves a layer to another index, and returns the keymap with its layers in their new
    /// order.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request), and [`RpcError::MoveLayer`] if the keyboard
    /// doesn't move it.
    pub async fn move_layer(&self, start_index: u32, dest_index: u32) -> Result<Keymap, RpcError> {
        let request = KeymapRequest::MoveLayer {
            start_index,
            dest_index,
        };
        self.keymap_call(request, |response| match response {
            KeymapResponse::MoveLayer(result) => Some(result.map_err(Into::into)),
            _ => None,
        })
        .await
    }

    /// Adds a layer at the end, in a free slot.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request), and [`RpcError::AddLayer`] if the keyboard
    /// doesn't add one, for example because every slot is in use.
    pub async fn add_layer(&self) -> Result<AddedLayer, RpcError> {
        self.keymap_call(KeymapRequest::AddLayer, |response| match response {
            KeymapResponse::AddLayer(result) => Some(result.map_err(Into::into)),
            _ => None,
        })
        .await
    }

    /// Removes a layer. Its slot keeps the bindings, so
    /// [`restore_layer`](Self::restore_layer) can bring it back.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request), and [`RpcError::RemoveLayer`] if the keyboard
    /// doesn't remove it.
    pub async fn remove_layer(&self, layer_index: u32) -> Result<(), RpcError> {
        let request = KeymapRequest::RemoveLayer { layer_index };
        self.keymap_call(request, |response| match response {
            KeymapResponse::RemoveLayer(result) => Some(result.map_err(Into::into)),
            _ => None,
        })
        .await
    }

    /// Brings back a removed layer at an index, and returns it.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request), and [`RpcError::RestoreLayer`] if the keyboard
    /// doesn't restore it.
    pub async fn restore_layer(&self, layer_id: LayerId, at_index: u32) -> Result<Layer, RpcError> {
        let request = KeymapRequest::RestoreLayer { layer_id, at_index };
        self.keymap_call(request, |response| match response {
            KeymapResponse::RestoreLayer(result) => Some(result.map_err(Into::into)),
            _ => None,
        })
        .await
    }

    /// Renames a layer.
    ///
    /// # Errors
    ///
    /// As for [`request`](Self::request), and [`RpcError::SetLayerProps`] if the keyboard
    /// doesn't rename it. A name too long for the firmware is refused before sending
    /// ([`InvalidRequest::LayerNameTooLong`]), once the client has seen the keymap.
    pub async fn set_layer_props(&self, layer_id: LayerId, name: String) -> Result<(), RpcError> {
        let request = KeymapRequest::SetLayerProps { layer_id, name };
        self.keymap_call(request, |response| match response {
            KeymapResponse::SetLayerProps(result) => Some(result.map_err(Into::into)),
            _ => None,
        })
        .await
    }

    /// Sends a request, and picks the answer out of its response, or returns `None` if the
    /// response is for another kind of request.
    async fn call<T>(
        &self,
        request: Request,
        answer: impl FnOnce(Response) -> Option<Result<T, RpcError>>,
    ) -> Result<T, RpcError> {
        answer(self.request(request).await?).unwrap_or(Err(RpcError::UnexpectedResponse))
    }

    /// [`call`](Self::call), for the `keymap` subsystem.
    async fn keymap_call<T>(
        &self,
        request: KeymapRequest,
        answer: impl FnOnce(KeymapResponse) -> Option<Result<T, RpcError>>,
    ) -> Result<T, RpcError> {
        self.call(Request::Keymap(request), |response| match response {
            Response::Keymap(response) => answer(response),
            _ => None,
        })
        .await
    }

    fn closed_by(&self) -> Arc<TransportError> {
        self.closed_by
            .get()
            .map_or_else(|| Arc::new(TransportError::Closed), Arc::clone)
    }
}

/// A request waiting in the queue, with the way back to its caller.
struct Command {
    request: Request,
    timeout: Duration,
    reply: oneshot::Sender<Result<Response, RpcError>>,
}

/// The task that owns the transport.
struct Connection<T> {
    transport: T,
    requests: mpsc::Receiver<Command>,
    notifications: broadcast::Sender<Notification>,
    closed_by: Arc<OnceLock<Arc<TransportError>>>,
    decoder: FrameDecoder,
    next_request_id: u32,
    shape: KeymapShape,
}

impl<T: Transport> Connection<T> {
    /// Serves requests until the transport fails, or every client handle is dropped.
    async fn run(mut self) {
        let Err(error) = self.serve().await else {
            return;
        };
        tracing::info!(%error, "the connection to the keyboard closed");
        // Set before the queue closes, so `Client::closed` sees it.
        self.closed_by.get_or_init(|| Arc::clone(&error));
        self.requests.close();
        while let Ok(command) = self.requests.try_recv() {
            // A caller that stopped waiting needs no answer.
            let _ = command
                .reply
                .send(Err(RpcError::Disconnected(Arc::clone(&error))));
        }
    }

    async fn serve(&mut self) -> Result<(), Arc<TransportError>> {
        loop {
            // Between requests, the keyboard can still send notifications.
            let command = tokio::select! {
                command = self.requests.recv() => match command {
                    Some(command) => command,
                    None => return Ok(()),
                },
                chunk = self.transport.recv() => {
                    let chunk = chunk.map_err(Arc::new)?;
                    self.read(&chunk, None);
                    continue;
                }
            };
            self.handle(command).await?;
        }
    }

    /// Sends one request, and answers its caller.
    async fn handle(&mut self, command: Command) -> Result<(), Arc<TransportError>> {
        let Command {
            request,
            timeout,
            reply,
        } = command;
        if reply.is_closed() {
            // The caller stopped waiting before its turn: don't send the request at all.
            return Ok(());
        }
        if let Err(invalid) = self.shape.check(&request) {
            let _ = reply.send(Err(invalid.into()));
            return Ok(());
        }

        let request_id = self.next_request_id;
        self.next_request_id = request_id.wrapping_add(1);
        let message = ClientMessage {
            request_id,
            request,
        };
        let frame = frame::encode(&message.encode());
        // The caller may stop waiting now, but the request is on its way, so the task still
        // waits for its response before sending the next one.
        let answer = match self
            .exchange(request_id, &frame, Instant::now() + timeout)
            .await
        {
            Ok(Some(response)) => {
                self.shape.update(&message.request, &response);
                match response {
                    Response::Error(condition) => Err(RpcError::Keyboard(condition)),
                    response => Ok(response),
                }
            }
            Ok(None) => Err(RpcError::Timeout(timeout)),
            Err(error) => {
                let error = Arc::new(error);
                let _ = reply.send(Err(RpcError::Disconnected(Arc::clone(&error))));
                return Err(error);
            }
        };
        let _ = reply.send(answer);
        Ok(())
    }

    /// Sends a request's frame, and waits until `deadline` for the response with its ID.
    /// Returns `None` if the deadline passes first.
    async fn exchange(
        &mut self,
        request_id: u32,
        frame: &[u8],
        deadline: Instant,
    ) -> Result<Option<Response>, TransportError> {
        match timeout_at(deadline, self.transport.send(frame)).await {
            Ok(sent) => sent?,
            Err(_) => return Ok(None),
        }
        loop {
            let chunk = tokio::select! {
                () = sleep_until(deadline) => return Ok(None),
                chunk = self.transport.recv() => chunk?,
            };
            if let Some(response) = self.read(&chunk, Some(request_id)) {
                return Ok(Some(response));
            }
        }
    }

    /// Reads a chunk from the keyboard. Notifications go to the receivers, and the response
    /// to `awaited`, if the chunk completes it, is returned. Everything else is logged and
    /// dropped.
    fn read(&mut self, chunk: &[u8], awaited: Option<u32>) -> Option<Response> {
        let mut found = None;
        for frame in self.decoder.decode(chunk) {
            let payload = match frame {
                Ok(payload) => payload,
                Err(error) => {
                    tracing::warn!(%error, "skipped bytes from the keyboard");
                    continue;
                }
            };
            match DeviceMessage::decode(&payload) {
                Ok(DeviceMessage::Notification(notification)) => {
                    // Sending fails only when nobody listens, which is fine.
                    let _ = self.notifications.send(notification);
                }
                Ok(DeviceMessage::Response {
                    request_id,
                    response,
                }) if found.is_none() && Some(request_id) == awaited => found = Some(response),
                Ok(DeviceMessage::Response { request_id, .. }) => {
                    tracing::warn!(request_id, "dropped a response to no request in flight");
                }
                Err(error) => {
                    tracing::warn!(%error, "dropped a message from the keyboard that couldn't be decoded");
                }
            }
        }
        found
    }
}
