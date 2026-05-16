//! This module contains code to support autocomplete matching 
//! It can be used in front-end tools

/// This represents a match for an HPO term to a potentially partial text
/// that is being entered by a user 
/// 

pub mod autocompleter;


#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HpoMatch {
    /// HPO identifier of the matched concept, e.g., HP:0011995
    pub id: String,
    /// Corresponding HPO label, e.g., Atrial septal dilatation 
    pub label: String,
    /// Text that was matched, e.g., Atrial septal aneurysm
    pub matched_text: String,
}