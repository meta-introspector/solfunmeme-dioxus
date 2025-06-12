use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Level {
    Zero,
    Succ(Box<Level>),
    Max(Box<Level>, Box<Level>),
    IMax(Box<Level>, Box<Level>),
    Param(&'static str),
    MVar(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LevelType {
    Zero,
    Succ(Box<LevelType>),
    Max(Box<LevelType>, Box<LevelType>),
    IMax(Box<LevelType>, Box<LevelType>),
    Param(String),
    MVar(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LevelDescr {
    level: String,
    kind: String,
}