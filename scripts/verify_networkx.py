import json
import sys

try:
    import networkx as nx
    from networkx.readwrite import json_graph
except ImportError:
    print("⚠️  NetworkX is not installed. Skipping physical validation.")
    sys.exit(0)

def verify_atlas(json_path):
    print(f"🔬  NetworkX: Final Validation of CCAP Atlas Export from {json_path}...")
    
    with open(json_path, 'r', encoding='utf-8') as f:
        data = json.load(f)

    try:
        # NOW: data natively contains 'nodes', 'edges', 'directed', 'multigraph'
        G = json_graph.node_link_graph(data)
        
        print(f"✅  Validation: SUCCESS")
        print(f"📊  Nodes Found: {G.number_of_nodes()}")
        print(f"📊  Edges Found: {G.number_of_edges()}")
        print(f"🧮  Graph Density: {nx.density(G):.6f}")
        
        # Test a complex graph algorithm to prove it's a real functional graph
        if G.number_of_nodes() > 0:
            pagerank = nx.pagerank(G, weight='weight')
            top_hubs = sorted(pagerank.items(), key=lambda x: x[1], reverse=True)[:3]
            print(f"🌟  Top 3 Hubs (by PageRank):")
            for node, score in top_hubs:
                print(f"    - {node} ({score:.4f})")
                
            print(f"\n✨  Conclusion: CCAP Gold is officially NetworkX-Native.")
            
    except Exception as e:
        print(f"❌  Validation: FAILED")
        print(f"⚠️   Error: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python verify_networkx.py <atlas_json>")
    else:
        verify_atlas(sys.argv[1])
