use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::{RdfGraph, term_to_string};
use solfunmeme_clifford::generate_multivector_from_string;

pub fn add_crate_data_internal(graph: &mut RdfGraph) -> Result<()> {
    let ns_clone = graph.namespaces.clone();
    let rdfs_label_str = term_to_string(&ns_clone.get_term("rdfs", "label")?);
    let has_clifford_vector_iri_str = term_to_string(&ns_clone.get_term("onto", "hasCliffordVector")?);
    let rdf_type_str = term_to_string(&ns_clone.get_term("rdf", "type")?);
    let crates_root_prefix = ns_clone.get_base_iri("crates_root").unwrap().as_str();
    let xsd_base_iri_str = ns_clone.get_base_iri("xsd").unwrap().as_str();

    let subjects = graph.get_subjects_with_property(&rdfs_label_str, &rdf_type_str)?;

    for subject in subjects {
        if subject.starts_with(crates_root_prefix) {
            if let Some(crate_name) = graph.get_property_value(&subject, &rdfs_label_str)? {
                let multivector = generate_multivector_from_string(&crate_name);
                let multivector_str = format!("{}", multivector);
                graph.add_literal_triple(
                    &subject,
                    &has_clifford_vector_iri_str,
                    &multivector_str,
                    xsd_base_iri_str,
                )?;
            }
        }
    }
    Ok(())
}
