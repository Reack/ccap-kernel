use ccap_kernel::engine::wiki::{WikiProxy, ProjectWikiData, RoomData, MemberData};
use ccap_kernel::engine::extractor::FileFeatures;
use ccap_kernel::engine::glossary::Glossary;
use ccap_kernel::engine::Linker;

#[test]
fn test_ui_html_structure_mock() {
    let repo_root = "mock_repo";
    let results = vec![
        ("src/main.rs".to_string(), FileFeatures::default()),
    ];
    let linker = Linker::new();
    let glossary = Glossary::default();

    let html = WikiProxy::generate_html_wiki(repo_root, &results, &linker, &glossary).expect("HTML generation failed");

    // 1. Verify CSS Framework & Theme
    assert!(html.contains("#AEB98F"), "Theme color #AEB98F (Amber Green) must be present");
    assert!(html.contains("vis-network"), "Vis.js library must be included");

    // 2. Verify Hierarchical Logic placeholders
    assert!(html.contains("showProject()"), "showProject function must be in the script");
    assert!(html.contains("renderNetwork(nodes)"), "renderNetwork function must be in the script");
    assert!(html.contains("backBtn"), "Back button must exist for navigation");

    // 3. Verify Data Injection
    assert!(html.contains("const fullData ="), "Data injection variable must exist");
}

#[test]
fn test_ui_mermaid_syntax_validity() {
    let repo_root = "mermaid_test_repo";
    // Mock room data with potential problematic characters
    let data = ProjectWikiData {
        project_name: "MermaidTest".to_string(),
        rooms: vec![
            RoomData {
                id: 1,
                label: "Core_Engine".to_string(),
                soul: "The 'Heart' of CCAP".to_string(), // Contains quotes
                members: vec![
                    MemberData { path: "src/lib.rs".to_string(), label: "lib".to_string(), is_hub: true, complexity: 0.9 }
                ],
                ..Default::default()
            },
            RoomData {
                id: 2,
                label: "IO_Bridge".to_string(),
                soul: "Data / Logic Connector".to_string(), // Contains slashes
                ..Default::default()
            }
        ],
        ..Default::default()
    };

    let html = ccap_kernel::engine::wiki::WikiTemplate::render_project_wiki(&data);

    // 1. Verify Mermaid Block Start
    assert!(html.contains("graph TD"), "Mermaid graph definition missing");

    // 2. Verify Node Sanitization (Mermaid doesn't like quotes/special chars in node labels directly)
    // Expectation: Labels should be escaped or wrapped in quotes properly by the engine
    assert!(html.contains("[\"Core_Engine\"]") || html.contains("[Core_Engine]"), "Node Core_Engine not found");
    
    // 3. Verify Edge Connection
    // Assuming the engine connects rooms based on some logic (e.g., sequentially or via dependencies)
    // If the engine generates subgraphs or links, verify the syntax
    assert!(html.contains("-->"), "Mermaid edge connection missing");
}

#[test]
fn test_ui_data_completeness() {
    let data = ProjectWikiData {
        project_name: "MockUI".to_string(),
        total_files: 5,
        rooms: vec![
            RoomData {
                id: 1,
                label: "UI_ROOM".to_string(),
                value: 10,
                keywords: vec!["frontend".to_string()],
                soul: "Interaction layer".to_string(),
                members: vec![
                    MemberData { path: "view.rs".to_string(), label: "view".to_string(), is_hub: true, complexity: 0.8 }
                ]
            }
        ],
        ..Default::default()
    };

    let html = ccap_kernel::engine::wiki::WikiTemplate::render_project_wiki(&data);
    
    assert!(html.contains("MockUI"), "Project name not found in HTML");
    assert!(html.contains("UI_ROOM"), "Room label not found in HTML");
    assert!(html.contains("Interaction layer"), "Room soul not found in HTML");
    assert!(html.contains("view.rs"), "Member path not found in HTML");
}
