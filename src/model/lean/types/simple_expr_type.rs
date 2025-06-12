use crate::{constants::*, level::Level};

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleExprType {
    BVar { index: Option<u64> },
    Sort { level: Level },
    Const { levels: Vec<Level>, decl_name: String },
    App { fn_expr: Box<SimpleExprType>, arg: Box<SimpleExprType> },
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

pub fn simple_expr_rec_chunk1() -> SimpleExprType {
    fn some_box(expr: SimpleExprType) -> Option<Box<SimpleExprType>> {
        Some(Box::new(expr))
    }

    SimpleExprType::ForallE {
        forbndr_typ_b: some_box(SimpleExprType::ForallE {
            forbndr_typ: some_box(SimpleExprType::Const {
                levels: levels_8(),
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
                            levels: vec![LEVEL_U2, LEVEL_U3],
                            decl_name: String::from("Level"),
                        }),
                        forbndr_typ_b: None,
                        forbd_b: some_box(SimpleExprType::App {
                            fn_expr: Box::new(SimpleExprType::BVar { index: None }),
                            arg: Box::new(SimpleExprType::App {
                                fn_expr: Box::new(SimpleExprType::Const {
                                    levels: levels_8(),
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