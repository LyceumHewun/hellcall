// #![allow(unused)]

use anyhow::{Context, Result};
use rdev::Key;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
#[cfg(not(debug_assertions))]
use vosk::{LogLevel};
use log::{info, warn};

use core::audio::*;
use core::command::*;
use core::keypress::*;
use core::keypress::LocalKey::{CTRL, DOWN, LEFT, RIGHT, UP};
use core::matcher::fuzzy::*;
use core::speaker::*;

mod core;

fn main() -> Result<()> {
    #[cfg(not(debug_assertions))]
    vosk::set_log_level(LogLevel::Error);
    env_logger::init();

    let model_path = env::args().nth(1).context("module path is empty")?;
    let grammar = vec![
        "呼叫",
        "机枪",
        "反 器材 步枪",
        "盟友",
        "一次性 反 坦克 炮",
        "无 后 坐 力 炮",
        "喷火 器",
        "机 炮",
        "重机枪",
        "空 爆 火箭 发射器",
        "突击 兵",
        "磁 轨 炮",
        "飞 矛",
        "火箭 发射器",
        "轨道 加特林",
        "轨道 空 爆",
        "轨道 一 二 零",
        "轨道 三 八 零",
        "游走 轨道 炮",
        "轨道 激光",
        "轨道 燃烧弹",
        "轨道 炮",
        "飞 鹰 扫射",
        "飞 鹰 空袭",
        "飞 鹰 集 束 弹",
        "飞 鹰 燃烧 弹",
        "跳 包",
        "飞 鹰 烟雾弹",
        "飞 鹰 火箭弹",
        "飞 鹰 航 弹",
        "快速 侦察 车",
        "轨道 打击",
        "轨道 毒气",
        "轨道 电磁 干扰",
        "轨道 烟雾 空袭",
        "重 机枪 炮台",
        "防护罩 生成 中 继 器",
        "特斯拉 塔",
        "掷 弹 兵 掩体",
        "反 步兵 地雷",
        "补给 背包",
        "榴弹 发射器",
        "激光 大炮",
        "燃烧 地雷",
        "激光 狗",
        "防弹 背包",
        "电弧 发射器",
        "反 坦克 地雷",
        "类 星体 炮",
        "防护罩 背包",
        "毒气 地雷",
        "哨 戒 机枪",
        "哨 戒 加特林",
        "哨 戒 迫击炮",
        "机枪 狗",
        "哨 戒 机 炮",
        "哨 戒 火箭 炮",
        "哨 戒 电磁 迫击炮",
        "爱国 者 装甲",
        "解放 者 装甲",
        "喷 毒 枪",
        "毒 狗",
        "定向 护盾",
        "反 坦克 炮台",
        "哨 戒 火焰 火箭炮",
        "便携 地狱 火 炸弹",
        "悬浮 背包",
        "唯一 真 旗",
        "电 狗",
        "电磁 榴弹 发射器",
        "哨 戒 激光 炮",
        "传送 背包",
        "纪元",
        "导弹 发射 井",
        "一次性 凝固 汽油 弹",
        "毒 矛",
        "增援",
        "求救 信 标",
        "补给",
        "飞 鹰 返航",
        "地狱 火 炸弹",
        "超级 地球 大炮",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let mut key_map: HashMap<LocalKey, Key> = HashMap::new();
    key_map.insert(UP, Key::KeyW);
    key_map.insert(DOWN, Key::KeyS);
    key_map.insert(LEFT, Key::KeyA);
    key_map.insert(RIGHT, Key::KeyD);
    key_map.insert(CTRL, Key::ControlLeft);
    let key_presser = Arc::new(KeyPresser::new(key_map));
    let speaker = Arc::new(Speaker::new()?);

    let command_map= [
        // Patriotic Administration Center 爱国行政中心
        ("机枪", vec![DOWN, LEFT, DOWN, UP, RIGHT], ""), // ↓ ← ↓ ↑ →
        ("反器材步枪", vec![DOWN, LEFT, RIGHT, UP, DOWN], ""), // ↓ ← → ↑ ↓
        ("盟友", vec![DOWN, LEFT, DOWN, UP, UP, LEFT], ""), // ↓ ← ↓ ↑ ↑ ←
        ("一次性反坦克炮", vec![DOWN, DOWN, LEFT, UP, RIGHT], ""), // ↓ ↓ ← ↑ →
        ("无后坐力炮", vec![DOWN, LEFT, RIGHT, RIGHT, LEFT], ""), // ↓ ← → → ←
        ("喷火器", vec![DOWN, LEFT, UP, DOWN, UP], ""), // ↓ ← ↑ ↓ ↑
        ("机炮", vec![DOWN, LEFT, DOWN, UP, UP, RIGHT], ""), // ↓ ← ↓ ↑ ↑ →
        ("重机枪", vec![DOWN, LEFT, UP, DOWN, DOWN], ""), // ↓ ← ↑ ↓ ↓
        ("空爆火箭发射器", vec![DOWN, UP, UP, LEFT, RIGHT], ""), // ↓ ↑ ↑ ← →
        ("突击兵", vec![DOWN, LEFT, UP, DOWN, RIGHT], ""), // ↓ ← ↑ ↓ →
        ("磁轨炮", vec![DOWN, RIGHT, DOWN, UP, LEFT, RIGHT], ""), // ↓ → ↓ ↑ ← →
        ("飞矛", vec![DOWN, DOWN, UP, DOWN, DOWN], ""), // ↓ ↓ ↑ ↓ ↓
        ("火箭发射器", vec![DOWN, DOWN, UP, DOWN, RIGHT], ""), // ↓ ↓ ↑ ↓ →
        // Orbital Cannons 轨道炮
        ("轨道加特林", vec![RIGHT, DOWN, LEFT, UP, UP], ""), // → ↓ ← ↑ ↑
        ("轨道空爆", vec![RIGHT, RIGHT, RIGHT], ""), // → → →
        ("轨道一二零", vec![RIGHT, RIGHT, DOWN, LEFT, RIGHT, DOWN], ""), // → → ↓ ← → ↓
        ("轨道三八零", vec![RIGHT, DOWN, UP, UP, LEFT, DOWN, DOWN], ""), // → ↓ ↑ ↑ ← ↓ ↓
        ("游走轨道炮", vec![RIGHT, DOWN, RIGHT, DOWN, RIGHT, DOWN], ""), // → ↓ → ↓ → ↓
        ("轨道激光", vec![RIGHT, DOWN, UP, RIGHT, DOWN], ""), // → ↓ ↑ → ↓
        ("轨道燃烧弹", vec![RIGHT, RIGHT, DOWN, LEFT, RIGHT, UP], "orbital_napalm_barrage.wav"), // → → ↓ ← → ↑
        ("轨道炮", vec![RIGHT, UP, DOWN, DOWN, RIGHT], ""), // → ↑ ↓ ↓ →
        // Hangar 机库
        ("飞鹰扫射", vec![UP, RIGHT, RIGHT], "eagle.wav"), // ↑ → →
        ("飞鹰空袭", vec![UP, RIGHT, DOWN, RIGHT], "eagle.wav"), // ↑ → ↓ →
        ("飞鹰集束弹", vec![UP, RIGHT, DOWN, DOWN, RIGHT], "eagle.wav"), // ↑ → ↓ ↓ →
        ("飞鹰燃烧弹", vec![UP, RIGHT, DOWN, UP], "eagle.wav"), // ↑ → ↓ ↑
        ("跳包", vec![DOWN, UP, UP, DOWN, UP], ""), // ↓ ↑ ↑ ↓ ↑
        ("飞鹰烟雾弹", vec![UP, RIGHT, UP, DOWN], "eagle.wav"), // ↑ → ↑ ↓
        ("飞鹰火箭弹", vec![UP, RIGHT, UP, LEFT], "eagle.wav"), // ↑ → ↑ ←
        ("飞鹰航弹", vec![UP, RIGHT, DOWN, DOWN, DOWN], "eagle.wav"), // ↑ → ↓ ↓ ↓
        ("快速侦察车", vec![LEFT, DOWN, RIGHT, DOWN, RIGHT, DOWN, UP], ""), // ← ↓ → ↓ → ↓ ↑
        // Bridge 舰桥
        ("轨道打击", vec![RIGHT, RIGHT, UP], ""), // → → ↑
        ("轨道毒气", vec![RIGHT, RIGHT, DOWN, RIGHT], ""), // → → ↓ →
        ("轨道电磁干扰", vec![RIGHT, RIGHT, LEFT, DOWN], ""), // → → ← ↓
        ("轨道烟雾空袭", vec![RIGHT, RIGHT, DOWN, UP], ""), // → → ↓ ↑
        ("重机枪炮台", vec![DOWN, UP, LEFT, RIGHT, RIGHT, LEFT], ""), // ↓ ↑ ← → → ←
        ("防护罩生成中继器", vec![DOWN, DOWN, LEFT, RIGHT, LEFT, RIGHT], ""), // ↓ ↓ ← → ← →
        ("特斯拉塔", vec![DOWN, UP, RIGHT, UP, LEFT, RIGHT], ""), // ↓ ↑ → ↑ ← →
        ("掷弹兵掩体", vec![DOWN, RIGHT, DOWN, LEFT, RIGHT], ""), // ↓ → ↓ ← →
        // Engineering Bay 工程湾
        ("反步兵地雷", vec![DOWN, LEFT, UP, RIGHT], ""), // ↓ ← ↑ →
        ("补给背包", vec![DOWN, LEFT, DOWN, UP, UP, DOWN], ""), // ↓ ← ↓ ↑ ↑ ↓
        ("榴弹发射器", vec![DOWN, LEFT, UP, LEFT, DOWN], ""), // ↓ ← ↑ ← ↓
        ("激光大炮", vec![DOWN, LEFT, DOWN, UP, LEFT], ""), // ↓ ← ↓ ↑ ←
        ("燃烧地雷", vec![DOWN, LEFT, LEFT, DOWN], ""), // ↓ ← ← ↓
        ("激光狗", vec![DOWN, UP, LEFT, UP, RIGHT, RIGHT], ""), // ↓ ↑ ← ↑ → →
        ("防弹背包", vec![DOWN, LEFT, DOWN, DOWN, UP, LEFT], ""), // ↓ ← ↓ ↓ ↑ ←
        ("电弧发射器", vec![DOWN, RIGHT, DOWN, UP, LEFT, LEFT], ""), // ↓ → ↓ ↑ ← ←
        ("反坦克地雷", vec![DOWN, LEFT, UP, UP], ""), // ↓ ← ↑ ↑
        ("类星体炮", vec![DOWN, DOWN, UP, LEFT, RIGHT], ""), // ↓ ↓ ↑ ← →
        ("防护罩背包", vec![DOWN, UP, LEFT, RIGHT, LEFT, RIGHT], ""), // ↓ ↑ ← → ← →
        ("毒气地雷", vec![DOWN, LEFT, LEFT, RIGHT], ""), // ↓ ← ← →
        // Robotics Workshop 机器厂房
        ("哨戒机枪", vec![DOWN, UP, RIGHT, RIGHT, UP], ""), // ↓ ↑ → → ↑
        ("哨戒加特林", vec![DOWN, UP, RIGHT, LEFT], ""), // ↓ ↑ → ←
        ("哨戒迫击炮", vec![DOWN, UP, RIGHT, RIGHT, DOWN], ""), // ↓ ↑ → → ↓
        ("机枪狗", vec![DOWN, UP, LEFT, UP, RIGHT, DOWN], ""), // ↓ ↑ ← ↑ → ↓
        ("哨戒机炮", vec![DOWN, UP, RIGHT, UP, LEFT, UP], ""), // ↓ ↑ → ↑ ← ↑
        ("哨戒火箭炮", vec![DOWN, UP, RIGHT, RIGHT, LEFT], ""), // ↓ ↑ → → ←
        ("哨戒电磁迫击炮", vec![DOWN, UP, RIGHT, DOWN, RIGHT], ""), // ↓  ↑  → ↓ →
        ("爱国者装甲", vec![LEFT, DOWN, RIGHT, UP, LEFT, DOWN, DOWN], ""), // ← ↓ → ↑ ← ↓ ↓
        ("解放者装甲", vec![LEFT, DOWN, RIGHT, UP, LEFT, DOWN, UP], ""), // ← ↓ → ↑ ← ↓ ↑
        // Warbonds 战争债卷
        // 化学特务
        ("喷毒枪", vec![DOWN, LEFT, UP, DOWN, LEFT], ""), // ↓ ← ↑ ↓ ←
        ("毒狗", vec![DOWN, UP, LEFT, UP, RIGHT, UP], ""), // ↓ ↑ ← ↑ → ↑
        // 都市传说
        ("定向护盾", vec![DOWN, UP, LEFT, RIGHT, UP, UP], ""), // ↓ ↑ ← → ↑ ↑
        ("反坦克炮台", vec![DOWN, UP, LEFT, RIGHT, RIGHT, RIGHT], ""), // ↓ ↑ ← → → →
        ("哨戒火焰火箭炮", vec![DOWN, UP, RIGHT, DOWN, UP, UP], ""), // ↓ ↑ → ↓ ↑ ↑
        // 自由之仆
        ("便携地狱火炸弹", vec![DOWN, RIGHT, UP, UP, UP], ""), // ↓ → ↑ ↑ ↑
        // 边缘正义
        ("悬浮背包", vec![DOWN, UP, UP, DOWN, LEFT, RIGHT], ""), // ↓ ↑ ↑ ↓ ← →
        // 典礼司仪
        ("唯一真旗", vec![DOWN, LEFT, RIGHT, RIGHT, UP], ""), // ↓ ← → → ↑
        // 法律之力
        ("电狗", vec![DOWN, UP, LEFT, UP, RIGHT, LEFT], ""), // ↓ ↑ ← ↑ → ←
        ("电磁榴弹发射器", vec![DOWN, RIGHT, UP, LEFT, RIGHT], ""), // ↓ → ↑ ← →
        // 变量控制
        ("哨戒激光炮", vec![DOWN, UP, RIGHT, DOWN, UP, RIGHT], ""), // ↓ ↑ → ↓ ↑ →
        ("传送背包", vec![DOWN, LEFT, RIGHT, DOWN, LEFT, RIGHT], ""), // ↓ ← → ↓ ← →
        ("纪元", vec![DOWN, LEFT, UP, LEFT, RIGHT], ""), // ↓ ← ↑ ← →
        // 尘卷风
        ("导弹发射井", vec![DOWN, UP, RIGHT, DOWN, DOWN], ""), // ↓ ↑ → ↓ ↓
        ("一次性凝固汽油弹", vec![DOWN, DOWN, LEFT, UP, LEFT], ""), // ↓ ↓ ← ↑ ←
        ("毒矛", vec![DOWN, RIGHT, DOWN, LEFT, UP, RIGHT], ""), // ↓ → ↓ ← ↑ →
        // Common
        ("增援", vec![UP, DOWN, RIGHT, LEFT, UP], "reinforce.wav"), // ↑ ↓ → ← ↑
        ("求救信标", vec![UP, DOWN, RIGHT, UP], ""), // ↑ ↓ → ↑
        ("补给", vec![DOWN, DOWN, UP, RIGHT], "resupply.wav"), // ↓ ↓ ↑ →
        ("飞鹰返航", vec![UP, UP, LEFT, UP, RIGHT], ""), // ↑ ↑ ← ↑ →
        // Objectives
        ("地狱火炸弹", vec![DOWN, UP, LEFT, DOWN, UP, RIGHT, DOWN, UP], ""), // ↓ ↑ ← ↓ ↑ → ↓ ↑
        ("超级地球大炮", vec![RIGHT, UP, UP, DOWN], ""), // → ↑ ↑ ↓
    ];
    let mut command_hashmap: HashMap<&'static str, Box<dyn Fn() + Send + Sync>> = HashMap::new();
    for (command, keys, audio_path) in command_map {
        let key_presser_ref = Arc::clone(&key_presser);
        let speaker_ref = Arc::clone(&speaker);
        command_hashmap.insert(command, Box::new(move || {
            key_presser_ref.push(keys.as_slice());

            if !audio_path.is_empty() {
                let audio_path = std::env::current_dir().unwrap().join("audio").join(audio_path);
                speaker_ref.play_wav(audio_path.to_str().unwrap()).unwrap();
            }
        }));
    }

    let command = Arc::new(Command::new(command_hashmap));
    let command_dic = command.keys().map(|x| x.to_string()).collect::<Vec<_>>();

    let config = AudioRecognizerConfig {
        chunk_time: 0.2, // 0.2 秒识别一次
        grammar,
        vad_silence_duration: 500,
    };
    let recognizer = AudioRecognizer::new(model_path.as_str(), config)?;
    let mut processor = AudioBufferProcessor::new(recognizer)?;

    let matcher = Arc::new(FuzzyMatcher::new(command_dic));

    let command_ref = Arc::clone(&command);
    let matcher_ref = Arc::clone(&matcher);
    let on_result = Box::new(move |result: RecognitionResult| {
        let speech = result.text.trim();
        if speech.is_empty() {
            return;
        }

        // 命中词
        let hit_word = "呼叫";
        if let Some(pos) = speech.rfind(hit_word) {
            let command_str = &speech[pos + hit_word.len()..];
            info!("speech: 呼叫 {}", &command_str);

            if let Some(command) = matcher_ref.match_str(&command_str) {
                info!("hit command: {}", command);
                command_ref.execute(command);
            }
        } else {
            warn!("miss required word '{}': {}", hit_word, speech);
        }
    });

    processor.start(on_result)?;
    // block
    key_presser.listen()?;

    Ok(())
}
