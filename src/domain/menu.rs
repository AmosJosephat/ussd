use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Menu item definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    /// Menu identifier
    pub id: String,

    /// Menu title/message
    pub title: HashMap<String, String>, // language -> text

    /// Menu options
    pub options: Vec<MenuOption>,

    /// Menu type
    pub menu_type: MenuType,

    /// Handler plugin name
    pub handler: Option<String>,

    /// Parent menu ID
    pub parent: Option<String>,

    /// Validation rules
    pub validation: Option<ValidationRules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuOption {
    /// Option key (what user types)
    pub key: String,

    /// Option label
    pub label: HashMap<String, String>, // language -> text

    /// Next menu/state
    pub next_state: String,

    /// Action to execute
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MenuType {
    /// Display menu with options
    Menu,

    /// Free text input
    Input,

    /// Final response (end session)
    Response,

    /// Dynamic menu (loaded by plugin)
    Dynamic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Input pattern (regex)
    pub pattern: Option<String>,

    /// Minimum length
    pub min_length: Option<usize>,

    /// Maximum length
    pub max_length: Option<usize>,

    /// Required field
    pub required: bool,

    /// Custom validator plugin
    pub validator: Option<String>,
}

impl MenuItem {
    pub fn get_title(&self, language: &str) -> String {
        self.title
            .get(language)
            .or_else(|| self.title.get("en"))
            .cloned()
            .unwrap_or_else(|| "Menu".to_string())
    }

    pub fn get_option(&self, key: &str) -> Option<&MenuOption> {
        self.options.iter().find(|opt| opt.key == key)
    }

    pub fn format_menu(&self, language: &str) -> String {
        let title = self.get_title(language);

        match self.menu_type {
            MenuType::Menu => {
                let mut output = format!("{}\n", title);
                for option in &self.options {
                    let label = option
                        .label
                        .get(language)
                        .or_else(|| option.label.get("en"))
                        .cloned()
                        .unwrap_or_default();
                    output.push_str(&format!("{}. {}\n", option.key, label));
                }
                output
            }
            MenuType::Input => title,
            MenuType::Response => title,
            MenuType::Dynamic => title,
        }
    }
}
