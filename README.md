# ScreenSteward Child

The child-side app — enforces screen time policies and shows remaining time, usage, and active rules to the child.

Architecture: Flutter UI (shared across OSes) + native enforcement core per OS (Rust on Linux, Swift on iOS/macOS, Kotlin on Android, C# on Windows). See `screensteward-docs/specs/2026-04-23-screensteward-fondations-design.md`, section 4.1 and section 2.

**Status:** Phase 0 scaffolding. Implementation starts in Phase 1 with Linux only.

## License

GNU General Public License v3.0 (GPL-3.0).
