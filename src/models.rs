use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::error::GdlError;
use crate::utils;
use std::ffi::OsStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    nodes: Vec<Node>,
}

impl Dialogue {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
        }
    }

    pub fn add_new_node(&mut self, node: Node) -> Result<(), GdlError> {
        self.nodes.push(node);
        Ok(())
    }

    pub fn new_file(path: &Path, format: Format) -> Result<(), GdlError> {
        let dialogue = Dialogue::deserialize(path)?;
        let dot_script = dialogue.create_dot_script("Test")?;

        if cfg!(debug_assertions) {
            std::fs::write(&std::env::current_dir()?.join("out.gv"), dot_script.as_bytes())?;
        }

        dialogue.render(format, &dot_script)?;

        Ok(())
    }

    pub fn serialize(&self, path: &Path) -> Result<(), GdlError> {
        let rif_config = serde_json::to_string_pretty(self)?;
        std::fs::write(path, rif_config)?;

        Ok(())
    }

    pub fn deserialize(path : &Path) -> Result<Self, GdlError> {
        Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    }

    // TODO
    fn create_dot_script(&self, name: &str) -> Result<String, GdlError> {
        let mut dot_script = format!("digraph {0} {{
", name);
        let global_attr = r#"    node [shape="record"]
"#;
        dot_script.push_str(global_attr);

        for node in &self.nodes {
            self.check_node_validation(node)?;
            // id|type|
            let mut label = format!("{{{0}|{1}}}|",node.node_type, node.id);
            let mut edges = String::new();
            let mut style: &str = "";
			match node.node_type {
				NodeType::Text => {
                    // id|type|speaker|
                    label.push_str(&format!("{0}|", node.speaker.as_ref().unwrap()));
                    if let Some(goto) = &node.goto {
                        edges = format!(r#"    {0} -> {1}
"#,node.id,goto);
                    }
                    style = "";
                }
				NodeType::Selection => {
                    if let Some(speaker) = &node.speaker {
                        // {id|type}|speaker|
                        label.push_str(&format!("{0}|", speaker));
                    }
                    style = r#"colorfill="white" color="green3";"#;
                    for sel in node.selections.as_ref().unwrap() {
                        edges.push_str(&format!(r#"    {0} -> {1}
"#, node.id, sel.goto));
                    }
                }
				NodeType::Branch => {
                    style = r#"colorfill="white" color="dodgerblue3";"#;
                    for div in node.branches.as_ref().unwrap() {
                        edges.push_str(&format!(r#"    {0} -> {1}
"#, node.id, div.goto));
                    }
                }
                NodeType::Start => {
                    label = format!("{0}|",node.text);
                        edges.push_str(&format!(r#"    start -> {0}
"#, node.goto.as_ref().unwrap()));
                }
                NodeType::End => {
                    label = format!("{0}|",node.text);
                }
                NodeType::Omit => {
                    label = format!("{{Omit|{0}}}|",node.id);
                    style = r#"colorfill="white" color="gray";"#;
                    edges.push_str(&format!(r#"    {0} -> {1}
"#, node.id,node.goto.as_ref().unwrap()));
                }
			}

            // r#"{node.id} [id="{node.id}" label="{{label.slice(0,-1)}}" {style}]\n"#, node.id, label ,style
			dot_script.push_str(&format!(r#"    {0} [id="{0}" label="{{{1}}}" {2}]
"#, node.id, &label[0..label.len() - 1],style));
			dot_script.push_str(&edges);

        }
        dot_script.push_str("}");

        Ok(dot_script)
    }

    fn check_node_validation(&self, node: &Node) -> Result<(), GdlError> {
        match node.node_type {
            NodeType::Text => {
                if let None = node.speaker { return Err(GdlError::InvalidNodeContent("Text node requires speaker")); }
            }
            NodeType::Selection => {
                if let None = node.selections { return Err(GdlError::InvalidNodeContent("Selection node requires selections array")); }
            }
            NodeType::Branch => {
                if let None = node.branches { return Err(GdlError::InvalidNodeContent("Branch node requires branches array")); }
            }
            NodeType::Start => {
                if let None = node.goto { return Err(GdlError::InvalidNodeContent("Start node requires goto id")); }
            }
            NodeType::Omit => {
                if let None = node.goto { return Err(GdlError::InvalidNodeContent("Omit node requires goto id")); }
            }
            NodeType::End => ()
        }
        Ok(())
    }

    fn render(&self,format : Format, dot_script: &str) -> Result<(), GdlError> {
        let args: Vec<&OsStr>;
        let out_file;
        match format {
            Format::Pdf => {
                out_file = std::env::current_dir()?.join("out.pdf");
                args = vec![
                    OsStr::new("-Tpdf"),
                    OsStr::new("-o"),
                    out_file.as_os_str()
                ];
            }
            Format::Png => {
                out_file = std::env::current_dir()?.join("out.png");
                args = vec![
                    OsStr::new("-Tpng"),
                    OsStr::new("-Gdpi=300"),
                    OsStr::new("-o"),
                    out_file.as_os_str()
                ];
            }
        }
        utils::dot_exec(args, dot_script)?;
        Ok(())
    }
}

pub enum Format {
    Pdf,
    Png,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    // Global attribute
    id: String,
    node_type: NodeType,
    goto: Option<String>,
    text: String,

    // Comes with Text and selection Node
    speaker: Option<String>,
    // Comes with selection node
    selections: Option<Vec<Selection>>,
    // Comes with branch node
    branches: Option<Vec<Branch>>,
}

impl Node {
    pub fn omit_node(id: &str, goto: &str) -> Self {
        Self {
            id: id.to_owned(),
            node_type: NodeType::Omit,
            text: "".to_owned(),
            speaker : None,
            goto : Some(goto.to_owned()),
            selections : None,
            branches: None,
        }
    }
    pub fn start_node(text: &str, goto: &str) -> Self {
        Self {
            id: String::from("start"),
            node_type: NodeType::Start,
            text: text.to_owned(),
            speaker : None,
            goto : Some(goto.to_owned()),
            selections : None,
            branches: None,
        }
    }

    pub fn end_node(text: &str) -> Self {
        Self {
            id: String::from("end"),
            node_type: NodeType::End,
            text: text.to_owned(),
            speaker : None,
            goto : None,
            selections : None,
            branches: None,
        }
    }

    pub fn text_node(id: &str, speaker: &str, text: &str, goto: Option<&str>) -> Self {
        let id = id.to_owned();
        let speaker = speaker.to_owned();
        let text = text.to_owned();
        let goto = goto.map(|s| s.to_owned());
        Self {
            id,
            node_type: NodeType::Text,
            text,
            speaker : Some(speaker),
            goto,
            selections : None,
            branches: None,
        }
    }
    pub fn selection_node(id: &str, speaker: Option<&str>, text: &str, goto: Option<&str>, selections: Vec<Selection>) -> Self {
        let id = id.to_owned();
        let speaker = speaker.map(|s| s.to_owned());
        let text = text.to_owned();
        let goto = goto.map(|s| s.to_owned());
        Self {
            id,
            node_type: NodeType::Selection,
            text,
            speaker,
            goto,
            selections : Some(selections),
            branches: None,
        }
    }

    pub fn branch_node(id: &str, goto: Option<&str>, branches: Vec<Branch>) -> Self {
        let id = id.to_owned();
        let goto = goto.map(|s| s.to_owned());
        Self {
            id,
            text: String::new(),
            node_type: NodeType::Branch,
            speaker : None,
            goto,
            selections : None,
            branches: Some(branches),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum NodeType {
    Start,
    Text,
    Branch,
    Selection,
    Omit,
    End,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: &str;

        match self {
            Self::Text => text = "Text",
            Self::Branch => text = "Branch",
            Self::Selection => text = "Selection",
            Self::Start => text = "Start",
            Self::End => text = "End",
            Self::Omit => text = "Omit",
        }
        write!(f,"{0}",text)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Selection {
    text: String,
    goto: String,
}
impl Selection {
    pub fn new( text: &str, goto: &str) -> Self {
        Self { 
            text: text.to_owned(),
            goto: goto.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    target: String,
    qual: String,
    goto: String,
} 

impl Branch {
    pub fn new(target: &str, qual: &str, goto: &str) -> Self {
        Self {
            target: target.to_owned(),
            qual: qual.to_owned(),
            goto: goto.to_owned(),
        }
    }
}
