use crate::engine::extractor::FileFeatures;

pub struct Mapper;

impl Mapper {
    pub fn to_telegram(path: &str, f: &FileFeatures) -> String {
        // 1. Determine ROLE
        let role = if f.symbol_count > 15 {
            "CORE"
        } else if f.symbol_count > 5 {
            "HUB"
        } else {
            "LEAF"
        };

        // 2. Determine TYPE (The logic engine vs I/O bridge vs Data store)
        let b_type = if f.io_density_score > 0.5 {
            "IO_BRIDGE"
        } else if f.control_flow_score > 0.4 {
            "LOGIC_ENGINE"
        } else {
            "DATA_SCHEMA"
        };

        // 3. Quantize scores to symbols for AI (e.g., +, ++, +++)
        let c_rank = Self::rank(f.control_flow_score);
        let d_rank = Self::rank(f.data_density_score);
        let i_rank = Self::rank(f.io_density_score);

        // 4. Extract symbols string
        let symbols = f.top_symbols.join("|").to_uppercase();

        // 5. Build the final Telegram string
        format!(
            "@NODE[{}] ROLE:{} TYPE:{} ACT:[{}] V:[c:{},d:{},i:{}]",
            path, role, b_type, symbols, c_rank, d_rank, i_rank
        )
    }

    fn rank(score: f32) -> &'static str {
        if score > 0.8 { "+++" }
        else if score > 0.5 { "++" }
        else if score > 0.2 { "+" }
        else { "-" }
    }
}
