use ccap_kernel::engine::Linker;
use ccap_kernel::engine::extractor::FileFeatures;

#[test]
fn test_dependency_graph_building_integrity() {
    let mut linker = Linker::new();
    let mut results = Vec::new();

    // A -> B -> C
    let mut f_a = FileFeatures::default();
    f_a.references.push("ccap . . b#Symbol#".to_string());
    results.push(("b.rs".to_string(), FileFeatures::default()));
    results.push(("a.rs".to_string(), f_a));

    linker.build_graph(&results);
    
    let edges = linker.export_edges();
    assert_eq!(edges.len(), 1, "Should have exactly one edge A -> B");
}

#[test]
fn test_dependency_impact_blast_radius() {
    let mut linker = Linker::new();
    let mut results = Vec::new();

    // D -> C -> B -> A
    let mut f_d = FileFeatures::default(); f_d.references.push("ccap . . c#S#".to_string());
    let mut f_c = FileFeatures::default(); f_c.references.push("ccap . . b#S#".to_string());
    let mut f_b = FileFeatures::default(); f_b.references.push("ccap . . a#S#".to_string());
    
    results.push(("a.rs".to_string(), FileFeatures::default()));
    results.push(("b.rs".to_string(), f_b));
    results.push(("c.rs".to_string(), f_c));
    results.push(("d.rs".to_string(), f_d));

    linker.build_graph(&results);

    // Impact of A should reach B, C, D (Incoming edges to A from B, etc)
    // Wait, linker.rs uses Incoming for dependents. 
    // If B references A, edge is B -> A. Neighbors(A, Incoming) is B.
    let report = linker.calculate_impact("a.rs").expect("Report should exist");
    
    println!("💥 Blast Radius of a.rs: radius={}, dependents={:?}", report.radius, report.dependents);
    
    assert_eq!(report.radius, 3, "Blast radius should be 3 layers (B, C, D)");
    assert!(report.dependents.contains(&"b.rs".to_string()));
    assert!(report.dependents.contains(&"c.rs".to_string()));
    assert!(report.dependents.contains(&"d.rs".to_string()));
}

#[test]
fn test_dependency_fuzzy_matching() {
    let mut linker = Linker::new();
    let mut results = Vec::new();

    // X references Y (without extension)
    let mut f_x = FileFeatures::default();
    f_x.references.push("ccap . . y#Symbol#".to_string());
    
    results.push(("y.py".to_string(), FileFeatures::default()));
    results.push(("x.py".to_string(), f_x));

    linker.build_graph(&results);
    
    let edges = linker.export_edges();
    assert_eq!(edges.len(), 1, "Fuzzy matching should link x.py to y.py");
}
