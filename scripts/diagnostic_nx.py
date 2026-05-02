import networkx as nx
from networkx.readwrite import json_graph
import json

# Let's see what the master thinks a Node-Link graph should look like
G_test = nx.path_graph(3)
G_test.nodes[0]['name'] = 'node0'
standard_data = json_graph.node_link_data(G_test)

print("--- NETWORKX STANDARD FORMAT ---")
print(json.dumps(standard_data, indent=2))

print("\n--- CHECKING OUR DATA ---")
try:
    with open('fastapi-symmetric.json', 'r', encoding='utf-8') as f:
        our_data = json.load(f)
    print(f"Our Keys: {list(our_data.keys())}")
    if "links" in our_data:
        print(f"Sample Link: {our_data['links'][0] if our_data['links'] else 'Empty'}")
    
    # Try to load
    G = json_graph.node_link_graph(our_data)
    print("✅ SUCCESS!")
except Exception as e:
    print(f"❌ FAILED: {e}")
