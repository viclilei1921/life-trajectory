# 生命轨迹 —— Server 模块设计

## 一、与整体架构的对齐

### 1.1 设计目标

| 目标 | 说明 |
| --- | --- |
| **Local-First 可上云** | 桌面端用 Tauri command；同一套 handlers / services / repositories 未来可挂 HTTP（axum） |
| **分层清晰** | 校验、编排、持久化职责分离，禁止下层反向依赖上层 |
| **错误体系统一** | 对外业务码只来自 `server/error.rs`（`AppError` / `BizCode`） |
| **配置集中** | 路径等配置只来自 `server/config.rs` |

### 1.2 在系统中的位置

```
┌─────────────────────────────────────────┐
│  Svelte UI                              │
│  bridge: vault.ts / entry.ts            │
└───────────────┬─────────────────────────┘
                │ Tauri invoke（未来可换 HTTP）
┌───────────────▼───────────────────────────┐
│  server/commands/     路由装配/云为route   │
│  server/handlers/     校验 + 响应封装      │
│  server/services/     业务流程编排         │
│  server/repositories/ 数据库访问逻辑       │
│  server/models|dto    领域实体 / 传输结构  │
├──────────────────────────────────────────┤
│  crypto/              纯加密原语(无 SQL)  │
│  SQLite + migrations                     │
└──────────────────────────────────────────┘
```

---

## 二、目录与分层边界（强约束）

### 2.1 目录结构

```
src-tauri/src/
├── lib.rs                 # 仅装配：mod crypto / server，setup + invoke_handler
├── crypto/                # 纯加密原语
└── server/
    ├── mod.rs             # bootstrap(app)
    ├── config.rs          # AppConfig（data_dir / db_path）
    ├── db.rs              # init_pool + sqlx migrate
    ├── state.rs           # AppState { db, vault }
    ├── error.rs           # AppError / BizCode
    ├── transport.rs       # 错误编码等传输适配（无业务码定义）
    ├── commands/          # #[tauri::command] 注册与拼装
    ├── handlers/          # 参数提取、校验、调 service
    ├── services/          # 用例编排
    ├── repositories/      # SQLx
    ├── models/            # 领域实体与枚举（单一事实源）
    ├── dto/               # 输入输出结构（camelCase JSON）
    └── middleware/        # 中间层（如 require_unlocked）
```

> 桌面端用 `commands/` 代替 Web 的 `routes/`；上云时新增 HTTP router，复用同一套 handlers。

### 2.2 依赖方向

```
commands ──► handlers ──► services ──► repositories ──► models
                │              │
                ├─► dto        └─► crypto
                ├─► error
                └─► middleware

dto ──► models
dto ──► transport（按需）
dto ──► error（按需）

state ──► db / crypto(VaultSession)
```

**允许**：`handlers → services → repositories → models`；`services → crypto`；`dto → models`。

**禁止**：

- `repositories` / `models` / `crypto` 依赖 `handlers` / `commands`
- `crypto` 依赖 `sqlx` / `tauri`
- `transport` 定义业务码或业务规则
- 新增「服务层统一校验模块」破坏就近内聚

### 2.3 校验职责

| 层 | 职责 |
| --- | --- |
| **handlers** | **唯一校验入口**：基础参数（非空 id/password）+ 可前置的业务规则 |
| **services** | 编排流程；返回业务失败（`VAULT_LOCKED` 等），**不**做接口入参格式校验 |
| **repositories** | 纯 SQL；返回 `models` 或映射为 `AppError` |
| **middleware** | 中间层复用（如 `require_unlocked`），由 handlers/services 显式调用 |

---

## 三、启动与状态

### 3.1 bootstrap 流程

[`lib.rs`](../src-tauri/src/lib.rs) → `server::bootstrap(app)`：

```
app_data_dir
  → AppConfig::from_app_data_dir
  → db::init_pool（创建目录 + SQLite + migrations）
  → VaultService::bootstrap_session
       ├─ device 且无 wrapped_dek → 生成 DEK、keyring 包装、写库、会话已解锁
       ├─ device 且已有 wrapped_dek → 自动 unwrap → 已解锁
       └─ password / none → 仅建空会话（password 需前端 unlock）
  → manage(AppState { db, vault: Mutex<VaultSession> })
```

### 3.2 AppState

```rust
pub struct AppState {
  pub db: SqlitePool,
  pub vault: tokio::sync::Mutex<VaultSession>, // DEK 仅在此内存中
}
```

- 使用 `tokio::sync::Mutex`，避免 async command 持锁跨 await 时 `Send` 失败。
- **DEK 绝不经 IPC 下发**；前端只拿 `VaultStatusDto`（mode / isUnlocked / cryptoVersion）。

### 3.3 配置

`AppConfig` 目前仅含数据目录与 `life.db` 路径。后续云端相关项（同步端点、设备 ID 等）也只在 `config.rs` 扩展，禁止在业务文件硬编码路径。

---

## 四、各层职责与现状映射

### 4.1 commands（路由装配）

薄封装：取 `State<AppState>` → 调 handler → `transport::to_command_result`。

| 文件 | 命令 |
| --- | --- |
| `commands/vault.rs` | `vault_get_status` / `vault_unlock` / `vault_lock` / `vault_setup_password` / `vault_change_password` |
| `commands/entry.rs` | `create_entry` / `get_entry` |
| `commands/debug.rs` | `encrypt_simple_text` / `decrypt_simple_text`（v1 演示，不写 DB） |

### 4.2 handlers

| 文件 | 校验示例 | 调用 |
| --- | --- | --- |
| `handlers/vault.rs` | password 非空 | `VaultService::*` |
| `handlers/entry.rs` | id 非空 | `EntryService::*` |
| `handlers/debug.rs` | password 非空 | `crypto::simple` |

### 4.3 services

| 服务 | 职责 |
| --- | --- |
| **VaultService** | bootstrap、status、unlock/lock、setup_password、change_password；编排 `VaultSession` + `vault` repository |
| **EntryService** | create/get：按 `VaultMode` 决定明文或 `encrypt_entry` / `decrypt_entry`，再写/读 repository |

### 4.4 repositories

| 文件 | 表 | 操作 |
| --- | --- | --- |
| `repositories/vault.rs` | `vault_config` | `load` / `save` |
| `repositories/entry.rs` | `entries` | `insert` / `find_by_id` |

只返回 `models`，不做加解密。

### 4.5 models / dto

| models（领域） | dto（对外） |
| --- | --- |
| `VaultMode`、`VaultConfig`、`WrappedDekUpdate` | `VaultStatusDto` |
| `Entry`、`NewEntry` | `EntryDto` |

DTO 使用 `serde rename_all = "camelCase"`，与前端 [`src/lib/bridge/vault.ts`](../src/lib/bridge/vault.ts) 对齐。

### 4.6 transport / middleware / error

- **transport**：`encode_error` / `to_command_result`；上云时可扩展 JSON body / 状态码映射，仍不定义 `BizCode`。
- **middleware**：`require_unlocked(session, mode)`。
- **error**：见 §五。

---

## 五、错误与业务码

### 5.1 类型

```rust
pub enum BizCode { /* 稳定字符串码 */ }
pub enum AppError {
  Biz(BizCode),
  Message(String), // INVALID_ARGUMENT: … / INTERNAL_ERROR: …
}
```

Tauri 侧当前将 `AppError` 转为 `String`（业务码或带前缀消息）。上云时可改为结构化 JSON，码表不变。

### 5.2 码表

| BizCode | 字符串 | 含义 |
| --- | --- | --- |
| `VaultLocked` | `VAULT_LOCKED` | 需解锁 |
| `VaultAlreadyUnlocked` | `VAULT_ALREADY_UNLOCKED` | 重复解锁 |
| `VaultInvalidPassword` | `VAULT_INVALID_PASSWORD` | 密码错误 |
| `VaultModeMismatch` | `VAULT_MODE_MISMATCH` | 操作与当前 mode 不符 |
| `MigrationInProgress` | `MIGRATION_IN_PROGRESS` | 批量迁移中 |
| `EntryNotFound` | `ENTRY_NOT_FOUND` | 记录不存在 |
| `CryptoInvalidBlob` | `CRYPTO_INVALID_BLOB` | 密文损坏/版本不支持 |
| `InvalidArgument` | `INVALID_ARGUMENT` | 入参非法（handlers） |
| `Internal` | `INTERNAL_ERROR` | 内部错误 |

`CryptoError` → `AppError` 的映射集中在 `error.rs`（如 `UnwrapFailed` → `VAULT_INVALID_PASSWORD`）。

---

## 六、请求链路（示例）

### 6.1 解锁（password）

```
UI vaultUnlock(password)
  → commands::vault_unlock
  → handlers：password 非空
  → VaultService::unlock
       → vault_repo::load
       → 校验 mode == password、未解锁
       → session.unlock_with_password(salt, wrapped)
  → VaultStatusDto
```

### 6.2 新建加密记录（device / password）

```
UI createEntry({ id, content, … })
  → commands::create_entry
  → handlers：id 非空
  → EntryService::create
       → vault_repo::load（取 mode）
       → session.require_dek（未解锁 → VAULT_LOCKED）
       → crypto::entry::encrypt_entry
       → entry_repo::insert（content=''，content_encrypted=blob）
  → EntryDto（明文 content 仅返回给 UI，不落库）
```

### 6.3 读取记录

```
get_entry
  → entry_repo::find_by_id
  → 若 is_encrypted：require DEK → decrypt_entry
  → EntryDto
```

---

## 七、上云预备（transport 演进）

当前传输 = Tauri IPC。目标是 **handlers 以下不感知传输**：

| 阶段 | 传输层 | 复用 |
| --- | --- | --- |
| 现在 | `commands/` + `transport::to_command_result` | handlers → services → repos |
| 上云 | `routes/`（axum）+ 同一 `transport` 扩展（JSON / Protobuf） | **同一套** handlers / services / repositories / models |

约束：

1. 不要在 `commands` 里写业务 SQL 或校验（保持薄）。
2. 不要让 `services` 返回 Tauri 专用类型；只返回 `dto` / `AppError`。
3. 鉴权、观测放 `middleware/`，桌面端可先空实现或仅 vault 锁检查。

```
桌面:  UI ──IPC──► commands ──► handlers ──► …
云端:  UI ──HTTP─► routes   ──► handlers ──► …（同一路径）
```

---

## 八、数据库与 migration

- 连接与 migrate：仅 [`server/db.rs`](../src-tauri/src/server/db.rs)。
- SQL 文件：[`src-tauri/migrations/`](../src-tauri/migrations/)（`0001_initial` entries）。

表职责简述：

| 表 | 说明 |
| --- | --- |
| `vault_config` | 单行；mode / kdf_salt / wrapped_dek / wrap_nonce / crypto_version |
| `entries` | `content` 明文模式用；加密模式用 `content_encrypted` + `is_encrypted` |

---

## 九、前端契约

命令名与字段保持稳定（见 bridge）：

| 命令 | 主要入参 | 返回 |
| --- | --- | --- |
| `vault_get_status` | — | `{ mode, isUnlocked, cryptoVersion }` |
| `vault_unlock` | `password` | 同上 |
| `vault_lock` | — | 同上 |
| `vault_setup_password` | `password` | 同上 |
| `vault_change_password` | `old`, `new` | `()` |
| `create_entry` | `id`, `content`, `happened_at`, `title?` | `EntryDto` |
| `get_entry` | `id` | `EntryDto` |

错误：字符串业务码（或 `INVALID_ARGUMENT: …`）。前端按码提示，不解析内部细节。

---

## 十、扩展清单（后续业务接入本分层）

新增用例时按此落点：

1. **models**：领域结构  
2. **repositories**：SQL  
3. **services**：编排（含是否调 crypto）  
4. **dto**：对外形状  
5. **handlers**：校验 + 调 service  
6. **commands**：注册 IPC（或未来 routes）  
7. 需要中间层时加 **middleware**，更新 **BizCode**（仅 `error.rs`）

优先待接（对照整体设计 API）：`update_entry`、`list_entries`、`search_entries`、人物/标签/媒体、导出归档等——一律走上述路径。
