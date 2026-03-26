> 本项目已集成在 [hellcall-desktop](https://github.com/LyceumHewun/hellcall-desktop) 项目，往后不再更新

# HellCall

**HellCall** 是一个专为《绝地潜兵 2》(Helldivers 2) 打造的语音指令工具，能够将你的语音指令转化为游戏内的键盘宏操作，让你通过说话即可快速呼叫战备！

## 特性

- **离线语音识别**：基于 Vosk 提供快速且精准的离线语音识别，保护隐私，无延迟。
- **智能指令匹配**：使用 Levenshtein 模糊匹配算法，即使语音识别有些许偏差也能准确执行对应指令。
- **自定义配置**：高度可定制化，支持自定义语音唤醒词、键盘按键映射、战备宏按键序列以及提示音效。
- **智能按键模拟**：支持自动按下并释放“战备”呼出菜单键，可自定义每个按键的间隔，稳定触发。
- **支持快捷键重发**：通过快捷键可以一键重复上一次呼叫的战备指令。
- **自动拾音与 VAD**：内置 WebRTC VAD 算法，精准判断语音起止时间。

## 安装与运行

### 1. 下载或编译
如果你是通过源码编译：
由于依赖 Vosk，你需要在编译或运行前准备好 Vosk 的动态链接库。
1. 前往 [Vosk Releases](https://github.com/alphacephei/vosk-api/releases) 下载对应平台的动态库（例如 `vosk-win64.zip`）。
2. 将解压出的库文件（如 `libvosk.dll`）放入项目的 `lib/` 文件夹内（如果没有该文件夹请自行创建）。
3. 执行编译命令：
```bash
cargo build --release --features cli
```

### 2. 准备 Vosk 语音模型
HellCall 依赖 [Vosk](https://alphacephei.com/vosk/models) 语音模型。
1. 下载适合你语言的模型（例如中文：`vosk-model-small-cn-0.22`）。
2. 解压到本地目录。
3. 设置环境变量 `VOSK_MODEL_PATH` 指向该模型目录。

**Windows CMD 示例:**

```cmd
set VOSK_MODEL_PATH=C:\path\to\vosk-model-small-cn-0.22
```

**PowerShell 示例:**

```powershell
$env:VOSK_MODEL_PATH="C:\path\to\vosk-model-small-cn-0.22"
```

### 3. 配置文件准备
在当前目录创建一个 `config.toml` 配置文件（或者通过 `HELLCALL_CONFIG_PATH` 环境变量指定其他路径）。配置示例请参考下方[配置说明](#-配置说明)。

如果你需要指令播放提示音效，请在当前目录下创建一个 `audio` 文件夹，并将对应的 `.wav` 音频文件放入其中。

### 4. 运行
运行可执行文件，根据提示选择你的麦克风设备，然后就可以在游戏里大喊呼叫战备了！

## ⚙️ 配置说明 (`config.toml`)

以下是一个完整的配置示例：

```toml
[recognizer]
# 音频识别的时间段 (秒)
chunk_time = 0.2
# 判断语音结束后的静音持续时间 (毫秒)
vad_silence_duration = 500

[key_presser]
# 按下战备呼出键（如 Ctrl）后等待多长时间再按方向键 (毫秒)
wait_open_time = 50
# 每个按键按下的持续时间 (毫秒)
key_release_interval = 20
# 两个不同按键之间的间隔时间 (毫秒)
diff_key_interval = 20

[key_map]
# 键盘按键参考: https://docs.rs/rdev/latest/rdev/enum.Key.html
# 鼠标按键参考：https://docs.rs/rdev/latest/rdev/enum.Button.html
# 将方向映射为物理按键 (W/A/S/D)
UP = "KeyW"
DOWN = "KeyS"
LEFT = "KeyA"
RIGHT = "KeyD"
# 打开战备页面的按键
OPEN = "ControlLeft"
# 重新执行上一次指令的快捷键
RESEND = "BackQuote"
# 扔出战备按键 (如鼠标左键)
THROW = "Left"

[trigger]
# 唤醒词配置：如果设置，必须先说出唤醒词。例如"呼叫 增援"
# 如果为空，则直接说出指令名称即可。
hit_word = "呼叫"

# 指令列表
[[commands]]
# 指令名称（语音识别词）
command = "增援"
# 按键序列 (遵循 key_map 中的定义)
keys = ["OPEN", "UP", "DOWN", "RIGHT", "LEFT", "UP"]
# 触发后随机播放的提示音效，需放置在 audio/ 目录下
audio_files = ["reinforce1.wav", "reinforce2.wav"]

[[commands]]
command = "补给包"
keys = ["OPEN", "DOWN", "LEFT", "DOWN", "UP", "UP", "DOWN"]
audio_files = ["supply.wav"]
```

## 📝 环境变量参数

HellCall 启动时可读取以下环境变量：

| 环境变量名 | 说明 | 默认值 |
| :--- | :--- | :--- |
| `VOSK_MODEL_PATH` | **必填**。Vosk 语音模型的本地文件夹路径。 | 无 |
| `HELLCALL_CONFIG_PATH` | 配置文件的路径。 | `config.toml` |
| `RUST_LOG` | 日志输出级别 (`info`, `warn`, `error`)。 | 无 |

## 🤝 交流与反馈

- **开源协议**: 免费使用，禁止商用。
- **Github**: [https://github.com/LyceumHewun/hellcall](https://github.com/LyceumHewun/hellcall)
- **交流群**: 1062683607

## ⚠️ 免责声明

本软件及相关代码仅用于学习与技术交流目的，旨在改善部分玩家的无障碍游戏体验。
使用本工具可能存在违反游戏服务条款的风险，所产生的任何后果（包括但不限于账号封禁）均由使用者本人承担。作者不对因使用本工具造成的任何直接或间接损失负责。**本工具永久免费，严禁用于任何商业牟利行为！**
