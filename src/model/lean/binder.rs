#[derive(Debug, Clone, PartialEq)]
pub enum BinderInfo {
    Default,
    Implicit,
    StrictImplicit,
    InstImplicit,
    AuxDecl,
}