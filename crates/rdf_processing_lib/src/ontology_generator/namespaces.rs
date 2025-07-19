use solfunmeme_rdf_utils::namespace_manager::NamespaceManager;

pub fn define_namespaces() -> NamespaceManager {
    let mut ns = NamespaceManager::new();
    ns.add_namespace("ex", "http://example.org/ontology/").unwrap();
    ns.add_namespace("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    ns.add_namespace("rdfs", "http://www.w3.org/2000/01/rdf-schema#").unwrap();
    ns.add_namespace("em", "http://example.org/emoji#").unwrap();
    ns.add_namespace("xsd", "http://www.w3.org/2001/XMLSchema#").unwrap();
    ns
}
