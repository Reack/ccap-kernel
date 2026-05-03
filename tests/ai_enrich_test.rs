use ccap_kernel::engine::wiki::WikiGenerator;
use ccap_kernel::engine::extractor::{FileFeatures, ScipSymbol};
use tiktoken_rs::cl100k_base;

#[test]
fn test_ai_enrich_prompt_fidelity() {
    let mut features = FileFeatures::default();
    features.control_flow_score = 0.8;
    features.io_density_score = 0.9;
    features.symbol_count = 15;
    features.top_symbols = vec!["SecurityEngine".to_string(), "encrypt".to_string()];
    
    features.exports.push(ScipSymbol {
        id: "ccap . . security#SecurityEngine#".to_string(),
        name: "SecurityEngine".to_string(),
        line: 10,
        ..Default::default()
    });

    let target_path = "src/security.rs";
    let prompt = WikiGenerator::generate_ai_enrich_prompt(target_path, &features);

    // 1. Verify Prompt Structure
    assert!(prompt.contains("[CCAP AEP]"), "Prompt must have AEP tag");
    assert!(prompt.contains("src/security.rs"), "Prompt must contain target path");

    // 2. Verify Semantic Telegram (ST-AAAK) Fidelity
    // Check if critical roles are inferred correctly based on features
    assert!(prompt.contains("ROLE:HUB"), "Symbol count 15 should trigger HUB role (>5)");
    assert!(prompt.contains("TYPE:IO_BRIDGE"), "High IO score (>0.5) should trigger IO_BRIDGE type");
    assert!(prompt.contains("SECURITYENGINE"), "Top symbols must be present in ACT section (UPPERCASE)");
    assert!(prompt.contains("ENCRYPT"), "Top symbols must be present in ACT section (UPPERCASE)");
}

#[test]
fn test_token_economics_compression() {
    // Mock a medium-sized source file (approx 100 lines / 2k tokens)
    let raw_code = "
        use anyhow::Result;
        pub struct Storage { path: String }
        impl Storage {
            pub fn new(path: &str) -> Self { Self { path: path.to_string() } }
            pub fn save(&self, data: &[u8]) -> Result<()> {
                // ... complex logic ...
                std::fs::write(&self.path, data)?;
                Ok(())
            }
            pub fn load(&self) -> Result<Vec<u8>> {
                let bytes = std::fs::read(&self.path)?;
                Ok(bytes)
            }
        }
    ".repeat(20); // Repeat to simulate a real file

    let mut features = FileFeatures::default();
    features.symbol_count = 5;
    features.top_symbols = vec!["Storage".to_string(), "save".to_string(), "load".to_string()];
    features.io_density_score = 1.0;
    
    let target_path = "src/storage.rs";
    let telegram = WikiGenerator::generate_ai_enrich_prompt(target_path, &features);

    let bpe = cl100k_base().unwrap();
    let raw_tokens = bpe.encode_with_special_tokens(&raw_code).len();
    let telegram_tokens = bpe.encode_with_special_tokens(&telegram).len();

    let compression_ratio = raw_tokens as f32 / telegram_tokens as f32;
    
    println!("📊 Economics: Raw={} tokens, Telegram={} tokens, Ratio={:.2}x", raw_tokens, telegram_tokens, compression_ratio);
    
    assert!(compression_ratio > 10.0, "Compression ratio must be at least 10x for 1M+ scaling");
}

#[test]
fn test_global_synthesis_package_format() {
    let results = vec![
        ("src/main.rs".to_string(), FileFeatures::default()),
        ("src/lib.rs".to_string(), FileFeatures::default()),
    ];

    let package = WikiGenerator::generate_global_ai_package(&results);

    assert!(package.contains("[CCAP GLOBAL SYNTHESIS]"), "Package must have global tag");
    assert!(package.contains("src/main.rs"), "Package must contain all files");
    assert!(package.contains("src/lib.rs"), "Package must contain all files");
    assert!(package.lines().count() >= 3, "Package should have multiple lines of telegrams");
}

#[test]
fn test_ai_semantic_alignment_mock() {
    let mut features = FileFeatures::default();
    features.top_symbols = vec!["SecurityEngine".to_string(), "AES256".to_string()];
    features.symbol_count = 20; // This triggers ROLE:CORE (>15)
    features.control_flow_score = 0.9; // Logic heavy
    features.io_density_score = 0.1;

    let target_path = "src/security.rs";
    let telegram = ccap_kernel::engine::Mapper::to_telegram(target_path, &features);

    // Mock AI Response (The 'story' AI generates based on the telegram)
    let ai_response = "
        # Module Analysis: src/security.rs
        This is a **CORE** component of the architecture.
        It implements the **SecurityEngine** which handles cryptographic operations.
        It also references **AES256** standards for high-security data protection.
    ";

    // 1. Verify Role Alignment
    assert!(telegram.contains("ROLE:CORE"), "Telegram must be marked as CORE");
    assert!(ai_response.to_uppercase().contains("CORE"), "AI story must reflect the CORE role");

    // 2. Verify Symbol Alignment (ACT)
    assert!(telegram.contains("SECURITYENGINE"), "Telegram must contain the symbol");
    assert!(telegram.contains("AES256"), "Telegram must contain the symbol");
    assert!(ai_response.to_uppercase().contains("SECURITYENGINE"), "AI story must mention SecurityEngine");
    assert!(ai_response.to_uppercase().contains("AES256"), "AI story must mention AES256");

    // 3. Verify Type Inference Alignment
    assert!(telegram.contains("TYPE:LOGIC_ENGINE"), "High control flow should trigger LOGIC_ENGINE");
    assert!(ai_response.to_lowercase().contains("implement") || ai_response.to_lowercase().contains("logic"), "AI story should imply logic implementation");
}
