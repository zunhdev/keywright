# Security policy

## Supported versions

Keywright is pre-release. Security fixes go to the `main` branch and, once releases exist, to the latest release.

## Reporting a vulnerability

Please **do not** open a public issue for a security problem.

Report it privately through GitHub instead: **[Report a vulnerability](https://github.com/zunhdev/keywright/security/advisories/new)**. You can also find this under _Security → Advisories → Report a vulnerability_.

Please include:

- what you found and where (file, command, or screen);
- steps to reproduce it;
- the impact you expect.

You can expect an acknowledgement within 7 days. We'll work on a fix with you, agree on a disclosure date, and credit you in the published advisory unless you'd rather not be named.

## Scope

Keywright talks to keyboards over USB and Bluetooth, and reads and writes keymap backup files. These areas are of particular interest:

- malformed data from a device or a file causing crashes, hangs, or unintended writes to the keyboard;
- anything that changes a keyboard's keymap or firmware without an explicit user action;
- problems in release artifacts, the build, or CI.

Report vulnerabilities in ZMK firmware itself to the [ZMK project](https://github.com/zmkfirmware/zmk).
