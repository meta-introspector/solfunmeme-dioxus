use super::{forbd::Forbd, sig::Sig};
use crate::level::Level;

#[derive(Debug, Clone, PartialEq)]
pub struct CnstInf {
    levels: Vec<Level>,
    decl_name: String,
    forbd: Forbd,
    binder_name: String,
    binder_info: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CnstInfB {
    sig: Sig,
    cnst_inf: CnstInf,
}
