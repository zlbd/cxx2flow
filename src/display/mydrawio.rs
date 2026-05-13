use crate::error::{Error, Result};
use crate::graph::{Graph, GraphNodeType};
use petgraph::{
    visit::IntoNodeReferences,
    visit::{EdgeRef, IntoEdgeReferences},
};

use super::GraphDisplay;
#[derive(Debug, Default)]
pub struct DrawIO {}

impl DrawIO {
    pub fn new() -> Self {
        DrawIO {}
    }
}

impl GraphDisplay for DrawIO {
    fn generate_from_graph(&self, graph: &Graph) -> Result<String> {
        let mut res = r#"<?xml version="1.0" encoding="UTF-8"?>
<mxfile host="app.diagrams.net">
    <diagram name="Page-1">
    <mxGraphModel dx="1012" dy="532" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="827" pageHeight="1169" math="0" shadow="0">
        <root>
        <mxCell id="0" />
        <mxCell id="1" parent="0" />
"#
        .to_string();

        // 生成节点
        for (id, node) in graph.node_references() {
            let node_id = format!("node_{}", id.index());
            let style = match node {
                GraphNodeType::Begin | GraphNodeType::End => "rectangle;rounded=1;whiteSpace=wrap;html=1;aspect=fixed;",
                GraphNodeType::Node(_) => "rounded=0;whiteSpace=wrap;html=1;",
                GraphNodeType::Choice(_) => "rhombus;whiteSpace=wrap;html=1;aspect=fixed;",
                GraphNodeType::Dummy => {
                    return Err(Error::UnexpectedDummyGraphNode {
                        graph: graph.clone(),
                    })
                }
            };

            let label = match node {
                GraphNodeType::Begin => "Begin".to_string(),
                GraphNodeType::End => "End".to_string(),
                GraphNodeType::Node(s) | GraphNodeType::Choice(s) => escape_xml(s),
                _ => unreachable!(),
            };

            res.push_str(&format!(
                r#"        <mxCell id="{}" value="{}" style="{}" parent="1" vertex="1">
            <mxGeometry x="0" y="0" width="120" height="60" as="geometry"/>
        </mxCell>
"#,
                node_id, label, style
            ));
        }

        // 生成边
        for (index, edge) in graph.edge_references().enumerate() {
            let edge_id = format!("edge_{}", index);
            let source_id = format!("node_{}", edge.source().index());
            let target_id = format!("node_{}", edge.target().index());

            let (label, style) = match edge.weight() {
                crate::graph::EdgeType::Normal => ("".to_string(), "".to_string()),
                crate::graph::EdgeType::Branch(true) => ("Y".to_string(), "label=&lt;Y&gt;;".to_string()),
                crate::graph::EdgeType::Branch(false) => ("N".to_string(), "label=&lt;N&gt;;".to_string()),
            };

            res.push_str(&format!(
                r#"        <mxCell id="{}" value="{}" style="edgeStyle=orthogonalEdgeStyle;rounded=0;html=1;exitX=0.5;exitY=1;entryX=0.5;entryY=0;{}" parent="1" source="{}" target="{}" edge="1">
            <mxGeometry relative="1" as="geometry"/>
        </mxCell>
"#,
                edge_id, label, style, source_id, target_id
            ));
        }

        res.push_str(
            r#"      </root>
    </mxGraphModel>
    </diagram>
</mxfile>"#,
        );

        Ok(res)
    }
}


// XML特殊字符转义
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}