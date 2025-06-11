
use std::collections::HashMap;

use crate::model::simple_expr;

// Equivalent to Lean's Level type
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
pub struct LevelDescr {
    level: String,
    kind: String,
}

            // Define globals for u_1 to u_8
pub const LEVEL_U1: Level = Level::Param("u_1");
pub const LEVEL_U2: Level = Level::Param("u_2");
pub const LEVEL_U3: Level = Level::Param("u_3");
pub const LEVEL_U4: Level = Level::Param("u_4");
pub const LEVEL_U5: Level = Level::Param("u_5");
pub const LEVEL_U6: Level = Level::Param("u_6");
pub const LEVEL_U7: Level = Level::Param("u_7");
pub const LEVEL_U8: Level = Level::Param("u_8");

// Use a function to return the vector at runtime, since Vec and .clone() are not allowed in consts
pub fn levels_8() -> Vec<Level> {
    vec![
        LEVEL_U1,
        LEVEL_U2,
        LEVEL_U3,
        LEVEL_U4,
        LEVEL_U5,
        LEVEL_U6,
        LEVEL_U7,
        LEVEL_U8,
    ]
}
#[derive(Debug, Clone, PartialEq)]
pub struct Forbd {
    forbndrTyp: String,
    forbdB: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Forbd2 {
    forbndr_typ: String, // Fixed naming
    forbd_b: String,     // Fixed naming
}

#[derive(Debug, Clone, PartialEq)]
pub struct CnstInf {
    levels: Vec<Level>,
    declName: String,
    forbd: Forbd,
    binderName: String,
    binderInfo: String,
}  

#[derive(Debug, Clone, PartialEq)]
pub struct CnstInf2 {
    levels: Vec<Level>,
    decl_name: String,   // Fixed naming
    forbd: Forbd,
    binder_name: String, // Fixed naming
    binder_info: String, // Fixed naming
}
#[derive(Debug, Clone, PartialEq)]
pub struct Sig {
    atype: String,
    forbndrTypB: String,
    binderName: String,
    binderInfo: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sig2 {
    atype: String,
    forbndr_typ_b: String, // Fixed naming
    binder_name: String,   // Fixed naming
    binder_info: String,   // Fixed naming
}


#[derive(Debug, Clone, PartialEq)]
pub struct Foo {

    akind: String,
    cnstInfB: CnstInfB,
}

// Rust translation of the SimpleExpr type and its recursor
// Based on the Lean 4 JSON representation



// Equivalent to Lean's Name type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Name {
    Anonymous,
    Str(Box<Name>, String),
    Num(Box<Name>, u64),
}

// Equivalent to Lean's BinderInfo
#[derive(Debug, Clone, PartialEq)]
pub enum BinderInfo {
    Default,
    Implicit,
    StrictImplicit,
    InstImplicit,
    AuxDecl,
}

// The main SimpleExpr type (inductive type from Lean)
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleExpr {
    // Bound variable with de Bruijn index
    BVar {
        index: u64,
    },
    
    // Sort (Type universe)
    Sort {
        level: Level,
    },
    
    // Constant with name and universe levels
    Const {
        name: Name,
        levels: Vec<Level>,
    },
    
    // Function application
    App {
        func: Box<SimpleExpr>,
        arg: Box<SimpleExpr>,
    },
    
    // Lambda abstraction
    Lam {
        binder_name: Name,
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfo,
    },
    
    // Dependent product (Pi type / forall)
    ForallE {
        binder_name: Name,
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfo,
    },
}

impl SimpleExpr {
    // Recursor implementation - pattern matching with continuation-passing style
    pub fn rec<T, F1, F2, F3, F4, F5, F6>(
        &self,
        bvar_case: F1,
        sort_case: F2,
        const_case: F3,
        app_case: F4,
        lam_case: F5,
        forall_case: F6,
    ) -> T
    where
        F1: FnOnce(u64) -> T + Clone,
        F2: FnOnce(&Level) -> T + Clone,
        F3: FnOnce(&Name, &[Level]) -> T + Clone,
        F4: FnOnce(&SimpleExpr, &SimpleExpr, T, T) -> T + Clone,
        F5: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfo, T, T) -> T + Clone,
        F6: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfo, T, T) -> T + Clone,
    {
        match self {
            SimpleExpr::BVar { index } => bvar_case(*index),
            SimpleExpr::Sort { level } => sort_case(level),
            SimpleExpr::Const { name, levels } => const_case(name, levels),
            SimpleExpr::App { func, arg } => {
                let func_ih = func.rec(
                    bvar_case.clone(), sort_case.clone(), const_case.clone(),
                    app_case.clone(), lam_case.clone(), forall_case.clone()
                );
                let arg_ih = arg.rec(
                    bvar_case.clone(), sort_case.clone(), const_case.clone(),
                    app_case.clone(), lam_case.clone(), forall_case.clone()
                );
                app_case(func, arg, func_ih, arg_ih)
            }
            SimpleExpr::Lam { binder_name, binder_type, body, binder_info } => {
                let binder_type_ih = binder_type.rec(
                    bvar_case.clone(), sort_case.clone(), const_case.clone(),
                    app_case.clone(), lam_case.clone(), forall_case.clone()
                );
                let body_ih = body.rec(
                    bvar_case, sort_case, const_case,
                    app_case, lam_case.clone(), forall_case
                );
                lam_case(binder_name, binder_type, body, binder_info, binder_type_ih, body_ih)
            }
            SimpleExpr::ForallE { binder_name, binder_type, body, binder_info } => {
                let binder_type_ih = binder_type.rec(
                    bvar_case.clone(), sort_case.clone(), const_case.clone(),
                    app_case.clone(), lam_case.clone(), forall_case.clone()
                );
                let body_ih = body.rec(
                    bvar_case, sort_case, const_case,
                    app_case, lam_case, forall_case.clone()
                );
                forall_case(binder_name, binder_type, body, binder_info, binder_type_ih, body_ih)
            }
        }
    }
    
    // Helper method for simple pattern matching without recursion
    pub fn match_expr<T, F1, F2, F3, F4, F5, F6>(
        &self,
        bvar_case: F1,
        sort_case: F2,
        const_case: F3,
        app_case: F4,
        lam_case: F5,
        forall_case: F6,
    ) -> T
    where
        F1: FnOnce(u64) -> T,
        F2: FnOnce(&Level) -> T,
        F3: FnOnce(&Name, &[Level]) -> T,
        F4: FnOnce(&SimpleExpr, &SimpleExpr) -> T,
        F5: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfo) -> T,
        F6: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfo) -> T,
    {
        match self {
            SimpleExpr::BVar { index } => bvar_case(*index),
            SimpleExpr::Sort { level } => sort_case(level),
            SimpleExpr::Const { name, levels } => const_case(name, levels),
            SimpleExpr::App { func, arg } => app_case(func, arg),
            SimpleExpr::Lam { binder_name, binder_type, body, binder_info } => {
                lam_case(binder_name, binder_type, body, binder_info)
            },
            SimpleExpr::ForallE { binder_name, binder_type, body, binder_info } => {
                forall_case(binder_name, binder_type, body, binder_info)
            },
        }
    }
}

// Example usage and helper functions
impl SimpleExpr {
    // Create a bound variable
    pub fn bvar(index: u64) -> Self {
        SimpleExpr::BVar { index }
    }
    
    // Create a sort
    pub fn sort(level: Level) -> Self {
        SimpleExpr::Sort { level }
    }
    
    // Create a constant
    pub fn const_expr(name: Name, levels: Vec<Level>) -> Self {
        SimpleExpr::Const { name, levels }
    }
    
    // Create an application
    pub fn app(func: SimpleExpr, arg: SimpleExpr) -> Self {
        SimpleExpr::App {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }
    
    // Create a lambda
    pub fn lam(name: Name, binder_type: SimpleExpr, body: SimpleExpr, info: BinderInfo) -> Self {
        SimpleExpr::Lam {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }
    
    // Create a forall (Pi type)
    pub fn forall_e(name: Name, binder_type: SimpleExpr, body: SimpleExpr, info: BinderInfo) -> Self {
        SimpleExpr::ForallE {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_expr_construction() {
        // Create a simple expression: λx: Type, x
        let type_sort = SimpleExpr::sort(Level::Zero);
        let x_var = SimpleExpr::bvar(0);
        let identity = SimpleExpr::lam(
            Name::Str(Box::new(Name::Anonymous), "x".to_string()),
            type_sort,
            x_var,
            BinderInfo::Default,
        );
        
        // Test pattern matching
        let result = identity.match_expr(
            |_| "bvar",
            |_| "sort", 
            |_, _| "const",
            |_, _| "app",
            |_, _, _, _| "lambda",
            |_, _, _, _| "forall",
        );
        
        assert_eq!(result, "lambda");
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CnstInfB {
    sig: Sig,
    cnst_inf: CnstInf, // Fixed naming
}

#[derive(Debug, Clone, PartialEq)]
pub struct CnstInfB2 {
    sig: Sig,
    cnstInf: CnstInf,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Foo2 {
    akind: String,
    cnst_inf_b: CnstInfB, // Fixed naming
}

// Enhanced Level enum to match the JSON better
#[derive(Debug, Clone, PartialEq)]
pub enum LevelType {
    Zero,
    Succ(Box<LevelType>),
    Max(Box<LevelType>, Box<LevelType>),
    IMax(Box<LevelType>, Box<LevelType>),
    Param(String),
    MVar(u64),
}

// Enhanced SimpleExpr to match JSON structure more closely
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleExprType {
    BVar {
        index: Option<u64>,
    },
    Sort {
        level: Level,
    },
    Const {
        levels: Vec<Level>,
        decl_name: String,
    },
    App {
        fn_expr: Box<SimpleExprType>,
        arg: Box<SimpleExprType>,
    },
    ForallE {
        forbndr_typ: Option<Box<SimpleExprType>>,
        forbndr_typ_b: Option<Box<SimpleExprType>>,
        forbd_b: Option<Box<SimpleExprType>>,
        forbd: Option<Box<SimpleExprType>>,
        binder_name: String,
        binder_info: String,
    },
    Lam {
        binder_name: String,
        binder_type: Box<SimpleExprType>,
        body: Box<SimpleExprType>,
        binder_info: String,
    },
}

// Convert the JSON chunk 1 into a Rust value at runtime (not a const)
pub fn simple_expr_rec_chunk1() -> SimpleExprType {
    // Helper to box and Some
    fn some_box(expr: SimpleExprType) -> Option<Box<SimpleExprType>> {
        Some(Box::new(expr))
    }

    SimpleExprType::ForallE {
        forbndr_typ_b: some_box(SimpleExprType::ForallE {
            forbndr_typ: some_box(SimpleExprType::Const {
                levels: vec![
                    LEVEL_U1,
                    LEVEL_U2,
                    LEVEL_U3,
                    LEVEL_U4,
                    LEVEL_U5,
                    LEVEL_U6,
                    LEVEL_U7,
                    LEVEL_U8,
                ],
                decl_name: String::from("SimpleExpr"),
            }),
            forbndr_typ_b: None,
            forbd_b: some_box(SimpleExprType::Sort {
                level: Level::Param("u"),
            }),
            forbd: some_box(SimpleExprType::ForallE {
                forbndr_typ: some_box(SimpleExprType::ForallE {
                    forbndr_typ: some_box(SimpleExprType::Sort {
                        level: Level::Param("u_1"),
                    }),
                    forbndr_typ_b: None,
                    forbd_b: None,
                    forbd: None,
                    binder_name: String::from("Nat"),
                    binder_info: String::from("implicit"),
                }),
                forbndr_typ_b: None,
                forbd_b: some_box(SimpleExprType::ForallE {
                    forbndr_typ: some_box(SimpleExprType::ForallE {
                        forbndr_typ: some_box(SimpleExprType::Const {
                            levels: vec![
                                LEVEL_U2,
                                LEVEL_U3,
                            ],
                            decl_name: String::from("Level"),
                        }),
                        forbndr_typ_b: None,
                        forbd_b: some_box(SimpleExprType::App {
                            fn_expr: Box::new(SimpleExprType::BVar { index: None }),
                            arg: Box::new(SimpleExprType::App {
                                fn_expr: Box::new(SimpleExprType::Const {
                                    levels: vec![
                                        LEVEL_U1,
                                        LEVEL_U2,
                                        LEVEL_U3,
                                        LEVEL_U4,
                                        LEVEL_U5,
                                        LEVEL_U6,
                                        LEVEL_U7,
                                        LEVEL_U8,
                                    ],
                                    decl_name: String::from("SimpleExpr.sort"),
                                }),
                                arg: Box::new(SimpleExprType::BVar { index: None }),
                            }),
                        }),
                        forbd: None,
                        binder_name: String::from("u"),
                        binder_info: String::from("default"),
                    }),
                    forbndr_typ_b: None,
                    forbd_b: None,
                    forbd: None,
                    binder_name: String::from("sort"),
                    binder_info: String::from("default"),
                }),
                forbd: None,
                binder_name: String::from("bvar"),
                binder_info: String::from("default"),
            }),
            binder_name: String::from("t"),
            binder_info: String::from("default"),
        }),
        forbndr_typ: None,
        forbd_b: None,
        forbd: None,
        binder_name: String::from(""),
        binder_info: String::from(""),
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;
    
    #[test]
    fn test_chunk1_structure() {
        // Test that the structure compiles and can be accessed
        let expr = simple_expr_rec_chunk1();
        match &expr {
            SimpleExprType::ForallE { binder_name, .. } => {
                println!("Root binder: {}", binder_name);
            }
            _ => panic!("Expected ForallE at root"),
        }
    }
}

fn main() {
    println!("Chunk 1 converted to Rust structure");
    println!("Structure depth: Very deep nested ForallE expressions");
}