-- ScreenSteward — initial schema (Phase 1, D39 app identity by content_hash).

CREATE TABLE family (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL
) STRICT;

CREATE TABLE parent (
    id TEXT PRIMARY KEY NOT NULL,
    family_id TEXT NOT NULL REFERENCES family(id) ON DELETE CASCADE,
    display_name TEXT NOT NULL,
    auth_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL
) STRICT;
CREATE INDEX idx_parent_family ON parent(family_id);

CREATE TABLE child (
    id TEXT PRIMARY KEY NOT NULL,
    family_id TEXT NOT NULL REFERENCES family(id) ON DELETE CASCADE,
    display_name TEXT NOT NULL,
    birth_year INTEGER,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL
) STRICT;
CREATE INDEX idx_child_family ON child(family_id);

CREATE TABLE child_device (
    id TEXT PRIMARY KEY NOT NULL,
    child_id TEXT NOT NULL REFERENCES child(id) ON DELETE CASCADE,
    hostname TEXT NOT NULL,
    platform TEXT NOT NULL,
    noise_pubkey BLOB,
    last_seen_at TEXT NOT NULL
) STRICT;
CREATE INDEX idx_child_device_child ON child_device(child_id);

CREATE TABLE policy (
    id TEXT PRIMARY KEY NOT NULL,
    child_id TEXT NOT NULL REFERENCES child(id) ON DELETE CASCADE,
    scope_json TEXT NOT NULL,
    rules_json TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    active_from TEXT,
    active_until TEXT,
    created_at TEXT NOT NULL,
    modified_at TEXT NOT NULL
) STRICT;
CREATE INDEX idx_policy_child ON policy(child_id);

CREATE TABLE usage_counter (
    child_id TEXT NOT NULL REFERENCES child(id) ON DELETE CASCADE,
    device_id TEXT NOT NULL REFERENCES child_device(id) ON DELETE CASCADE,
    date TEXT NOT NULL,
    minutes_used INTEGER NOT NULL,
    PRIMARY KEY (child_id, device_id, date)
) STRICT;

CREATE TABLE usage_event (
    id TEXT PRIMARY KEY NOT NULL,
    child_id TEXT NOT NULL REFERENCES child(id) ON DELETE CASCADE,
    device_id TEXT NOT NULL REFERENCES child_device(id) ON DELETE CASCADE,
    content_hash TEXT NOT NULL,
    basename TEXT NOT NULL,
    path TEXT NOT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    category TEXT
) STRICT;
CREATE INDEX idx_usage_event_child_date ON usage_event(child_id, started_at);

CREATE TABLE policy_exception (
    id TEXT PRIMARY KEY NOT NULL,
    child_id TEXT NOT NULL REFERENCES child(id) ON DELETE CASCADE,
    granted_by_parent_id TEXT REFERENCES parent(id) ON DELETE SET NULL,
    status TEXT NOT NULL CHECK (status IN ('pending','approved','denied','expired')),
    reason TEXT,
    duration_minutes INTEGER,
    granted_at TEXT,
    expires_at TEXT,
    created_at TEXT NOT NULL
) STRICT;
CREATE INDEX idx_policy_exception_child_status ON policy_exception(child_id, status);

CREATE TABLE app_signature (
    content_hash TEXT PRIMARY KEY NOT NULL,
    display_name TEXT,
    known_basenames TEXT NOT NULL,
    known_paths TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL
) STRICT;

CREATE TABLE idempotency (
    request_id TEXT PRIMARY KEY NOT NULL,
    response_json TEXT NOT NULL,
    seen_at TEXT NOT NULL
) STRICT;
CREATE INDEX idx_idempotency_seen ON idempotency(seen_at);
