# Ratatui TUI 项目分包规划

本规划遵循领域驱动设计（DDD）分层架构，将 ratatui 终端应用按职责拆分为多个 crate/workspace member。

## 架构分层

```
┌─────────────────────────────────────────────┐
│  tui (Presentation)                         │
│  - 页面渲染、组件、事件循环、键位绑定          │
├─────────────────────────────────────────────┤
│  application (Application)                  │
│  - 用例编排、命令处理、状态流转               │
├─────────────────────────────────────────────┤
│  domain (Domain)                            │
│  - 实体、值对象、领域服务、仓储 trait         │
├─────────────────────────────────────────────┤
│  infrastructure (Infrastructure)            │
│  - 仓储实现、配置加载、外部 API 客户端        │
└─────────────────────────────────────────────┘
```

## 依赖规则

```
tui → application → domain
infrastructure → domain
❌ domain → infrastructure
❌ domain → application
❌ application → tui
```

## Crate 职责

### `domain`

核心业务逻辑，**零 UI 框架依赖**，仅允许依赖 `serde`、`chrono`、`thiserror`、`async-trait`。

| 目录/模块 | 职责 |
|-----------|------|
| `entity/` | 领域实体（如 Skill、Agent、Task） |
| `value_object/` | 值对象（如 SkillId、Status） |
| `repository/` | 仓储 trait 定义（接口） |
| `service/` | 领域服务（纯业务逻辑） |
| `event/` | 领域事件 |

### `application`

编排领域逻辑，处理命令与查询。

| 目录/模块 | 职责 |
|-----------|------|
| `command/` | 命令对象（如 `CreateSkillCommand`） |
| `query/` | 查询对象（如 `ListSkillsQuery`） |
| `dto/` | 数据传输对象 |
| `service/` | 应用服务（用例编排） |
| `mapper/` | DTO ↔ 领域模型转换器 |

### `infrastructure`

技术实现细节。

| 目录/模块 | 职责 |
|-----------|------|
| `repository/` | 仓储 trait 的具体实现（文件、SQLite、HTTP） |
| `config/` | 配置加载（环境变量、配置文件） |
| `client/` | 外部 API 客户端（如 agent-platform-server） |
| `persistence/` | 数据持久化底层 |

### `tui`

ratatui 专属代码，负责所有终端 UI 相关逻辑。

| 目录/模块 | 职责 |
|-----------|------|
| `app.rs` | 应用状态机（`App` struct），持有当前页面路由与全局状态 |
| `event/` | 事件系统（键位、定时器、异步事件） |
| `handler/` | 输入处理器（将键位映射为 application command） |
| `page/` | 页面级组件（如 `HomePage`、`SkillListPage`） |
| `component/` | 可复用小组件（如 `SearchBar`、`Table`、`Modal`） |
| `widget/` | 自定义 ratatui `Widget` / `StatefulWidget` 实现 |
| `layout/` | 布局辅助函数与约束定义 |
| `theme/` | 颜色、样式、符号主题配置 |
| `router.rs` | 页面路由/导航逻辑 |
| `runner.rs` | 主事件循环（`tokio::select!` 或 crossterm 事件循环） |

## 事件循环设计

```rust
// tui/runner.rs
loop {
    terminal.draw(|frame| app.render(frame))?;

    let event = event_rx.recv().await?;
    let command = handler::handle(event, &app)?;
    let effect = application_service::execute(command).await?;
    app.apply(effect);
}
```

- `event`：原始输入（键位、鼠标、resize、定时器）
- `command`：领域无关的应用命令
- `effect`：应用层返回的状态变更/副作用描述
- `app.apply`：更新本地 UI 状态

## 模型隔离

三种独立模型，层间通过 mapper 转换：

1. **Domain Model**（`domain::entity::*`）— 纯业务语义
2. **DTO**（`application::dto::*`）— 应用层数据契约
3. **View Model**（`tui::page::*::model`）— UI 渲染所需数据结构

## 命名约定

| 类型 | 约定 | 示例 |
|------|------|------|
| 仓储 trait | `XxxRepository` | `SkillRepository` |
| 仓储实现 | `XxxRepositoryImpl` | `FileSkillRepository` |
| 应用服务 | `XxxUseCase` | `ListSkillsUseCase` |
| TUI 页面 | `XxxPage` | `SkillListPage` |
| TUI 组件 | `XxxComponent` | `SearchBarComponent` |
| 命令 | `XxxCommand` | `CreateSkillCommand` |
| 查询 | `XxxQuery` | `ListSkillsQuery` |

## 外部依赖分配

| Crate | 允许的外部依赖 |
|-------|---------------|
| `domain` | `serde`, `chrono`, `thiserror`, `uuid`, `async-trait` |
| `application` | `domain`, `tokio`, `tracing` |
| `infrastructure` | `domain`, `tokio`, `reqwest`, `toml`, `serde_json`, `sqlx` (可选) |
| `tui` | `ratatui`, `crossterm`, `tokio`, `application`, `infrastructure` |

## 启动入口

```
skillhub-cli (bin crate)
├── main.rs          # 解析 CLI 参数，初始化依赖，启动 tui::runner
├── Cargo.toml       # workspace root，依赖 tui + infrastructure
```

或保持单 crate 多模块结构（`src/domain/`, `src/application/`, `src/infrastructure/`, `src/tui/`），按需要拆分为 workspace。
