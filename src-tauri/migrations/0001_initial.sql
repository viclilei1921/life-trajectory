-- 初始 schema：entries + vault 配置

CREATE TABLE entries (
  id                 TEXT PRIMARY KEY,
  title              TEXT,
  content            TEXT NOT NULL DEFAULT '',
  content_encrypted  BLOB,
  is_encrypted       INTEGER NOT NULL DEFAULT 0,
  entry_type         TEXT NOT NULL DEFAULT 'note',
  happened_at        INTEGER NOT NULL,
  happened_precision TEXT NOT NULL DEFAULT 'exact',
  timezone           TEXT NOT NULL DEFAULT 'Asia/Shanghai',
  location_id        TEXT,
  mood               INTEGER,
  energy             INTEGER,
  weather            TEXT,
  metrics            TEXT,
  is_favorite        INTEGER NOT NULL DEFAULT 0,
  created_at         INTEGER NOT NULL,
  updated_at         INTEGER NOT NULL,
  deleted_at         INTEGER
);

CREATE INDEX idx_entries_happened_at ON entries(happened_at);
CREATE INDEX idx_entries_deleted ON entries(deleted_at);

CREATE TABLE vault_config (
  id              INTEGER PRIMARY KEY CHECK (id = 1),
  mode            TEXT NOT NULL CHECK (mode IN ('none', 'password')),
  kdf_salt        BLOB,
  wrapped_dek     BLOB,
  wrap_nonce      BLOB,
  crypto_version  INTEGER NOT NULL DEFAULT 1,
  created_at      INTEGER NOT NULL,
  updated_at      INTEGER NOT NULL
);

INSERT INTO vault_config (id, mode, crypto_version, created_at, updated_at)
VALUES (1, 'none', 1, unixepoch(), unixepoch());
