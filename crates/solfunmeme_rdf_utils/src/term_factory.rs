use sophia_api::term::{SimpleTerm, BnodeId};
use sophia_iri::IriRef;
use sophia_api::ns::xsd;
use sophia_api::MownStr;

pub fn iri_term(iri_string: String) -> anyhow::Result<SimpleTerm<'static>> {
    Ok(SimpleTerm::Iri(IriRef::new_unchecked(MownStr::from(iri_string))))
}

pub fn literal_term(value: &str) -> SimpleTerm {
    SimpleTerm::LiteralDatatype(MownStr::from(value), IriRef::new_unchecked(xsd::string.iriref().unwrap()))
}

pub fn literal_term_typed<'a>(value: &'a str, datatype_iri: &'a str) -> anyhow::Result<SimpleTerm<'a>> {
    let iri_ref = IriRef::new_unchecked(MownStr::from(datatype_iri));
    Ok(SimpleTerm::LiteralDatatype(MownStr::from(value), iri_ref))
}

pub fn bnode_term(id: String) -> anyhow::Result<SimpleTerm<'static>> {
    Ok(SimpleTerm::BlankNode(BnodeId::new(MownStr::from(id))?))
}