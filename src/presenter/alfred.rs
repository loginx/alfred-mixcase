use serde::{Deserialize, Serialize};
use serde_json;

/// Icon represents the icon object in Alfred JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub icon_type: Option<String>,
    pub path: String,
}

/// Text represents the text object in Alfred JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub largetype: Option<String>,
}

/// Item represents a single result item in Alfred JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
}

/// ScriptFilterOutput is the root object for Alfred Script Filter JSON output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptFilterOutput {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Item>,
}

impl ScriptFilterOutput {
    /// Creates a new ScriptFilterOutput with empty items.
    pub fn new() -> Self {
        ScriptFilterOutput {
            items: Vec::new(),
        }
    }

    /// Adds an item to the ScriptFilterOutput.
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    /// Converts the ScriptFilterOutput to Alfred-compatible JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl Default for ScriptFilterOutput {
    fn default() -> Self {
        Self::new()
    }
}
