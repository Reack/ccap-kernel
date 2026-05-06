use crate::engine::extractor::FileFeatures;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Flavor {
    OpenAI, // Standard ST-AAAK
    Claude, // XML-scaffolded
    Gemini, // Long-context segmented
}

pub struct Mapper;

impl Mapper {
    pub fn to_telegram(path: &str, f: &FileFeatures) -> String {
        Self::to_flavor_telegram(path, f, &Flavor::OpenAI)
    }

    pub fn to_flavor_telegram(path: &str, f: &FileFeatures, flavor: &Flavor) -> String {
        let role = if f.symbol_count > 15 { "CORE" } else if f.symbol_count > 5 { "HUB" } else { "LEAF" };
        let b_type = if f.io_density_score > 0.5 { "IO_BRIDGE" } else if f.control_flow_score > 0.4 { "LOGIC_ENGINE" } else { "DATA_SCHEMA" };
        
        let c_rank = Self::rank(f.control_flow_score);
        let d_rank = Self::rank(f.data_density_score);
        let i_rank = Self::rank(f.io_density_score);
        let symbols = f.top_symbols.join("|").to_uppercase();

        let base_content = format!(
            "ROLE:{} TYPE:{} ACT:[{}] V:[c:{},d:{},i:{}]",
            role, b_type, symbols, c_rank, d_rank, i_rank
        );

        match flavor {
            Flavor::OpenAI => {
                format!("@NODE[{}] {}", path, base_content)
            },
            Flavor::Claude => {
                // 為 Claude 加入輕量級標籤，增強其推理鏈定位
                format!("<m id=\"{}\">{}</m>", path, base_content)
            },
            Flavor::Gemini => {
                // 為 Gemini 加入顯著的邊界符號，防止在巨型脈絡中發生語義融合
                format!(">>>>> NODE: {} <<<<<\n{}\n--------------------", path, base_content)
            }
        }
    }

    fn rank(score: f32) -> &'static str {
        if score > 0.8 { "+++" } else if score > 0.5 { "++" } else if score > 0.2 { "+" } else { "-" }
    }
}
