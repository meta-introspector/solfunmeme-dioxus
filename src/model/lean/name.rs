use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Name {
    Anonymous,
    Str(Box<Name>, String),
    Num(Box<Name>, u64),
}
