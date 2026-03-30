use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum BinderInfo {
    Default,
    Implicit,
    StrictImplicit,
    InstImplicit,
    AuxDecl,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinderInfoData {
    pub implicit: bool,
    pub strict: bool,
}

impl BinderInfoData {
    #[allow(dead_code)]
    pub fn default() -> Self {
        BinderInfoData {
            implicit: false,
            strict: false,
        }
    }
    #[allow(dead_code)]
    pub fn implicit() -> Self {
        BinderInfoData {
            implicit: true,
            strict: false,
        }
    }
}
