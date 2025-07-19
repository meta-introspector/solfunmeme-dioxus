use anyhow::Result;
use sophia_iri::{Iri, IriRef};
use sophia_api::MownStr;
use sophia_api::term::SimpleTerm;

pub fn create_iri(base_iri: &str, local_name: &str) -> Result<Iri<String>> {
    Ok(Iri::new(format!("{}{}", base_iri, local_name))?)
}

pub fn create_iri_ref(base_iri: &str, local_name: &str) -> Result<IriRef<'static>> {
    Ok(IriRef::new_unchecked(MownStr::from(format!("{}{}", base_iri, local_name).as_str())))
}

pub fn create_iri_term(base_iri: &str, local_name: &str) -> Result<SimpleTerm> {
    Ok(SimpleTerm::Iri(create_iri_ref(base_iri, local_name)?))
}
