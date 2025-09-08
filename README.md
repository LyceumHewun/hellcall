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
所有支持的语音命令都在 `main.rs` 文件的 `command_map` 中定义，可以自由修改和扩展


| icon | Stratagem | 语音命令 | 音频 |
|------|-----------|----------|----------|
| ![MG-43 Machine Gun](https://helldivers.wiki.gg/images/thumb/Machine_Gun_Stratagem_Icon.png/50px-Machine_Gun_Stratagem_Icon.png?3dfe18) | MG-43 Machine Gun | 机枪 |  |
| ![APW-1 Anti-Materiel Rifle](https://helldivers.wiki.gg/images/thumb/Anti-Materiel_Rifle_Stratagem_Icon.png/50px-Anti-Materiel_Rifle_Stratagem_Icon.png?fcf5b1) | APW-1 Anti-Materiel Rifle | 反器材步枪 |  |
| ![M-105 Stalwart](https://helldivers.wiki.gg/images/thumb/Stalwart_Stratagem_Icon.png/50px-Stalwart_Stratagem_Icon.png?b35d66) | M-105 Stalwart | 盟友 |  |
| ![GR-8 Recoilless Rifle](https://helldivers.wiki.gg/images/thumb/Expendable_Anti-Tank_Stratagem_Icon.png/50px-Expendable_Anti-Tank_Stratagem_Icon.png?2634c5) | GR-8 Recoilless Rifle | 一次性反坦克炮 |  |
| ![FLAM-40 Flamethrower](https://helldivers.wiki.gg/images/thumb/Flamethrower_Stratagem_Icon.png/50px-Flamethrower_Stratagem_Icon.png?f36171) | FLAM-40 Flamethrower | 喷火器 |  |
| ![AC-8 Autocannon](https://helldivers.wiki.gg/images/thumb/Autocannon_Stratagem_Icon.png/50px-Autocannon_Stratagem_Icon.png?fcd624) | AC-8 Autocannon | 机炮 |  |
| ![MG-206 Heavy Machine Gun](https://helldivers.wiki.gg/images/thumb/Heavy_Machine_Gun_Stratagem_Icon.png/50px-Heavy_Machine_Gun_Stratagem_Icon.png?910281) | MG-206 Heavy Machine Gun | 重机枪 |  |
| ![Volley Gun](https://helldivers.wiki.gg/images/thumb/RL-77_Airburst_Rocket_Launcher_Stratagem_Icon.png/50px-RL-77_Airburst_Rocket_Launcher_Stratagem_Icon.png?ccc753) | Volley Gun | 空爆火箭发射器 |  |
| ![Assault Rifle](https://helldivers.wiki.gg/images/thumb/Commando_Stratagem_Icon.png/50px-Commando_Stratagem_Icon.png?30c1d0) | Assault Rifle | 突击兵 |  |
| ![Railgun](https://helldivers.wiki.gg/images/thumb/Railgun_Stratagem_Icon.png/50px-Railgun_Stratagem_Icon.png?e120a8) | Railgun | 磁轨炮 |  |
| ![Spear](https://helldivers.wiki.gg/images/thumb/Spear_Stratagem_Icon.png/50px-Spear_Stratagem_Icon.png?e120a8) | Spear | 飞矛 |  |
| ![Recoilless Rifle](https://helldivers.wiki.gg/images/thumb/Recoilless_Rifle_Stratagem_Icon.png/50px-Recoilless_Rifle_Stratagem_Icon.png?e120a8) | Recoilless Rifle | 火箭发射器 |  |
| ![Orbital Gatling Barrage](https://helldivers.wiki.gg/images/thumb/Orbital_Gatling_Barrage_Stratagem_Icon.png/50px-Orbital_Gatling_Barrage_Stratagem_Icon.png?e120a8) | Orbital Gatling Barrage | 轨道加特林 |  |
| ![Orbital Airburst Strike](https://helldivers.wiki.gg/images/thumb/Orbital_Airburst_Strike_Stratagem_Icon.png/50px-Orbital_Airburst_Strike_Stratagem_Icon.png?e120a8) | Orbital Airburst Strike | 轨道空爆 |  |
| ![Orbital 120MM HE Barrage](https://helldivers.wiki.gg/images/thumb/Orbital_120mm_HE_Barrage_Stratagem_Icon.png/50px-Orbital_120mm_HE_Barrage_Stratagem_Icon.png?e120a8) | Orbital 120MM HE Barrage | 轨道一二零 |  |
| ![Orbital 380MM HE Barrage](https://helldivers.wiki.gg/images/thumb/Orbital_380mm_HE_Barrage_Stratagem_Icon.png/50px-Orbital_380mm_HE_Barrage_Stratagem_Icon.png?e120a8) | Orbital 380MM HE Barrage | 轨道三八零 |  |
| ![Orbital Walking Barrage](https://helldivers.wiki.gg/images/thumb/Orbital_Walking_Barrage_Stratagem_Icon.png/50px-Orbital_Walking_Barrage_Stratagem_Icon.png?e120a8) | Orbital Walking Barrage | 游走轨道炮 |  |
| ![Orbital Laser](https://helldivers.wiki.gg/images/thumb/Orbital_Laser_Stratagem_Icon.png/50px-Orbital_Laser_Stratagem_Icon.png?e120a8) | Orbital Laser | 轨道激光 |  |
| ![Orbital Incendiary Mines](https://helldivers.wiki.gg/images/thumb/Orbital_Napalm_Barrage_Stratagem_Icon.png/50px-Orbital_Napalm_Barrage_Stratagem_Icon.png?e6fc05) | Orbital Incendiary Mines | 轨道燃烧弹 | orbital_napalm_barrage.wav |
| ![Orbital Railcannon Strike](https://helldivers.wiki.gg/images/thumb/Orbital_Railcannon_Strike_Stratagem_Icon.png/50px-Orbital_Railcannon_Strike_Stratagem_Icon.png?e120a8) | Orbital Railcannon Strike | 轨道炮 |  |
| ![Eagle Strafing Run](https://helldivers.wiki.gg/images/thumb/Eagle_Strafing_Run_Stratagem_Icon.png/50px-Eagle_Strafing_Run_Stratagem_Icon.png?e6ad30) | Eagle Strafing Run | 飞鹰扫射 | eagle.wav |
| ![Eagle Airstrike](https://helldivers.wiki.gg/images/thumb/Eagle_Airstrike_Stratagem_Icon.png/50px-Eagle_Airstrike_Stratagem_Icon.png?685944) | Eagle Airstrike | 飞鹰空袭 | eagle.wav |
| ![Eagle Cluster Bomb](https://helldivers.wiki.gg/images/thumb/Eagle_Cluster_Bomb_Stratagem_Icon.png/50px-Eagle_Cluster_Bomb_Stratagem_Icon.png?4c4860) | Eagle Cluster Bomb | 飞鹰集束弹 | eagle.wav |
| ![Eagle Napalm Airstrike](https://helldivers.wiki.gg/images/thumb/Eagle_Napalm_Airstrike_Stratagem_Icon.png/50px-Eagle_Napalm_Airstrike_Stratagem_Icon.png?ab5aa8) | Eagle Napalm Airstrike | 飞鹰燃烧弹 | eagle.wav |
| ![Jump Pack](https://helldivers.wiki.gg/images/thumb/Jump_Pack_Stratagem_Icon.png/50px-Jump_Pack_Stratagem_Icon.png?e120a8) | Jump Pack | 跳包 |  |
| ![Eagle Smoke Strike](https://helldivers.wiki.gg/images/thumb/Eagle_Smoke_Strike_Stratagem_Icon.png/50px-Eagle_Smoke_Strike_Stratagem_Icon.png?e120a8) | Eagle Smoke Strike | 飞鹰烟雾弹 | eagle.wav |
| ![Eagle 110MM Rocket Pods](https://helldivers.wiki.gg/images/thumb/Eagle_110mm_Rocket_Pods_Stratagem_Icon.png/50px-Eagle_110mm_Rocket_Pods_Stratagem_Icon.png?e120a8) | Eagle 110MM Rocket Pods | 飞鹰火箭弹 | eagle.wav |
| ![Eagle 500KG Bomb](https://helldivers.wiki.gg/images/thumb/Eagle_500kg_Bomb_Stratagem_Icon.png/50px-Eagle_500kg_Bomb_Stratagem_Icon.png?e120a8) | Eagle 500KG Bomb | 飞鹰航弹 | eagle.wav |
| ![Light Armored Vehicle](https://helldivers.wiki.gg/images/thumb/M-102_Fast_Recon_Vehicle_Stratagem_Icon.png/50px-M-102_Fast_Recon_Vehicle_Stratagem_Icon.png?8cb2ad) | Light Armored Vehicle | 快速侦察车 |  |
| ![Orbital Precision Strike](https://helldivers.wiki.gg/images/thumb/Orbital_Precision_Strike_Stratagem_Icon.png/50px-Orbital_Precision_Strike_Stratagem_Icon.png?561f51) | Orbital Precision Strike | 轨道打击 |  |
| ![Orbital Gas Strike](https://helldivers.wiki.gg/images/thumb/Orbital_Gas_Strike_Stratagem_Icon.png/50px-Orbital_Gas_Strike_Stratagem_Icon.png?5d9ba4) | Orbital Gas Strike | 轨道毒气 |  |
| ![Orbital EMS Strike](https://helldivers.wiki.gg/images/thumb/Orbital_EMS_Strike_Stratagem_Icon.png/50px-Orbital_EMS_Strike_Stratagem_Icon.png?77534f) | Orbital EMS Strike | 轨道电磁干扰 |  |
| ![Orbital Smoke Strike](https://helldivers.wiki.gg/images/thumb/Orbital_Smoke_Strike_Stratagem_Icon.png/50px-Orbital_Smoke_Strike_Stratagem_Icon.png?a063f8) | Orbital Smoke Strike | 轨道烟雾空袭 |  |
| ![E/MG-101 HMG Emplacement](https://helldivers.wiki.gg/images/thumb/HMG_Emplacement_Stratagem_Icon.png/50px-HMG_Emplacement_Stratagem_Icon.png?5f9e66) | E/MG-101 HMG Emplacement | 重机枪炮台 |  |
| ![FX-12 Shield Generator Relay](https://helldivers.wiki.gg/images/thumb/Shield_Generator_Relay_Stratagem_Icon.png/50px-Shield_Generator_Relay_Stratagem_Icon.png?64b940) | FX-12 Shield Generator Relay | 防护罩生成中继器 |  |
| ![Tesla Tower](https://helldivers.wiki.gg/images/thumb/Tesla_Tower_Stratagem_Icon.png/50px-Tesla_Tower_Stratagem_Icon.png?e120a8) | Tesla Tower | 特斯拉塔 |  |
| ![Grenadier Bunker](https://helldivers.wiki.gg/images/thumb/GL-21_Grenadier_Battlement_Stratagem_Icon.png/50px-GL-21_Grenadier_Battlement_Stratagem_Icon.png?747ef0) | Grenadier Bunker | 掷弹兵掩体 |  |
| ![Anti-Personnel Minefield](https://helldivers.wiki.gg/images/thumb/Anti-Personnel_Minefield_Stratagem_Icon.png/50px-Anti-Personnel_Minefield_Stratagem_Icon.png?e120a8) | Anti-Personnel Minefield | 反步兵地雷 |  |
| ![Supply Pack](https://helldivers.wiki.gg/images/thumb/Supply_Pack_Stratagem_Icon.png/50px-Supply_Pack_Stratagem_Icon.png?e120a8) | Supply Pack | 补给背包 |  |
| ![Grenade Launcher](https://helldivers.wiki.gg/images/thumb/Grenade_Launcher_Stratagem_Icon.png/50px-Grenade_Launcher_Stratagem_Icon.png?e120a8) | Grenade Launcher | 榴弹发射器 |  |
| ![Laser Cannon](https://helldivers.wiki.gg/images/thumb/Laser_Cannon_Stratagem_Icon.png/50px-Laser_Cannon_Stratagem_Icon.png?e120a8) | Laser Cannon | 激光大炮 |  |
| ![Incendiary Mines](https://helldivers.wiki.gg/images/thumb/Incendiary_Mines_Stratagem_Icon.png/50px-Incendiary_Mines_Stratagem_Icon.png?e120a8) | Incendiary Mines | 燃烧地雷 |  |
| ![Guard Dog Rover](https://helldivers.wiki.gg/images/thumb/Guard_Dog_Rover_Stratagem_Icon.png/50px-Guard_Dog_Rover_Stratagem_Icon.png?e120a8) | Guard Dog Rover | 激光狗 |  |
| ![Ballistic Shield Backpack](https://helldivers.wiki.gg/images/thumb/Ballistic_Shield_Backpack_Stratagem_Icon.png/50px-Ballistic_Shield_Backpack_Stratagem_Icon.png?e120a8) | Ballistic Shield Backpack | 防弹背包 |  |
| ![Arc Thrower](https://helldivers.wiki.gg/images/thumb/Arc_Thrower_Stratagem_Icon.png/50px-Arc_Thrower_Stratagem_Icon.png?e120a8) | Arc Thrower | 电弧发射器 |  |
| ![Anti-Tank Mines](https://helldivers.wiki.gg/images/thumb/MD-17_Anti-Tank_Mines_Stratagem_Icon.png/50px-MD-17_Anti-Tank_Mines_Stratagem_Icon.png?589106) | Anti-Tank Mines | 反坦克地雷 |  |
| ![Quasar Cannon](https://helldivers.wiki.gg/images/thumb/Quasar_Cannon_Stratagem_Icon.png/50px-Quasar_Cannon_Stratagem_Icon.png?e120a8) | Quasar Cannon | 类星体炮 |  |
| ![Shield Generator Pack](https://helldivers.wiki.gg/images/thumb/Shield_Generator_Pack_Stratagem_Icon.png/50px-Shield_Generator_Pack_Stratagem_Icon.png?e120a8) | Shield Generator Pack | 防护罩背包 |  |
| ![Gas Mines](https://helldivers.wiki.gg/images/thumb/Gas_Minefield_Stratagem_Icon.png/50px-Gas_Minefield_Stratagem_Icon.png?3b8000) | Gas Mines | 毒气地雷 |  |
| ![Machine Gun Sentry](https://helldivers.wiki.gg/images/thumb/Machine_Gun_Sentry_Stratagem_Icon.png/50px-Machine_Gun_Sentry_Stratagem_Icon.png?e120a8) | Machine Gun Sentry | 哨戒机枪 |  |
| ![Gatling Sentry](https://helldivers.wiki.gg/images/thumb/Gatling_Sentry_Stratagem_Icon.png/50px-Gatling_Sentry_Stratagem_Icon.png?e120a8) | Gatling Sentry | 哨戒加特林 |  |
| ![Mortar Sentry](https://helldivers.wiki.gg/images/thumb/Mortar_Sentry_Stratagem_Icon.png/50px-Mortar_Sentry_Stratagem_Icon.png?e120a8) | Mortar Sentry | 哨戒迫击炮 |  |
| ![Guard Dog](https://helldivers.wiki.gg/images/thumb/Guard_Dog_Stratagem_Icon.png/50px-Guard_Dog_Stratagem_Icon.png?e120a8) | Guard Dog | 机枪狗 |  |
| ![Autocannon Sentry](https://helldivers.wiki.gg/images/thumb/Autocannon_Sentry_Stratagem_Icon.png/50px-Autocannon_Sentry_Stratagem_Icon.png?e120a8) | Autocannon Sentry | 哨戒机炮 |  |
| ![Rocket Sentry](https://helldivers.wiki.gg/images/thumb/Rocket_Sentry_Stratagem_Icon.png/50px-Rocket_Sentry_Stratagem_Icon.png?e120a8) | Rocket Sentry | 哨戒火箭炮 |  |
| ![EMS Mortar Sentry](https://helldivers.wiki.gg/images/thumb/AM-23_EMS_Mortar_Sentry_Stratagem_Icon.png/50px-AM-23_EMS_Mortar_Sentry_Stratagem_Icon.png?f0de8a) | EMS Mortar Sentry | 哨戒电磁迫击炮 |  |
| ![EXO-44 Patriot Exosuit](https://helldivers.wiki.gg/images/thumb/EXO-45_Patriot_Exosuit_Stratagem_Icon.png/50px-EXO-45_Patriot_Exosuit_Stratagem_Icon.png?64a72f) | EXO-44 Patriot Exosuit | 爱国者装甲 |  |
| ![EXO-48 Liberator Exosuit](https://helldivers.wiki.gg/images/thumb/EXO-49_Emancipator_Exosuit_Stratagem_Icon.png/50px-EXO-49_Emancipator_Exosuit_Stratagem_Icon.png?6f2e3c) | EXO-48 Liberator Exosuit | 解放者装甲 |  |
| ![Toxin Sprayer](https://helldivers.wiki.gg/images/thumb/Sterilizer_Stratagem_Icon.png/50px-Sterilizer_Stratagem_Icon.png?5f6a3c) | Toxin Sprayer | 喷毒枪 |  |
| ![Guard Dog Toxin](https://helldivers.wiki.gg/images/thumb/Guard_Dog_Dog_Breath_Stratagem_Icon.png/50px-Guard_Dog_Dog_Breath_Stratagem_Icon.png?9e6385) | Guard Dog Toxin | 毒狗 |  |
| ![Directional Shield](https://helldivers.wiki.gg/images/thumb/SH-51_Directional_Shield_Stratagem_Icon.png/50px-SH-51_Directional_Shield_Stratagem_Icon.png?4e63ec) | Directional Shield | 定向护盾 |  |
| ![Anti-Tank Emplacement](https://helldivers.wiki.gg/images/thumb/E_AT-12_Anti-Tank_Emplacement_Stratagem_Icon.png/50px-E_AT-12_Anti-Tank_Emplacement_Stratagem_Icon.png?3fbc70) | Anti-Tank Emplacement | 反坦克炮台 |  |
| ![Incendiary Rocket Sentry](https://helldivers.wiki.gg/images/thumb/A_FLAM-40_Flame_Sentry_Stratagem_Icon.png/50px-A_FLAM-40_Flame_Sentry_Stratagem_Icon.png?57c6a5) | Incendiary Rocket Sentry | 哨戒火焰火箭炮 |  |
| ![Hellbomb](https://helldivers.wiki.gg/images/thumb/Portable_Hellbomb_Stratagem_Icon.png/50px-Portable_Hellbomb_Stratagem_Icon.png?c9a263) | Hellbomb | 便携地狱火炸弹 |  |
| ![Hover Pack](https://helldivers.wiki.gg/images/thumb/Hover_Pack_Stratagem_Icon.png/50px-Hover_Pack_Stratagem_Icon.png?e120a8) | Hover Pack | 悬浮背包 |  |
| ![Flag of Super Earth](https://helldivers.wiki.gg/images/thumb/CQC-1_One_True_Flag_Stratagem_Icon.png/50px-CQC-1_One_True_Flag_Stratagem_Icon.png?3cf8b5) | Flag of Super Earth | 唯一真旗 |  |
| ![Guard Dog Electricity](https://helldivers.wiki.gg/images/thumb/AX_ARC-3_%22Guard_Dog%22_K-9_Stratagem_Icon.png/50px-AX_ARC-3_%22Guard_Dog%22_K-9_Stratagem_Icon.png?8ac008) | Guard Dog Electricity | 电狗 |  |
| ![Arc Thrower](https://helldivers.wiki.gg/images/thumb/GL-52_De-Escalator_Stratagem_Icon.png/50px-GL-52_De-Escalator_Stratagem_Icon.png?2fe4cc) | Arc Thrower | 电磁榴弹发射器 |  |
| ![Laser Sentry](https://helldivers.wiki.gg/images/thumb/A_LAS-98_Laser_Sentry_Stratagem_Icon.png/50px-A_LAS-98_Laser_Sentry_Stratagem_Icon.png?684009) | Laser Sentry | 哨戒激光炮 |  |
| ![Teleport Pack](https://helldivers.wiki.gg/images/thumb/LIFT-182_Warp_Pack_Stratagem_Icon.png/50px-LIFT-182_Warp_Pack_Stratagem_Icon.png?ea0112) | Teleport Pack | 传送背包 |  |
| ![Era](https://helldivers.wiki.gg/images/thumb/PLAS-45_Epoch_Stratagem_Icon.png/50px-PLAS-45_Epoch_Stratagem_Icon.png?691ee9) | Era | 纪元 |  |
|  | Missile Silo | 导弹发射井 |  |
|  | One-Time Use Napalm Bomb | 一次性凝固汽油弹 |  |
|  | Toxin Spear | 毒矛 |  |
| ![Reinforce](https://helldivers.wiki.gg/images/thumb/Reinforce_Stratagem_Icon.png/50px-Reinforce_Stratagem_Icon.png?e120a8) | Reinforce | 增援 | reinforce.wav |
| ![SOS Beacon](https://helldivers.wiki.gg/images/thumb/SOS_Beacon_Stratagem_Icon.png/50px-SOS_Beacon_Stratagem_Icon.png?e120a8) | SOS Beacon | 求救信标 |  |
| ![Resupply](https://helldivers.wiki.gg/images/thumb/Resupply_Stratagem_Icon.png/50px-Resupply_Stratagem_Icon.png?e120a8) | Resupply | 补给 | resupply.wav |
| ![Eagle Rearm](https://helldivers.wiki.gg/images/thumb/Eagle_Rearm_Stratagem_Icon.png/50px-Eagle_Rearm_Stratagem_Icon.png?e120a8) | Eagle Rearm | 飞鹰返航 |  |
| ![Hellfire Bomb](https://helldivers.wiki.gg/images/thumb/Hellbomb_Stratagem_Icon.png/50px-Hellbomb_Stratagem_Icon.png?6c240b) | Hellfire Bomb | 地狱火炸弹 |  |
| ![Super Earth Cannon](https://helldivers.wiki.gg/images/thumb/SEAF_Artillery_Stratagem_Icon.png/50px-SEAF_Artillery_Stratagem_Icon.png?19a114) | Super Earth Cannon | 超级地球大炮 |  |


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