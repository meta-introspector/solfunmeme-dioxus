//use crate::level::Level;

use super::{forbd::Forbd, sig::Sig};
//use crate::level::Level;

use crate::model::level::Level;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct CnstInf {
    pub levels: Vec<Level>,
    pub decl_name: String,
    pub forbd: Forbd,
    pub binder_name: String,
    pub binder_info: String,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct CnstInfB {
    pub sig: Sig,
    pub cnst_inf: CnstInf,
}
