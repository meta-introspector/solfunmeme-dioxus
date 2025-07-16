use sophia_api::prelude::{IriRef, Term};
use sophia_api::term::SimpleTerm;

pub struct Namespaces<'a> {
    pub ex_iri: IriRef<String>,
    pub rdf_iri: IriRef<String>,
    pub rdfs_iri: IriRef<String>,
    pub em_iri: IriRef<String>,

    pub ex: SimpleTerm<'a>,
    pub rdf: SimpleTerm<'a>,
    pub rdfs: SimpleTerm<'a>,
    pub em: SimpleTerm<'a>,
}

pub fn define_namespaces<'a>() -> Namespaces<'a> {
    let ex_iri: IriRef<String> = IriRef::new_unchecked("http://example.org/ontology/".into());
    let rdf_iri: IriRef<String> = IriRef::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#".into());
    let rdfs_iri: IriRef<String> = IriRef::new_unchecked("http://www.w3.org/2000/01/rdf-schema#".into());
    let em_iri: IriRef<String> = IriRef::new_unchecked("http://example.org/emoji#".into());

    let ex = ex_iri.clone().into_term();
    let rdf = rdf_iri.clone().into_term();
    let rdfs = rdfs_iri.clone().into_term();
    let em = em_iri.clone().into_term();

    Namespaces {
        ex_iri,
        rdf_iri,
        rdfs_iri,
        em_iri,
        ex,
        rdf,
        rdfs,
        em,
    }
}
