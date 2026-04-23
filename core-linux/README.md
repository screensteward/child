# ScreenSteward — Core Linux

Rust daemon for local/LAN Linux mode. Runs as a systemd **system service** (root) so a non-sudoer child user cannot stop it.

Spec of reference: `screensteward-docs/specs/2026-04-23-screensteward-phase1-design.md`.

## Workspace crates

- `ss-core-model` — pure entities, policy evaluator, CRDT counter (no I/O).
- `ss-core-store` — SQLCipher persistence, migrations, DAO, systemd-creds keyring.
- `ss-core-enforce` — cgroups v2 freezer, `/proc` tracker, DBus idle detection.
- `ss-core-ipc` — JSON-RPC 2.0 server over Unix socket, argon2id auth, notifications.
- `ss-core-daemon` — `screensteward-core` binary (systemd unit).
- `ss-core-ctl` — `screensteward-ctl` binary (admin CLI).

## Quick dev

```
just build       # debug build
just test        # tests (excluding #[ignore])
just check       # fmt + clippy strict
just dev         # foreground daemon with /tmp socket
sudo just install  # dogfood install
```

## Root tests

Tests that manipulate real cgroups are marked `#[ignore]`. Run them with `just test-root` under root or inside a `--privileged` container.
