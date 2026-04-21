# SkillHub CLI 登录功能使用指南

## 问题修复

### 原始问题
运行 `skillhub login` 时，程序无法输入用户名和密码，陷入死循环。

### 修复内容

1. **添加了字符输入处理**
   - 在 `handler.rs` 中添加了 `InputChar` 和 `Backspace` 命令
   - 在 `runner.rs` 中处理字符输入，实时更新用户名和密码字段

2. **优化了视觉效果**
   - 重新设计了登录页面布局
   - 添加了圆角边框和颜色主题
   - 显示光标位置，方便用户输入
   - 改进了帮助文本提示

## 使用方法

### 启动登录

```bash
# 运行程序（如果未登录会自动进入登录页面）
skillhub login

# 或直接运行 CLI
skillhub
```

### 登录界面操作

登录页面提供了一个美观的表单界面，包含以下功能：

#### 输入操作
- **Tab**: 在用户名和密码字段之间切换
- **Enter**: 进入当前字段的编辑模式
- **字符键**: 在编辑模式下输入字符
- **Backspace**: 删除前一个字符

#### 提交登录
- **Ctrl+S**: 提交登录（需要填写用户名和密码）
- 编辑完密码后按 **Enter** 也会自动提交

#### 退出
- **Ctrl+C** 或 **q**: 退出程序

### 视觉效果

登录页面具有以下视觉特性：

1. **居中弹窗**
   - 圆角边框设计
   - 青色（Cyan）主题色
   - 自适应屏幕大小

2. **输入字段**
   - 用户名字段：显示实际输入内容
   - 密码字段：以圆点（•）显示字符，隐藏实际内容
   - 编辑状态：高亮显示当前编辑字段（青色背景）
   - 光标显示：在编辑位置显示光标

3. **状态提示**
   - 编辑状态：显示 "(editing)" 黄色提示
   - 空字段：显示 "(press Enter to edit)" 灰色提示
   - 底部帮助栏：显示可用按键

### 登录流程

1. 启动程序后，如果未登录会自动显示登录页面
2. 按 **Enter** 进入用户名编辑模式
3. 输入用户名
4. 按 **Tab** 或 **Enter** 切换到密码字段
5. 输入密码（字符以圆点显示）
6. 按 **Ctrl+S** 或 **Enter** 提交登录
7. 登录成功后自动跳转到主页

### 配置保存

登录成功后，认证令牌会自动保存到：
```
~/.skillhub/config.toml
```

下次启动程序时会自动使用保存的令牌，无需重复登录。

## 技术实现

### 架构

```
┌─────────────────────────────────────────────────┐
│  用户按下键盘                                    │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────┐
│  handler.rs: handle_key_event()                 │
│  - 将按键转换为 Command                          │
│  - InputChar/Backspace 用于字符输入             │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────┐
│  runner.rs: handle_command()                    │
│  - 处理 InputChar: 添加字符到当前字段           │
│  - 处理 Backspace: 删除最后一个字符              │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────┐
│  render.rs: render_login_page()                 │
│  - 显示用户名和密码字段                          │
│  - 高亮当前编辑字段                              │
│  - 显示光标位置                                  │
└─────────────────────────────────────────────────┘
```

### 关键代码

#### 字符输入处理

```rust
// handler.rs
fn handle_login_key(key: KeyEvent, app: &App) -> Command {
    match key.code {
        KeyCode::Char(c) if app.login.is_editing_username || app.login.is_editing_password => {
            Command::InputChar(c)
        }
        KeyCode::Backspace if app.login.is_editing_username || app.login.is_editing_password => {
            Command::Backspace
        }
        // ...
    }
}

// runner.rs
Command::InputChar(c) => {
    if app.login.is_editing_username {
        app.login.username.push(c);
    } else if app.login.is_editing_password {
        app.login.password.push(c);
    }
}
Command::Backspace => {
    if app.login.is_editing_username {
        app.login.username.pop();
    } else if app.login.is_editing_password {
        app.login.password.pop();
    }
}
```

#### 光标显示

```rust
// render.rs
if app.login.is_editing_username {
    let cursor_x = chunks[2].x + 2 + app.login.username.len() as u16;
    let cursor_y = chunks[2].y + 1;
    f.set_cursor_position(Position::new(cursor_x, cursor_y));
}
```

## 故障排除

### 无法输入字符
- 确保已按 **Enter** 进入编辑模式
- 检查当前字段是否高亮显示（青色背景）

### 登录失败
- 检查服务器地址是否正确
- 验证用户名和密码是否正确
- 查看错误消息提示

### 配置问题
如果需要重新登录，可以删除配置文件：
```bash
rm ~/.skillhub/config.toml
```

## 下一步

登录成功后，您可以：
- 搜索和浏览技能
- 查看技能详情
- 下载和安装技能
- 发布自己的技能
- 管理收藏和通知
