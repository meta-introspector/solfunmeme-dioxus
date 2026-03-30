//use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum LevelType<'a> {
    Zero,
    //Succ(Box<LevelType>),
    //Max(Box<LevelType>, Box<LevelType>),
    //IMax(Box<LevelType>, Box<LevelType>),
    Param(&'a str),
    MVar(u64),
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct LevelDescr {
    pub level: String,
    pub kind: String,
}

// Equivalent to Lean's Level type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Level {
    Zero,
    Succ(Box<Level>),
    Max(Box<Level>, Box<Level>),
    IMax(Box<Level>, Box<Level>),
    Param(String),
    MVar(u64),
}

// Define globals for u_1 to u_8
#[allow(dead_code)]
pub fn LEVEL_U1F() -> Level {
    Level::Param("u_1".to_string())
}
#[allow(dead_code)]
pub fn LEVEL_U2F() -> Level {
    Level::Param("u_2".to_string())
}
#[allow(dead_code)]
pub fn LEVEL_U3F() -> Level {
    Level::Param("u_3".to_string())
}
#[allow(dead_code)]
pub fn LEVEL_U4F() -> Level {
    Level::Param("u_4".to_string())
}
#[allow(dead_code)]
pub fn LEVEL_U5F() -> Level {
    Level::Param("u_5".to_string())
}
#[allow(dead_code)]
pub fn LEVEL_U6F() -> Level {
    Level::Param("u_6".to_string())
}
#[allow(dead_code)]
pub fn LEVEL_U7F() -> Level {
    Level::Param("u_7".to_string())
}
#[allow(dead_code)]
pub fn LEVEL_U8F() -> Level {
    Level::Param("u_8".to_string())
}

// Use a function to return the vector at runtime, since Vec and .clone() are not allowed in consts
#[allow(dead_code)]
pub fn levels_8() -> Vec<Level> {
    vec![
        LEVEL_U1F(),
        LEVEL_U2F(),
        LEVEL_U3F(),
        LEVEL_U4F(),
        LEVEL_U5F(),
        LEVEL_U6F(),
        LEVEL_U7F(),
        LEVEL_U8F(),
    ]
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(tag = "type")]
// struct Level {
//     level: String,
//     kind: String,
// }

// Helper function to convert Level to String
pub fn level_to_string(level: &Level) -> String {
    use crate::model::level::Level;
    match level {
        Level::Zero => "0".to_string(),
        Level::Succ(l) => format!("succ({})", level_to_string(l)),
        Level::Max(l1, l2) => format!("max({}, {})", level_to_string(l1), level_to_string(l2)),
        Level::IMax(l1, l2) => format!("imax({}, {})", level_to_string(l1), level_to_string(l2)),
        Level::Param(s) => s.to_string(),
        Level::MVar(n) => format!("?{n}"),
    }
}

// // Enhanced Level enum to match the JSON better
// #[derive(Debug, Clone, PartialEq)]
// pub enum LevelType {
//     Zero,
//     Succ(Box<LevelType>),
//     Max(Box<LevelType>, Box<LevelType>),
//     IMax(Box<LevelType>, Box<LevelType>),
//     Param(String),
//     MVar(u64),
// }

//use super::level::Level;
//use crate::model::lean::LevelType;
//use crate::level::LevelType;
#[allow(dead_code)]
pub const LEVEL_U1: LevelType = LevelType::Param("u_1");
#[allow(dead_code)]
pub const LEVEL_U2: LevelType = LevelType::Param("u_2");
#[allow(dead_code)]
pub const LEVEL_U3: LevelType = LevelType::Param("u_3");
#[allow(dead_code)]
pub const LEVEL_U4: LevelType = LevelType::Param("u_4");
#[allow(dead_code)]
pub const LEVEL_U5: LevelType = LevelType::Param("u_5");
#[allow(dead_code)]
pub const LEVEL_U6: LevelType = LevelType::Param("u_6");
#[allow(dead_code)]
pub const LEVEL_U7: LevelType = LevelType::Param("u_7");
#[allow(dead_code)]
pub const LEVEL_U8: LevelType = LevelType::Param("u_8");
#[allow(dead_code)]
pub fn levels_8f() -> Vec<LevelType<'static>> {
    vec![
        LEVEL_U1, LEVEL_U2, LEVEL_U3, LEVEL_U4, LEVEL_U5, LEVEL_U6, LEVEL_U7, LEVEL_U8,
    ]
}
