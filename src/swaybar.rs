use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
pub struct Block {
    pub full_text: String,
    pub short_text: Option<String>,
    pub color: Option<String>,
    pub background: Option<String>,
    pub border: Option<String>,
    pub border_top: Option<usize>,
    pub border_right: Option<usize>,
    pub border_bottom: Option<usize>,
    pub border_left: Option<usize>,
    pub min_width: Option<BlockMinWidth>,
    pub align: Option<BlockAlign>,
    pub name: Option<String>,
    pub instance: Option<String>,
    pub urgent: Option<bool>,
    pub separator: Option<bool>,
    pub separator_block_width: Option<usize>,
    pub markup: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockAlign {
    Center,
    Right,
    Left,
}

#[allow(unused)]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BlockMinWidth {
    Pixels(usize),
    Text(String),
}
