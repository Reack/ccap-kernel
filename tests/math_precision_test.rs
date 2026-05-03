use ccap_kernel::engine::MathEngine;
use std::collections::HashMap;

#[test]
fn test_math_laplacian_fiedler_determinism() {
    // Test Case: Star Graph (Center 0 connected to 1, 2, 3)
    // Laplacian should have an eigenvalue of 1.0 (multiplicity n-2)
    let n = 4;
    let edges = vec![(0, 1, 1.0), (0, 2, 1.0), (0, 3, 1.0)];
    
    let fiedler = MathEngine::compute_fiedler_value(n, &edges);
    
    // For a star graph with weight 1, the second smallest eigenvalue (Fiedler) is 1.0
    println!("⭐ Star Graph Fiedler Value: {:.4}", fiedler);
    assert!((fiedler - 1.0).abs() < 1e-5, "Fiedler value of star graph must be 1.0");
}

#[test]
fn test_math_clustering_boundary_robustness() {
    let paths = vec!["a.rs".to_string(), "b.rs".to_string()];
    let sym_data = HashMap::new();
    
    // Case 1: n < k (Number of files less than requested clusters)
    // The engine should handle this gracefully without crashing
    let clusters = MathEngine::spectral_cluster(&paths, &[], 5, &sym_data).expect("Should handle n < k");
    assert!(clusters.len() <= paths.len());
    
    // Case 2: Empty input
    let empty_paths: Vec<String> = vec![];
    let clusters = MathEngine::spectral_cluster(&empty_paths, &[], 2, &sym_data).unwrap();
    assert_eq!(clusters.len(), 0);
}

#[test]
fn test_math_eigenvalue_ordering() {
    // 0 -- 1 (Path graph n=2)
    // Laplacian = [1 -1; -1 1]
    // Eigenvalues: 0, 2
    let n = 2;
    let edges = vec![(0, 1, 1.0)];
    let fiedler = MathEngine::compute_fiedler_value(n, &edges);
    
    assert!((fiedler - 2.0).abs() < 1e-5, "Fiedler value of n=2 path graph must be 2.0");
}
