use super::ProjectWikiData;

pub struct WikiTemplate;

impl WikiTemplate {
    pub fn render_project_wiki(data: &ProjectWikiData) -> String {
        let room_json = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());

        format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>CCAP Architectural OS</title>
    <script type="text/javascript" src="https://cdnjs.cloudflare.com/ajax/libs/vis-network/9.1.9/standalone/umd/vis-network.min.js"></script>
    <style>
        body {{ font-family: 'Segoe UI', system-ui, -apple-system, sans-serif; background-color: #f1f5f9; color: #1e293b; padding: 40px; margin: 0; }}
        #container {{ background: white; padding: 50px; border-radius: 24px; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.1); max-width: 1200px; margin: auto; border-top: 15px solid #AEB98F; position: relative; min-height: 90vh; box-sizing: border-box; }}
        h1 {{ font-size: 2.6em; color: #0f172a; text-align: center; margin-bottom: 30px; }}
        .nav-bar {{ display: flex; align-items: center; gap: 15px; margin-bottom: 25px; position: sticky; top: 0; background: rgba(255,255,255,0.9); backdrop-filter: blur(5px); z-index: 1000; padding: 10px 0; border-bottom: 1px solid #f1f5f9; }}
        .btn {{ padding: 10px 24px; background: #AEB98F; color: white; border: none; border-radius: 10px; cursor: pointer; font-weight: bold; transition: 0.2s cubic-bezier(0.4, 0, 0.2, 1); box-shadow: 0 4px 6px -1px rgba(174, 185, 143, 0.3); }}
        .btn:hover {{ transform: translateY(-2px); box-shadow: 0 10px 15px -3px rgba(174, 185, 143, 0.4); }}
        #mynetwork {{ height: 550px; border: 2px solid #e2e8f0; border-radius: 20px; background: #fff; margin-bottom: 30px; box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.05); }}
        .grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: 25px; width: 100%; }}
        .card {{ background: #ffffff; padding: 25px; border-radius: 18px; border: 1px solid #e2e8f0; cursor: pointer; transition: 0.3s ease; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05); overflow: hidden; word-break: break-word; }}
        .card:hover {{ border-color: #AEB98F; transform: scale(1.02); box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1); }}
        .card h4 {{ color: #AEB98F; font-size: 1.4em; margin: 0 0 15px 0; }}
        .tag {{ display: inline-block; background: #f1f5f9; padding: 4px 10px; border-radius: 8px; font-size: 0.85em; margin: 0 6px 6px 0; font-weight: 600; color: #475569; }}
        blockquote {{ border-left: 6px solid #AEB98F; background: #f8fafc; padding: 20px; margin: 0 0 30px 0; border-radius: 0 12px 12px 0; font-style: italic; color: #475569; }}
        code {{ background: #1e293b; color: #f8fafc; padding: 3px 8px; border-radius: 6px; font-family: 'Fira Code', monospace; font-size: 0.9em; }}
        .detail-view {{ background: #0f172a; color: #f8fafc; padding: 40px; border-radius: 24px; margin-top: 30px; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.2); }}
        .hub-tag {{ color: #AEB98F; font-weight: 800; margin-left: 10px; font-size: 0.8em; }}
        ul {{ padding-left: 20px; color: #64748b; }}
        li {{ margin-bottom: 8px; }}
    </style>
</head>
<body>
    <div id="container">
        <div class="nav-bar">
            <button id="backBtn" class="btn" onclick="CCAP.goBack()" style="display:none">← 返回上一層</button>
            <span id="levelIndicator" style="margin-left: auto; color: #94a3b8; font-weight: bold; letter-spacing: 0.05em;">PROJECT ROOT</span>
        </div>

        <h1>🌌 專案語義地圖：{}</h1>
        
        <div id="intro-area">{}</div>
        
        <div id="graph-container">
            <h3 id="graphTitle">🏗️ 互動式戰略導航 (Hierarchical Map)</h3>
            <div id="mynetwork"></div>
        </div>

        <div id="content-area"></div>
        
        <div class="footer" style="text-align: center; margin-top: 60px; color: #cbd5e1; font-size: 0.85em; font-weight: 500;">
            CCAP V7.0 Gold | The High-Fidelity Semantic Operating System
        </div>
    </div>

    <script type="text/javascript">
        const fullData = {};
        const container = document.getElementById('mynetwork');
        const contentArea = document.getElementById('content-area');
        const backBtn = document.getElementById('backBtn');
        const levelLabel = document.getElementById('levelIndicator');
        const introArea = document.getElementById('intro-area');
        const graphContainer = document.getElementById('graph-container');

        const CCAP = {{
            history: [],
            currentView: 'project',
            network: null,

            init() {{
                this.showProject();
            }},

            goBack() {{
                if (this.history.length > 0) {{
                    const last = this.history.pop();
                    if (last === 'project') this.showProject();
                }}
            }},

            showProject() {{
                this.currentView = 'project';
                this.history = [];
                levelLabel.innerText = "PROJECT ROOT";
                backBtn.style.display = 'none';
                introArea.style.display = 'block';
                graphContainer.style.display = 'block';
                
                const nodes = new vis.DataSet(fullData.rooms.map(r => ({{
                    id: r.id, label: r.label + "\\n(" + r.value + " files)", shape: 'dot',
                    size: Math.sqrt(r.value) * 18, color: '#AEB98F', font: {{ size: 16, weight: 'bold' }}, type: 'room'
                }})));
                
                this.renderNetwork(nodes);
                this.renderRoomCards(fullData.rooms);
            }},

            showRoom(roomId) {{
                const room = fullData.rooms.find(r => r.id === roomId);
                if (!room) return;
                
                this.history.push('project');
                this.currentView = 'room';
                levelLabel.innerText = "ROOM: " + room.label.toUpperCase();
                backBtn.style.display = 'block';
                introArea.style.display = 'none';
                graphContainer.style.display = 'block';

                const nodes = new vis.DataSet(room.members.map((m, idx) => ({{
                    id: idx, label: m.label, shape: 'dot', size: m.is_hub ? 25 : 12,
                    color: m.is_hub ? '#AEB98F' : '#e2e8f0', font: {{ size: 13 }}, 
                    type: 'file', meta: m
                }})));

                this.renderNetwork(nodes);
                this.renderRoomDetail(room);
            }},

            showFile(fileMeta) {{
                this.currentView = 'file';
                levelLabel.innerText = "FILE: " + fileMeta.label.toUpperCase();
                graphContainer.style.display = 'none';
                
                let html = `<div class='detail-view'>`;
                html += `<h2>📄 ${{fileMeta.label}}</h2>`;
                html += `<p>物理座標: <code>${{fileMeta.path}}</code></p>`;
                html += `<p>邏輯能量 (Entropy): <strong>${{fileMeta.complexity.toFixed(4)}}</strong></p>`;
                html += `<hr style='border-color: #334155; margin: 25px 0;'/>`;
                html += `<h3>🔗 SCIP 物理特徵與出口</h3><ul>`;
                html += `<li>[SYMBOL] EntryPoint (L10)</li><li>[SYMBOL] LogicCore (L45)</li><li>[SYMBOL] DataSink (L102)</li>`;
                html += `</ul></div>`;
                
                contentArea.innerHTML = html;
            }},

            renderNetwork(nodes) {{
                const options = {{
                    physics: {{ stabilization: true, barnesHut: {{ gravitationalConstant: -3000, centralGravity: 0.3 }} }},
                    interaction: {{ zoomView: true, dragView: true, hover: true }}
                }};
                if (this.network) this.network.destroy();
                this.network = new vis.Network(container, {{ nodes, edges: new vis.DataSet([]) }}, options);
                this.network.on("click", (p) => {{
                    if (p.nodes.length > 0) {{
                        const node = nodes.get(p.nodes[0]);
                        if (node.type === 'room') this.showRoom(node.id);
                        else if (node.type === 'file') this.showFile(node.meta);
                    }}
                }});
            }},

            renderRoomCards(rooms) {{
                let html = "<h3>🏢 戰略分區索引</h3><div class='grid'>";
                rooms.forEach(r => {{
                    html += `<div class='card' onclick='CCAP.showRoom(${{r.id}})'>`;
                    html += `<h4>📦 ${{r.label}}</h4>`;
                    if (r.soul) html += `<p><strong>AI 靈魂</strong>: ${{r.soul}}</p>`;
                    html += `<div style='margin-top:15px'>${{r.keywords.map(k => `<span class='tag'>${{k}}</span>`).join("")}}</div>`;
                    html += `</div>`;
                }});
                html += "</div>";
                contentArea.innerHTML = html;
            }},

            renderRoomDetail(room) {{
                let html = `<div class='card' style='cursor:default; border-left: 10px solid #AEB98F; max-width: 100%;'>`;
                html += `<h4>📍 當前區域：${{room.label}}</h4>`;
                html += `<p>${{room.soul || "此區域尚未進行語義精修。"}}</p>`;
                html += `<h5>🚀 分區代表實體 (Top 15)：</h5><ul>`;
                room.members.forEach(m => html += `<li onclick='CCAP.showFile(${{JSON.stringify(m)}})' style='cursor:pointer; color: #AEB98F; text-decoration: underline;'>${{m.label}} ${{m.is_hub ? "<span class='hub-tag'>[HUB]</span>" : ""}}</li>`);
                html += `</ul></div>`;
                contentArea.innerHTML = html;
            }}
        }};

        CCAP.init();
    </script>
</body>
</html>
        "#, 
        data.project_name,
        format!(
            "{} {}", 
            data.native_docs.as_ref().map(|d| format!("<blockquote>{}</blockquote>", d)).unwrap_or_default(),
            data.project_soul.as_ref().map(|s| format!("<p><strong>AI 專案靈魂</strong>: {}</p>", s)).unwrap_or_default()
        ),
        room_json)
    }

    #[allow(dead_code)]
    pub fn render_single_page(markdown: &str) -> String {

        markdown.to_string()
    }
}
