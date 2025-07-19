use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::{RdfGraph, term_to_string};
use solfunmeme_clifford::generate_multivector_from_string;

pub fn add_emoji_data_internal(graph: &mut RdfGraph) -> Result<()> {
    let ns_clone = graph.namespaces.clone();
    let type_prop_str = term_to_string(&ns_clone.get_term("rdf", "type")?);
    let emoji_class_str = term_to_string(&ns_clone.get_term("em", "Emoji")?);
    let has_clifford_vector_iri_str = term_to_string(&ns_clone.get_term("onto", "hasCliffordVector")?);
    let xsd_base_iri_str = ns_clone.get_base_iri("xsd").unwrap().as_str();

    let subjects = graph.get_subjects_with_property(&type_prop_str, &emoji_class_str)?;

    for subject in subjects {
        let emoji_name = subject.split('#').last().unwrap_or("").to_string();
        let multivector = generate_multivector_from_string(&emoji_name);
        let multivector_str = format!("{}", multivector);
        graph.add_literal_triple(
            &subject,
            &has_clifford_vector_iri_str,
            &multivector_str,
            xsd_base_iri_str,
        )?;
    }
    Ok(())
}
