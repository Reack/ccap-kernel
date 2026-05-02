import json
import sys

def convert_to_dot(json_path, dot_path):
    print(f"🌉  KG Bridge: Converting {json_path} to Graphviz DOT format...")
    
    with open(json_path, 'r', encoding='utf-8') as f:
        data = json.load(f)

    with open(dot_path, 'w', encoding='utf-8') as f:
        f.write('digraph CCAP_Atlas {\n')
        f.write('  rankdir=LR;\n')
        f.write('  node [shape=box, style=filled, color=lightblue, fontname="Helvetica"];\n')
        
        # Write Nodes
        for node in data['nodes']:
            node_id = node['id'].replace('\\', '/').replace('.', '_').replace('/', '_')
            label = f"{node['id']}\n[v:{node['features']['control_flow_score']:.1f}, {node['features']['data_density_score']:.1f}, {node['features']['io_density_score']:.1f}]"
            f.write(f'  "{node_id}" [label="{label}"];\n')
            
        # Write Edges
        for edge in data['edges']:
            source = edge['from'].replace('\\', '/').replace('.', '_').replace('/', '_')
            target = edge['to'].replace('\\', '/').replace('.', '_').replace('/', '_')
            f.write(f'  "{source}" -> "{target}" [label="{edge["weight"]:.2f}"];\n')
            
        f.write('}\n')
    
    print(f"✅  Conversion complete! Visual graph saved to: {dot_path}")
    print("💡  Tip: You can now view this with Graphviz or online tools like https://dreampuf.github.io/GraphvizOnline/")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python bridge_to_graphviz.py <atlas_json>")
    else:
        convert_to_dot(sys.argv[1], "graph_visualization.dot")
