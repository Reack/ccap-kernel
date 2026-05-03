use ccap_kernel::engine::Mapper;
use ccap_kernel::engine::extractor::{FileFeatures, ScipSymbol};

#[test]
fn test_st_aaak_protocol_fidelity_mapping() {
    let mut features = FileFeatures::default();
    features.symbol_count = 20;
    features.control_flow_score = 0.9;
    features.data_density_score = 0.3;
    features.io_density_score = 0.1;
    features.top_symbols = vec!["SecurityEngine".to_string(), "AES256".to_string()];
    
    let path = "src/security.rs";
    let telegram = Mapper::to_telegram(path, &features);

    // 1. Verify Node Path
    assert!(telegram.contains("@NODE[src/security.rs]"), "Telegram must identify the correct node");

    // 2. Verify Role & Type (Strategic alignment)
    assert!(telegram.contains("ROLE:CORE"), "Symbol count 20 should be CORE");
    assert!(telegram.contains("TYPE:LOGIC_ENGINE"), "High control flow (0.9) should be LOGIC_ENGINE");

    // 3. Verify Vector Quantization (Math precision)
    assert!(telegram.contains("V:[c:+++,d:+,i:-]"), "Vector ranks must match the scores");

    // 4. Verify Symbols (ACT segment)
    assert!(telegram.contains("ACT:[SECURITYENGINE|AES256]"), "Top symbols must be capitalized and piped");
}

#[test]
fn test_protocol_to_patch_linkage() {
    // This test simulates the 'Holy Grail' of CCAP: 
    // AI reads a telegram -> Picks a symbol -> We map it to physical coordinates for patching.
    
    let mut features = FileFeatures::default();
    let sym_id = "ccap . . security#SecurityEngine#";
    features.exports.push(ScipSymbol {
        id: sym_id.to_string(),
        name: "SecurityEngine".to_string(),
        range: (100, 500), // Byte coordinates
        ..Default::default()
    });
    features.top_symbols = vec!["SecurityEngine".to_string()];

    let telegram = Mapper::to_telegram("src/security.rs", &features);
    
    // AI response would say: "I want to patch SecurityEngine"
    let ai_intent_symbol = "SECURITYENGINE";
    
    // Find the symbol in features by name (case-insensitive as per ACT protocol)
    let matched_symbol = features.exports.iter()
        .find(|s| s.name.to_uppercase() == ai_intent_symbol)
        .expect("Symbol mapping failed");

    assert_eq!(matched_symbol.id, sym_id);
    assert_eq!(matched_symbol.range, (100, 500));
    
    // Verify that the telegram actually contained enough info to prompt the AI
    assert!(telegram.contains(ai_intent_symbol), "Telegram must contain the symbol name for AI discovery");
}
