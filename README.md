# HellCall - Helldivers 2 语音控制辅助工具

一个专为 Helldivers 2 游戏设计的语音控制辅助工具，可以通过语音命令自动执行游戏中的快捷键操作和播放对应的音效。

## 功能特性

- 🎤 实时语音识别，支持中文语音命令
- 🎮 自动模拟键盘按键操作
- 🔊 播放对应的游戏音效反馈
- 🎯 支持模糊匹配语音命令
- ⚡ 低延迟响应，实时处理语音输入

## 安装要求

### 系统要求
- Windows 操作系统
- Rust 1.70+ 开发环境

### 系统依赖项

#### 1. 安装 FFmpeg
本项目需要系统环境中安装 FFmpeg：

**Windows 安装步骤：**
1. 从 FFmpeg 官网下载 Windows 版本：https://ffmpeg.org/download.html
2. 解压下载的文件
3. 将 bin 目录添加到系统 PATH 环境变量中
4. 验证安装：在命令行运行 `ffmpeg -version`

#### 2. 安装 Vosk API 库
vosk-rs 需要系统环境中安装 Vosk API 库：

**Windows 安装步骤：**
1. 从 Vosk 官网下载预编译的 Windows 版本：https://alphacephei.com/vosk/install
2. 将下载的 DLL 文件所在目录添加到系统 PATH 环境变量中
3. 或者将 DLL 文件复制到系统目录（如 C:\Windows\System32）

#### 3. 下载语音识别模型
```bash
# 需要安装 Vosk 语音识别模型
# 从 https://alphacephei.com/vosk/models 下载中文模型
# 推荐使用 vosk-model-small-cn-0.22 模型
```

## 安装步骤

1. 克隆项目
```bash
git clone <repository-url>
cd hellcall
```

2. 安装 Rust 依赖
```bash
cargo build --release
```

3. 下载 Vosk 中文模型并解压到指定目录

## 使用方法

1. 运行程序（需要提供 Vosk 模型路径）
```bash
cargo run --release /path/to/vosk-model
```

2. 说出语音命令，以"呼叫"开头，例如：
   - "呼叫飞鹰"
   - "呼叫补给"
   - "呼叫增援"

3. 程序会自动识别语音并执行对应的按键操作和播放音效



## 配置说明

### 命令配置
所有支持的语音命令都在 `main.rs` 文件的 `command_map` 中定义，可以自由修改和扩展：

```rust
let command_map= [
    ("呼叫飞鹰", vec![UP, RIGHT, RIGHT], "eagle.wav"),
    ("呼叫飞鹰五百", vec![UP, RIGHT, DOWN, DOWN, DOWN], "eagle.wav"),
    ("呼叫飞鹰空袭", vec![UP, RIGHT, DOWN, LEFT], "eagle.wav"),
    ("呼叫飞鹰集束弹", vec![UP, RIGHT, DOWN, DOWN, RIGHT], "eagle.wav"),
    ("呼叫飞鹰汽油弹", vec![UP, RIGHT, DOWN, UP], "eagle.wav"),
    ("呼叫飞鹰烟雾弹", vec![UP, RIGHT, UP, DOWN], "eagle.wav"),
    ("呼叫飞鹰一百一", vec![UP, RIGHT, UP, LEFT], "eagle.wav"),
    ("呼叫补给", vec![DOWN, DOWN, UP, RIGHT], "resupply.wav"),
    ("呼叫补给包", vec![DOWN, LEFT, DOWN, UP, UP, DOWN], "resupply.wav"),
    ("呼叫轨道火", vec![RIGHT, RIGHT, DOWN, LEFT, RIGHT, UP], "orbital_napalm_barrage.wav"),
    ("呼叫榴弹枪", vec![DOWN, LEFT, UP, LEFT, DOWN], "resupply.wav"),
    ("呼叫增援", vec![UP, DOWN, RIGHT, LEFT, UP], "reinforce.wav"),
];
```

### 按键映射配置
在 `main.rs` 中可以自定义按键映射：
```rust
 let mut key_map: HashMap<LocalKey, Key> = HashMap::new();
 key_map.insert(UP, Key::KeyW);      // W 键
 key_map.insert(DOWN, Key::KeyS);    // S 键
 key_map.insert(LEFT, Key::KeyA);    // A 键
 key_map.insert(RIGHT, Key::KeyD);   // D 键
 key_map.insert(CTRL, Key::ControlLeft); // Ctrl 键
 ```

### 语音识别配置
```rust
let config = AudioRecognizerConfig {
    chunk_time: 0.2,           // 0.2 秒识别一次
    grammar,                   // 识别语法
    vad_silence_duration: 500, // 静音检测时长(ms)
};
```

### 音频文件指南

音频文件存放在 `audio/` 目录下

## 开发说明

### 添加新命令
1. 在 `command_map` 中添加新的命令元组
2. 提供对应的按键序列和音效文件
3. 更新语法识别列表

### 构建发布版本
```bash
cargo build --release
```

### 运行测试
```bash
cargo test
```

## 免责声明

本项目仅用于学习和技术研究目的，请勿用于破坏游戏平衡或违反游戏规则的行为。