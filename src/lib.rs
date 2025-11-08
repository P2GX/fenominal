//! Implementation of the Fenominal text mining algorithm.
//!
//! ## Configure Fenominal
//!
//! A [`fenominal::Fenominal`] struct is created from [`ontolius::ontology::csr::FullCsrOntology`],
//! which can, in turn, be loaded from a HPO JSON file:
//!
//! ```
//! use std::fs::File;
//! use std::io::BufReader;
//! use std::sync::Arc;
//! use flate2::bufread::GzDecoder;
//! use fenominal::fenominal::Fenominal;
//! use ontolius::io::OntologyLoaderBuilder;
//! use ontolius::ontology::csr::FullCsrOntology;
//!
//! // Load HPO from the repo, use `flate2` to decompress on the fly
//! let hp_path = "resources/hp.v2025-03-03.json.gz";
//! let loader = OntologyLoaderBuilder::new().obographs_parser().build();
//! let hpo: FullCsrOntology = loader.load_from_read(
//!             GzDecoder::new(BufReader::new(File::open(hp_path).expect("HPO should be readable")))
//!           ).expect("HPO should be well formatted");
//! let hpo = Arc::new(hpo);
//! // Configure Fenominal
//! let fenominal = Fenominal::new(hpo);
//! ```
//!
//! ## Use Fenominal
//! 
//! The input to fenominal is a text that may contain 
//! [Human Phenotype Ontology (HPO)](https://hpo.jax.org/app/) term labels.
//! 
//!
//! ### Example
//! 
//! Get [`fenominal::FenominalHit`]s for an example text:
//! 
//! ```
//! use std::fs::File;
//! use std::io::BufReader;
//! use std::sync::Arc;
//! use flate2::bufread::GzDecoder;
//! use fenominal::fenominal::Fenominal;
//! use ontolius::io::OntologyLoaderBuilder;
//! use ontolius::ontology::csr::FullCsrOntology;
//! let hp_path = "resources/hp.v2025-03-03.json.gz";
//! let loader = OntologyLoaderBuilder::new().obographs_parser().build();
//! let hpo: FullCsrOntology = loader.load_from_read(
//!              GzDecoder::new(BufReader::new(File::open(hp_path).expect("HPO should be readable")))
//!            ).expect("HPO should be well formatted");
//! let hpo = Arc::new(hpo);
//! let fenominal = Fenominal::new(hpo);
//! use fenominal::fenominal::FenominalHit;
//!
//! // Perform text mining
//! let text = "Intellectual disability, macrocephaly, scoliosis";
//! let hits: Vec<FenominalHit> = fenominal.process(&text);
//!
//! let labels: Vec<_> = hits.iter().map(|hit| &hit.label).collect();
//! assert_eq!(labels, &["Intellectual disability", "Macrocephaly", "Scoliosis"]);
//! ```
//!
//! 

mod core_document;
pub mod fenominal;
mod hpo;
mod simple_sentence;
mod simple_token;
mod stopwords;
