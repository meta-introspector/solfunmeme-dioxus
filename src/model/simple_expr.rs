// Simple expression types and constructors for Lean 4 expressions
// This module provides types and constructors for representing Lean 4 expressions in Rust.
//use std::collections::HashMap;
use serde::{Deserialize, Serialize};
//use crate::model::binder::BinderInfo;
use crate::model::{
    binder::BinderInfoData,
    level::{Level, *},
};
//use crate::model::simple_expr;

/// Represents a bound variable in Lean 4 expressions
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Forbd {
    pub forbndrTyp: String,
    pub forbdB: String,
}

/// Represents a bound variable with snake_case naming
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Forbd2 {
    pub forbndr_typ: String,
    pub forbd_b: String,
}

/// Represents constant information in Lean 4 expressions
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct CnstInf {
    pub levels: Vec<Level>,
    pub declName: String,
    pub forbd: Forbd,
    pub binderName: String,
    pub binderInfo: String,
}

/// Represents constant information with snake_case naming
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct CnstInf2 {
    pub levels: Vec<Level>,
    pub decl_name: String,
    pub forbd: Forbd,
    pub binder_name: String,
    pub binder_info: String,
}

/// Represents a signature in Lean 4 expressions
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Sig {
    pub atype: String,
    pub forbndrTypB: String,
    pub binderName: String,
    pub binderInfo: String,
}

/// Represents a signature with snake_case naming
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Sig2 {
    pub atype: String,
    pub forbndr_typ_b: String,
    pub binder_name: String,
    pub binder_info: String,
}

/// Represents a kind in Lean 4 expressions
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Foo {
    pub akind: String,
    pub cnstInfB: CnstInfB,
}

// Rust translation of the SimpleExpr type and its recursor
// Based on the Lean 4 JSON representation

// Equivalent to Lean's Name type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Name {
    Anonymous,
    Str(Box<Name>, String),
    Num(Box<Name>, u64),
}

// Equivalent to Lean's BinderInfo

// The main SimpleExpr type (inductive type from Lean)
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum SimpleExpr {
    // Bound variable with de Bruijn index
    BVar {
        index: u64,
    },

    // Sort (Type universe)
    Sort {
        level: Level,
    },
    #[allow(dead_code)]
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
        //#[serde(rename = "binderType")]
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfoData,
    },

    // Dependent product (Pi type / forall)
    ForallE {
        binder_name: Name,
        //	#[serde(rename = "binderType")]
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfoData,
    },
}

impl SimpleExpr {
    // Recursor implementation - pattern matching with continuation-passing style
    #[allow(dead_code)]
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
        F5: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfoData, T, T) -> T + Clone,
        F6: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfoData, T, T) -> T + Clone,
    {
        match self {
            SimpleExpr::BVar { index } => bvar_case(*index),
            SimpleExpr::Sort { level } => sort_case(level),
            SimpleExpr::Const { name, levels } => const_case(name, levels),
            SimpleExpr::App { func, arg } => {
                let func_ih = func.rec(
                    bvar_case.clone(),
                    sort_case.clone(),
                    const_case.clone(),
                    app_case.clone(),
                    lam_case.clone(),
                    forall_case.clone(),
                );
                let arg_ih = arg.rec(
                    bvar_case.clone(),
                    sort_case.clone(),
                    const_case.clone(),
                    app_case.clone(),
                    lam_case.clone(),
                    forall_case.clone(),
                );
                app_case(func, arg, func_ih, arg_ih)
            }
            SimpleExpr::Lam {
                binder_name,
                binder_type,
                body,
                binder_info,
            } => {
                let binder_type_ih = binder_type.rec(
                    bvar_case.clone(),
                    sort_case.clone(),
                    const_case.clone(),
                    app_case.clone(),
                    lam_case.clone(),
                    forall_case.clone(),
                );
                let body_ih = body.rec(
                    bvar_case,
                    sort_case,
                    const_case,
                    app_case,
                    lam_case.clone(),
                    forall_case.clone(),
                );
                lam_case(
                    binder_name,
                    binder_type,
                    body,
                    binder_info,
                    binder_type_ih,
                    body_ih,
                )
            }
            SimpleExpr::ForallE {
                binder_name,
                binder_type,
                body,
                binder_info,
            } => {
                let binder_type_ih = binder_type.rec(
                    bvar_case.clone(),
                    sort_case.clone(),
                    const_case.clone(),
                    app_case.clone(),
                    lam_case.clone(),
                    forall_case.clone(),
                );
                let body_ih = body.rec(
                    bvar_case,
                    sort_case,
                    const_case,
                    app_case,
                    lam_case,
                    forall_case.clone(),
                );
                forall_case(
                    binder_name,
                    binder_type,
                    body,
                    binder_info,
                    binder_type_ih,
                    body_ih,
                )
            }
        }
    }
    #[allow(dead_code)]
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
        F5: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfoData) -> T,
        F6: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfoData) -> T,
    {
        match self {
            SimpleExpr::BVar { index } => bvar_case(*index),
            SimpleExpr::Sort { level } => sort_case(level),
            SimpleExpr::Const { name, levels } => const_case(name, levels),
            SimpleExpr::App { func, arg } => app_case(func, arg),
            SimpleExpr::Lam {
                binder_name,
                binder_type,
                body,
                binder_info,
            } => lam_case(binder_name, binder_type, body, binder_info),
            SimpleExpr::ForallE {
                binder_name,
                binder_type,
                body,
                binder_info,
            } => forall_case(binder_name, binder_type, body, binder_info),
        }
    }
}

// Example usage and helper functions
impl SimpleExpr {
    #[allow(dead_code)]
    // Create a bound variable
    pub fn bvar(index: u64) -> Self {
        SimpleExpr::BVar { index }
    }
    #[allow(dead_code)]
    // Create a sort
    pub fn sort(level: Level) -> Self {
        SimpleExpr::Sort { level }
    }
    #[allow(dead_code)]
    // Create a constant
    pub fn const_expr(name: Name, levels: Vec<Level>) -> Self {
        SimpleExpr::Const { name, levels }
    }
    #[allow(dead_code)]
    // Create an application
    pub fn app(func: SimpleExpr, arg: SimpleExpr) -> Self {
        SimpleExpr::App {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }
    #[allow(dead_code)]
    // Create a lambda
    pub fn lam(
        name: Name,
        binder_type: SimpleExpr,
        body: SimpleExpr,
        info: BinderInfoData,
    ) -> Self {
        SimpleExpr::Lam {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }
    #[allow(dead_code)]
    // Create a forall (Pi type)
    pub fn forall_e(
        name: Name,
        binder_type: SimpleExpr,
        body: SimpleExpr,
        info: BinderInfoData,
    ) -> Self {
        SimpleExpr::ForallE {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }
    #[allow(dead_code)]
    // Helper method to create a simple function type
    pub fn arrow(domain: SimpleExpr, codomain: SimpleExpr) -> Self {
        Self::forall_e(
            Name::anonymous(),
            domain,
            codomain,
            BinderInfoData {
                implicit: false,
                strict: false,
            },
        )
    }
    #[allow(dead_code)]
    // Helper method to create a simple lambda with anonymous binder
    pub fn lambda(binder_type: SimpleExpr, body: SimpleExpr) -> Self {
        Self::lam(
            Name::anonymous(),
            binder_type,
            body,
            BinderInfoData::default(),
        )
    }
}

//#[cfg(test)]
mod tests {

    #[test]
    fn test_simple_expr_construction() {
        // Create a simple expression: λx: Type, x
        let type_sort = SimpleExpr::sort(Level::Zero);
        let x_var = SimpleExpr::bvar(0);
        let identity = SimpleExpr::lam(
            Name::Str(Box::new(Name::Anonymous), "x".to_string()),
            type_sort,
            x_var,
            BinderInfoData::default(),
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
    pub sig: Sig,
    pub cnst_inf: CnstInf, // Fixed naming
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct CnstInfB2 {
    pub sig: Sig,
    pub cnstInf: CnstInf,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Foo2 {
    pub akind: String,
    pub cnst_inf_b: CnstInfB, // Fixed naming
}

// Enhanced SimpleExpr to match JSON structure more closely

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum SimpleExprType<'a> {
    BVar {
        index: Option<u64>,
    },
    Sort {
        level: Level,
    },
    Const {
        levels: Vec<Level>,
        #[serde(rename = "declName")]
        decl_name: std::borrow::Cow<'a, str>,
    },
    App {
        #[serde(rename = "fn")]
        fn_expr: Box<SimpleExprType<'a>>,
        arg: Box<SimpleExprType<'a>>,
    },
    ForallE {
        forbndr_typ: Option<Box<SimpleExprType<'a>>>,
        forbndr_typ_b: Option<Box<SimpleExprType<'a>>>,
        forbd_b: Option<Box<SimpleExprType<'a>>>,
        forbd: Option<Box<SimpleExprType<'a>>>,

        #[serde(rename = "binderName")]
        binder_name: std::borrow::Cow<'a, str>,

        #[serde(rename = "binderInfo")]
        binder_info: std::borrow::Cow<'a, str>,
    },
    Lam {
        #[serde(rename = "binderName")]
        binder_name: std::borrow::Cow<'a, str>,

        #[serde(rename = "binderType")]
        binder_type: Box<SimpleExprType<'a>>,
        body: Box<SimpleExprType<'a>>,

        #[serde(rename = "binderInfo")]
        binder_info: std::borrow::Cow<'a, str>,
    },
}
impl<'a> SimpleExprType<'a> {
    #[allow(dead_code)]
    pub fn bzero() -> Self {
        SimpleExprType::BVar { index: Some(0) }
    }
}

#[allow(dead_code)]
// Convert the JSON chunk 1 into a Rust value at runtime (not a const)
pub fn simple_expr_rec_chunk1<'a>() -> SimpleExprType<'a> {
    // Helper to box and Some
    fn some_box<'a>(expr: SimpleExprType<'a>) -> Option<Box<SimpleExprType<'a>>> {
        Some(Box::new(expr))
    }

    SimpleExprType::ForallE {
        forbndr_typ_b: some_box(SimpleExprType::ForallE {
            forbndr_typ: some_box(SimpleExprType::Const {
                levels: vec![
                    LEVEL_U1F(),
                    LEVEL_U2F(),
                    LEVEL_U3F(),
                    LEVEL_U4F(),
                    LEVEL_U5F(),
                    LEVEL_U6F(),
                    LEVEL_U7F(),
                    LEVEL_U8F(),
                ],
                decl_name: std::borrow::Cow::Borrowed("SimpleExpr"),
            }),
            forbndr_typ_b: None,
            forbd_b: some_box(SimpleExprType::Sort {
                level: Level::Param("u".to_string()),
            }),
            forbd: some_box(SimpleExprType::ForallE {
                forbndr_typ: some_box(SimpleExprType::ForallE {
                    forbndr_typ: some_box(SimpleExprType::Sort {
                        level: Level::Param("u_1".to_string()),
                    }),
                    forbndr_typ_b: None,
                    forbd_b: None,
                    forbd: None,
                    binder_name: std::borrow::Cow::Borrowed("Nat"),
                    binder_info: std::borrow::Cow::Borrowed("implicit"),
                }),
                forbndr_typ_b: None,
                forbd_b: some_box(SimpleExprType::ForallE {
                    forbndr_typ: some_box(SimpleExprType::ForallE {
                        forbndr_typ: some_box(SimpleExprType::Const {
                            levels: vec![LEVEL_U2F(), LEVEL_U3F()],
                            decl_name: std::borrow::Cow::Borrowed("Level"),
                        }),
                        forbndr_typ_b: None,
                        forbd_b: some_box(SimpleExprType::App {
                            fn_expr: Box::new(SimpleExprType::BVar { index: None }),
                            arg: Box::new(SimpleExprType::App {
                                fn_expr: Box::new(SimpleExprType::Const {
                                    levels: vec![
                                        LEVEL_U1F(),
                                        LEVEL_U2F(),
                                        LEVEL_U3F(),
                                        LEVEL_U4F(),
                                        LEVEL_U5F(),
                                        LEVEL_U6F(),
                                        LEVEL_U7F(),
                                        LEVEL_U8F(),
                                    ],
                                    decl_name: std::borrow::Cow::Borrowed("SimpleExpr.sort"),
                                }),
                                arg: Box::new(SimpleExprType::BVar { index: None }),
                            }),
                        }),
                        forbd: None,
                        binder_name: std::borrow::Cow::Borrowed("u"),
                        binder_info: std::borrow::Cow::Borrowed("default"),
                    }),
                    forbndr_typ_b: None,
                    forbd_b: None,
                    forbd: None,
                    binder_name: std::borrow::Cow::Borrowed("sort"),
                    binder_info: std::borrow::Cow::Borrowed("default"),
                }),
                forbd: None,
                binder_name: std::borrow::Cow::Borrowed("bvar"),
                binder_info: std::borrow::Cow::Borrowed("default"),
            }),
            binder_name: std::borrow::Cow::Borrowed("t"),
            binder_info: std::borrow::Cow::Borrowed("default"),
        }),
        forbndr_typ: None,
        forbd_b: None,
        forbd: None,
        binder_name: std::borrow::Cow::Borrowed(""),
        binder_info: std::borrow::Cow::Borrowed(""),
    }
}

//#[cfg(test)]
mod tests2 {

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

#[allow(dead_code)]
fn maintest() {
    println!("Chunk 1 converted to Rust structure");
    println!("Structure depth: Very deep nested ForallE expressions");
}

impl Forbd {
    #[allow(dead_code)]
    pub fn new(forbndr_typ: String, forbd_b: String) -> Self {
        Self {
            forbndrTyp: forbndr_typ,
            forbdB: forbd_b,
        }
    }
}

impl Forbd2 {
    #[allow(dead_code)]
    pub fn new(forbndr_typ: String, forbd_b: String) -> Self {
        Self {
            forbndr_typ,
            forbd_b,
        }
    }
}

impl CnstInf {
    #[allow(dead_code)]
    pub fn new(
        levels: Vec<Level>,
        decl_name: String,
        forbd: Forbd,
        binder_name: String,
        binder_info: String,
    ) -> Self {
        Self {
            levels,
            declName: decl_name,
            forbd,
            binderName: binder_name,
            binderInfo: binder_info,
        }
    }
}

impl CnstInf2 {
    #[allow(dead_code)]
    pub fn new(
        levels: Vec<Level>,
        decl_name: String,
        forbd: Forbd,
        binder_name: String,
        binder_info: String,
    ) -> Self {
        Self {
            levels,
            decl_name,
            forbd,
            binder_name,
            binder_info,
        }
    }
}

impl Sig {
    #[allow(dead_code)]
    pub fn new(
        atype: String,
        forbndr_typ_b: String,
        binder_name: String,
        binder_info: String,
    ) -> Self {
        Self {
            atype,
            forbndrTypB: forbndr_typ_b,
            binderName: binder_name,
            binderInfo: binder_info,
        }
    }
}

impl Sig2 {
    #[allow(dead_code)]
    pub fn new(
        atype: String,
        forbndr_typ_b: String,
        binder_name: String,
        binder_info: String,
    ) -> Self {
        Self {
            atype,
            forbndr_typ_b,
            binder_name,
            binder_info,
        }
    }
}

impl Foo {
    #[allow(dead_code)]
    pub fn new(akind: String, cnst_inf_b: CnstInfB) -> Self {
        Self {
            akind,
            //cnst_inf_b: cnst_inf_b,
            cnstInfB: cnst_inf_b,
        }
    }
}

impl CnstInfB {
    #[allow(dead_code)]
    pub fn new(sig: Sig, cnst_inf: CnstInf) -> Self {
        Self { sig, cnst_inf }
    }
}

impl CnstInfB2 {
    #[allow(dead_code)]
    pub fn new(sig: Sig, cnst_inf: CnstInf) -> Self {
        Self {
            sig,
            cnstInf: cnst_inf,
        }
    }
}

impl Foo2 {
    #[allow(dead_code)]
    pub fn new(akind: String, cnst_inf_b: CnstInfB) -> Self {
        Self { akind, cnst_inf_b }
    }
}

impl Name {
    #[allow(dead_code)]
    pub fn anonymous() -> Self {
        Self::Anonymous
    }
    #[allow(dead_code)]
    pub fn str(name: Name, string: String) -> Self {
        Self::Str(Box::new(name), string)
    }
    #[allow(dead_code)]
    pub fn num(name: Name, num: u64) -> Self {
        Self::Num(Box::new(name), num)
    }
    #[allow(dead_code)]
    pub fn from_string(s: String) -> Self {
        Self::Str(Box::new(Self::Anonymous), s)
    }
}

impl<'a> SimpleExprType<'a> {
    #[allow(dead_code)]
    pub fn bvar(index: Option<u64>) -> SimpleExprType<'static> {
        SimpleExprType::BVar { index }
    }

    #[allow(dead_code)]
    pub fn sort(level: Level) -> SimpleExprType<'static> {
        SimpleExprType::Sort { level }
    }

    #[allow(dead_code)]
    pub fn const_expr(levels: Vec<Level>, decl_name: String) -> SimpleExprType<'static> {
        SimpleExprType::Const {
            levels,
            decl_name: std::borrow::Cow::Owned(decl_name),
        }
    }

    #[allow(dead_code)]
    pub fn app(fn_expr: SimpleExprType<'a>, arg: SimpleExprType<'a>) -> SimpleExprType<'a> {
        SimpleExprType::App {
            fn_expr: Box::new(fn_expr),
            arg: Box::new(arg),
        }
    }

    #[allow(dead_code)]
    pub fn forall_e(
        forbndr_typ: Option<SimpleExprType<'a>>,
        forbndr_typ_b: Option<SimpleExprType<'a>>,
        forbd_b: Option<SimpleExprType<'a>>,
        forbd: Option<SimpleExprType<'a>>,
        binder_name: &'a str,
        binder_info: &'a str,
    ) -> SimpleExprType<'a> {
        SimpleExprType::ForallE {
            forbndr_typ: forbndr_typ.map(Box::new),
            forbndr_typ_b: forbndr_typ_b.map(Box::new),
            forbd_b: forbd_b.map(Box::new),
            forbd: forbd.map(Box::new),
            binder_name: std::borrow::Cow::Borrowed(binder_name),
            binder_info: std::borrow::Cow::Borrowed(binder_info),
        }
    }
    #[allow(dead_code)]
    pub fn lam(
        binder_name: &'a str,
        binder_type: SimpleExprType<'a>,
        body: SimpleExprType<'a>,
        binder_info: &'a str,
    ) -> SimpleExprType<'a> {
        SimpleExprType::Lam {
            binder_name: std::borrow::Cow::Borrowed(binder_name),
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: std::borrow::Cow::Borrowed(binder_info),
        }
    }
}
