//! # SimpleHpoParser Module
//!
//! This module defines the `SimpleHpoParser` struct, which parses the hp.json file using serde
//! For speed and simplicity, we do not parse in all of the data but instead leverage user-defined functions
//! to extract the required information.
//!
//! ## Example
//!
//! ```ignore
//! use ontolius::prelude::TermId;
//! use std::collections::HashMap;
//! use ferriphene::hpo::clinical_mapper::ClinicalMapper;
//! use ferriphene::hpo::simple_hpo_parser::SimpleHpoParser;
//! use ferriphene::fenominal_traits::TermIdToTextMapper;
//! let simple_parser = SimpleHpoParser::new("/path/to/hp.json");
//! let t2tmap: HashMap<String, TermId> = simple_parser.get_text_to_term_map();
//! let mut clinical_mapper = ClinicalMapper::from_map(&t2tmap);
//! ```


use ontolius::prelude::TermId;
use std::{collections::{HashMap, HashSet}, fs, path::PathBuf, str::FromStr};
use crate::fenominal_traits::TermIdToTextMapper;

/// Transform a URI such as "http://purl.obolibrary.org/obo/HP_6000699" into a CURIE HP:6000699
fn get_hp_id<T: AsRef<str>>(uri: T) -> String {
    uri.as_ref()
        .rsplit('/')
        .next()
        .expect("Could not get last field of uri split")
        .replace('_', ":")
}

/// Simple structure for holding id, label, and synonyms of a Node from the hp.json file
/// 
/// This struct is needed internally only
#[derive(Debug)]
struct HpNode {
    term_id: String,
    term_label: String,
    synonyms: Vec<String>
}


impl HpNode {
    pub fn new<T: Into<String>, V: Into<String>>(term_id: T, lbl: V, syns: Vec<String>) -> Self {
        HpNode {
            term_id: term_id.into(),
            term_label: lbl.into(),
            synonyms: syns
        }
    }
}




/// Simple structure for holding source/dest from the hp.json file
/// 
/// This struct is needed internally only
struct HpEdge {
    source: String,
    destination: String,
}

impl HpEdge {
    pub fn new(s:String, d:String) -> Self {
        HpEdge{
            source: get_hp_id(s),
            destination: get_hp_id(d),
        }
    }
}

impl std::fmt::Debug for HpEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Edge: ")
         .field("source", &self.source)
         .field("destination", &self.destination)
         .finish()
    }
}

/// Simple ontology of Nodes and Edges
/// 
/// This is used internally only. The main purpose is to filter for Nodes that descend from Phenotypic abnormality
#[derive(Debug)]
struct SimpleOntology {
    nodes: Vec<HpNode>,
    edges: Vec<HpEdge>,
    version: String
}

impl SimpleOntology {
    pub fn new<T: Into<String>>(e: Vec<HpEdge>, n: Vec<HpNode>, ver: T) -> Self {
        SimpleOntology {
            nodes: n,
            edges: e,
            version: ver.into()
        }
    }
}

/// Simple HPO parser
/// 
/// Leverages serde to parse JSON and extract labels and synoynms.
pub struct SimpleHpoParser {
    simple_ontology: SimpleOntology
}

impl SimpleHpoParser {
    /// Creates a new SimpleHpoParser the specified path to the hp.json file
    pub fn new(hp_json_path: &str) -> Result<SimpleHpoParser, String> {
        let sonto = get_ontology(hp_json_path)?;
        Ok(SimpleHpoParser {
            simple_ontology: sonto
        })
    }
}


impl TermIdToTextMapper for SimpleHpoParser {
    /// Crreates a map with keys being all lower-case HPO term labels and synonyms and values being the corresponding TermIds.
    fn get_text_to_term_map(&self) -> HashMap<String, TermId> {
        let mymap = build_graph(&self.simple_ontology);
        let minimum_synonym_length = 4;
        let pheno_abn = "HP:0000118"; // Root term for Phenotypic abnormality
        let pheno_subontology_tid_set: HashSet<String> = get_descendants(&mymap, pheno_abn);
        let mut text_to_tid_map = HashMap::new();
        // These are commmon false-positive results related to HPO synonyms that occur in other contexts
        let omittable_labels: HashSet<String> = ["negative".to_string(), "weakness".to_string()].iter().cloned().collect();
        for hp_node in &self.simple_ontology.nodes {
            let term_id_string = &hp_node.term_id;
            if pheno_subontology_tid_set.contains(term_id_string) {
                let tid = TermId::from_str(term_id_string);
                if tid.is_err() {
                    println!("Could not parse {}", term_id_string);
                    continue;
                }
                let tid = tid.unwrap();
                text_to_tid_map.insert(hp_node.term_label.to_ascii_lowercase(), tid.clone());
                for syno in &hp_node.synonyms {
                    if omittable_labels.contains(syno) {
                        continue;
                    }
                    if syno.len() < minimum_synonym_length {
                        continue;
                    }
                    text_to_tid_map.insert(syno.to_ascii_lowercase(), tid.clone());
                }
            }
        }
        text_to_tid_map
    }

}


/// Create a HashMap that represents the is-a graph of the HPO
/// 
/// The key of the grap is a parent, and the value is a vector of children term ids (represented as Strings)
fn build_graph(ontology: &SimpleOntology) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for edge in &ontology.edges {
        graph.entry(edge.destination.clone()).or_default().push(edge.source.clone());
    }

    graph
}


fn get_descendants(graph: &HashMap<String, Vec<String>>, root: &str) -> HashSet<String> {
    let mut subset = HashSet::new();
    let mut stack = vec![root.to_string()];

    while let Some(node) = stack.pop() {
        if !subset.insert(node.clone()) {
            continue; // Skip if already visited
        }
        if let Some(children) = graph.get(&node) {
            stack.extend(children.iter().cloned());
        }
    }

    subset
}

fn get_version(graph:&serde_json::Value)-> String {
    if let Some(meta) = graph["meta"].as_object() {
        if let Some(version) = meta["version"].as_str() {
            return version.to_string()
        }
    }
    "n/a".to_string()
}


fn get_node(node: &serde_json::Value) -> Option<HpNode> {
    //println!("{:?}",node);
    let uri = node["id"].as_str();
    if uri.is_none() {
        return None;
    }
    let uri = uri.unwrap();
    let term_id = get_hp_id(uri);
    if ! term_id.starts_with("HP") {
        return None;
    }
    let label = node["lbl"].as_str();
    if label.is_none() {
        return None; // should nevel happen
    }
    let label = label.unwrap();
    let binding = Vec::new();
    let synonyms_list = node["meta"]["synonyms"].as_array()
        .unwrap_or(&binding); // If missing, use an empty array

    let mut synonyms = Vec::new();
    for syno in synonyms_list {
        if let Some(syn_val) = syno["val"].as_str() {
            synonyms.push(syn_val.to_string());
        }
    }
    Some(HpNode::new(term_id, label, synonyms))
}


fn get_ontology(hp_json_path: &str) -> Result<SimpleOntology, String> {
    let mut edge_array:Vec<HpEdge> = Vec::new();
    let mut node_array: Vec<HpNode> = Vec::new();
    let hp_json_path = PathBuf::from(hp_json_path);
    let data = fs::read_to_string(&hp_json_path)
        .unwrap_or_else(|_| panic!("Unable to read file at '{}'.", hp_json_path.display()));
    let json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
    if let Some(graphs) = json["graphs"].as_array() {
        let graph = graphs.first().expect("Could not find HPO graph");
        let version = get_version(&graph);
        println!("version: {}", version);
       
        if let Some(edges) = graph["edges"].as_array() {
            for edge in edges {
                let s = edge["sub"].as_str().expect("edge did not have sub");
                let d = edge["obj"].as_str().expect("edge did not have obj");
                edge_array.push(HpEdge::new(s.to_string(),d.to_string()));
            }
            let n_edges = edge_array.len();
            println!("edges {}", n_edges);
        }
        if let Some(nodes) = graph["nodes"].as_array() {
            for node in nodes {
                let n = get_node(node);
                if n.is_some() {
                    node_array.push(n.unwrap());
                    //println!("Go no sy");
                }
               
            }
            let n_nodes = node_array.len();
            println!("nodes {}", n_nodes);
        }
       return Ok(SimpleOntology::new(edge_array, node_array, version));
    }
   Err(format!("Could not create ontology from '{}'", hp_json_path.display()))
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_curification() {
        let uri = "http://purl.obolibrary.org/obo/HP_6000699";
        let curie = get_hp_id(uri);
        assert_eq!("HP:6000699", curie);
    }


}