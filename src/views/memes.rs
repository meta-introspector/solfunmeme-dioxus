use std::collections::HashMap;
use dioxus::prelude::*;
//use dioxus_desktop::{LaunchBuilder, Config, WindowBuilder, LogicalSize};
use crate::model::memes::Expression;
//use crate::model::memes::{Controller, MemesAppState, LiftedExpression, ExpressionType};
//use crate::controller::Controller;
use crate::style::Styles;

// ============================================================================
// MAIN APPLICATION
// ============================================================================
// pub fn launch_app() {
//     LaunchBuilder::new()
//         .with_cfg(Config::new().with_window(
//             WindowBuilder::new()
//                 .with_title("SimpleExpr Manager")
//                 .with_inner_size(LogicalSize::new(1200.0, 800.0))
//         ))
//         .launch(App);
// }
#[derive(PartialEq, Clone)]
pub enum ExpressionType {
    #[allow(dead_code)]
    FromString,
    #[allow(dead_code)]
    BVar,
    #[allow(dead_code)]
    Sort,
    #[allow(dead_code)]
    Const,
    #[allow(dead_code)]
    Lambda,
    #[allow(dead_code)]
    Forall,
    #[allow(dead_code)]
    App,
}
#[derive(Props, PartialEq, Clone)]
pub struct MemesAppState {
    const_name: String,
    binder_name: String,
    current_name: String,
    current_input: String,
    current_description: String,
    current_tags: Vec<String>,
    search_query: String,
    implicit_binder: bool,
    expression_type: ExpressionType,
    expressions: HashMap<String, Expression>,
    filtered_expressions: Vec<String>,
}

impl MemesAppState {
    #[allow(dead_code)]
    pub fn default() -> Self {
        MemesAppState {
            expressions: HashMap::new(),
            filtered_expressions: vec![],
            current_description: "".to_string(),
            binder_name: "".to_string(),
            search_query: "".to_string(),
            const_name: "".to_string(),
            current_input: "".to_string(),
            current_tags: vec!["".to_string()],
            implicit_binder: false,
            expression_type: ExpressionType::Const,
            current_name: "FIXME".to_string(),
        }
    }
}

#[component]
pub fn Memes() -> Element {
    let state = use_signal(MemesAppState::default);

    rsx! {
        div {
            class: "app-container",
            style: "{Styles::app_container()}",
            Header {}
            InputSection { state }
            ExpressionList { state }
            VectorSpace { state }
            MemesFooter {}
        }
    }
}

// ============================================================================
// HEADER COMPONENT
// ============================================================================

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "app-header",
            style: "{Styles::header()}",
            h1 {
                style: "{Styles::header_title()}",
                "🧠 SimpleExpr Manager"
            }
            p {
                style: "{Styles::header_subtitle()}",
                "Lambda Calculus Expression Builder & Analyzer"
            }
        }
    }
}

// ============================================================================
// INPUT SECTION COMPONENT
// ============================================================================

#[derive(Props, PartialEq, Clone)]
pub struct StateProps {
    pub state: Signal<MemesAppState>,
}

#[component]
pub fn InputSection(props: StateProps) -> Element {
    let state = props.state;

    rsx! {
        section {
            style: "{Styles::section()}",

            h2 {
                style: "{Styles::section_title()}",
                "Create Expression"
            }

            ExpressionTypeSelector { state }
            ExpressionInputs { state }
            MetadataInputs { state }
            CreateButton { state }
            SearchInput { state }
        }
    }
}

#[component]
pub fn ExpressionTypeSelector(props: StateProps) -> Element {
    let mut state = props.state;

    rsx! {
        div {
            style: "{Styles::radio_group()}",

            for expr_type in [
                ExpressionType::FromString,
                ExpressionType::BVar,
                ExpressionType::Sort,
                ExpressionType::Const,
                ExpressionType::Lambda,
                ExpressionType::Forall,
                ExpressionType::App
            ] {
                label {
                    style: "{Styles::radio_label()}",
                    input {
                        r#type: "radio",
                        name: "expression_type",
                        checked: state.read().expression_type == expr_type,
                        onchange: move |_| {
                            state.with_mut(|s| s.expression_type = expr_type.clone());
                        },
                    }
                    span {
                        style: "{Styles::font_weight_medium()}",
                        {match expr_type {
                            ExpressionType::BVar => "BVar",
                            ExpressionType::Sort => "Sort",
                            ExpressionType::Const => "Const",
                            ExpressionType::App => "App",
                            ExpressionType::Lambda => "Lambda",
                            ExpressionType::Forall => "Forall",
                            ExpressionType::FromString => "FromString",
                        }}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ExpressionInputs(props: StateProps) -> Element {
    let mut state = props.state;

    rsx! {
            div {
                style: "{Styles::form_grid()}",

                // Dynamic inputs based on expression type
                {match state.read().expression_type {
                    ExpressionType::BVar => rsx! {
                        input {
                            style: "{Styles::input()}",
                            placeholder: "De Bruijn index (e.g., 0, 1, 2...)",
                            value: state.read().current_input.clone(),
                            oninput: move |evt| {
                                state.with_mut(|s| s.current_input = evt.value().clone());
                            },
                        }
                    },
                    ExpressionType::Sort => rsx! {
                        input {
                            style: "{Styles::input()}",
                            placeholder: "Universe level (e.g., 0, 1, 2...)",
                            value: state.read().current_input.clone(),
                            oninput: move |evt| {
                                state.with_mut(|s| s.current_input = evt.value().clone());
                            },
                        }
                    },
                    ExpressionType::Const => rsx! {
                        input {
                            style: "{Styles::input()}",
                            placeholder: "Constant name (e.g., Nat, Bool, f...)",
                            value: state.read().current_input.clone(),
                            oninput: move |evt| {
                                state.with_mut(|s| s.current_input = evt.value().clone());
                            },
                        }
                    },
            //ExpressionType::Const => rsx! {
                    //     input {
                    //         style: "{Styles::input()}",
                    //         placeholder: "Constant name (e.g., Nat, Bool, f...)",
                    //         value: state.read().const_name.clone(),
                    //         oninput: move |evt| {
                    //             state.with_mut(|s| s.const_name = evt.value().clone());
                    //         },
                    //     }
                    // },
                    ExpressionType::App => rsx! {
                        input {
                            style: "{Styles::input()}",
                            placeholder: "Function application (e.g., f x y...)",
                            value: state.read().current_input.clone(),
                            oninput: move |evt| {
                                state.with_mut(|s| s.current_input = evt.value().clone());
                            },
                        }
                    },
                    ExpressionType::Lambda => rsx! {
                        input {
                            style: "{Styles::input()}",
                            placeholder: "Lambda abstraction (e.g., λx. x)",
                            value: state.read().current_input.clone(),
                            oninput: move |evt| {
                                state.with_mut(|s| s.current_input = evt.value().clone());
                            },
                        }
                    },
              //               ExpressionType::Lambda | ExpressionType::Forall => rsx! {
                    //     div {
                    //         style: "display: grid; grid-template-columns: 1fr auto; gap: 10px; align-items: center;",
                    //         input {
                    //             style: "{Styles::input()}",
                    //             placeholder: "Binder name (e.g., x, y, z...)",
                    //             value: state.read().binder_name.clone(),
                    //             oninput: move |evt| {
                    //                 state.with_mut(|s| s.binder_name = evt.value().clone());
                    //             },
                    //         }
                    //         label {
                    //             style: "{Styles::checkbox_label()}",
                    //             input {
                    //                 r#type: "checkbox",
                    //                 checked: state.read().implicit_binder,
                    //                 onchange: move |evt| {
                    //                     state.with_mut(|s| s.implicit_binder = evt.value().parse().unwrap_or(false));
                    //                 },
                    //             }
                    //             "Implicit"
                    //         }
                    //     }
                    // },
                    ExpressionType::Forall => rsx! {
                        input {
                            style: "{Styles::input()}",
                            placeholder: "Forall quantification (e.g., ∀x. P x)",
                            value: state.read().current_input.clone(),
                            oninput: move |evt| {
                                state.with_mut(|s| s.current_input = evt.value().clone());
                            },
                        }
                    },
                    ExpressionType::FromString => rsx! {
                        input {
                            style: "{Styles::input()}",
                            placeholder: "Enter expression as string...",
                            value: state.read().current_input.clone(),
                            oninput: move |evt| {
                                state.with_mut(|s| s.current_input = evt.value().clone());
                            },
                        }
                    },


                    // _ => rsx! {
                    //     textarea {
                    //         style: "{Styles::textarea()}",
                    //         placeholder: "Expression content...",
                    //         value: state.read().current_input.clone(),
                    //         oninput: move |evt| {
                    //             state.with_mut(|s| s.current_input = evt.value().clone());
                    //         },
                    //     }
                    // }
                }
            }
        }
    }
}

#[component]
pub fn MetadataInputs(props: StateProps) -> Element {
    let mut state = props.state;

    rsx! {
        div {
            style: "{Styles::form_grid()}",

            input {
                style: "{Styles::input()}",
                placeholder: "Expression name...",
                value: state.read().current_name.clone(),
                oninput: move |evt| {
                    state.with_mut(|s| s.current_name = evt.value().clone());
                },
            }

            textarea {
                style: "padding: 10px; border: 2px solid #ddd; border-radius: 8px; font-size: 16px; min-height: 50px; resize: vertical;",
                placeholder: "Description...",
                value: state.read().current_description.clone(),
                oninput: move |evt| {
                    state.with_mut(|s| s.current_description = evt.value().clone());
                },
            }

            input {
                style: "{Styles::input()}",
                placeholder: "Tags (comma-separated)...",
                value: state.read().current_tags.join(","),
                oninput: move |evt| {
                    state.with_mut(|s| s.current_tags = evt.value()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect());
                },
            }
        }
    }
}

pub struct Controller {}
impl Controller {
    #[allow(dead_code)]
    pub fn delete_expression(_state: MemesAppState, _id: String) {}
    #[allow(dead_code)]
    pub fn create_expression_from_type(_state: MemesAppState) -> Option<Expression> {
        Some(Expression {
            astring: Signal::new("".to_string()),
        })
    }
}
#[component]
pub fn CreateButton(props: StateProps) -> Element {
    #[allow(unused_variables)]
    //let mut state = props.state;
    let state = props.state;

    rsx! {
        button {
            style: "{Styles::primary_button()}",
            //onclick: move |_| {
               // let current_state = state.read().clone();

            //     if let Some(expr) = Controller::create_expression_from_type(current_state.clone()) {
            //         // let tags: Vec<String> = current_state.current_tags
            //         //     .split(',')
            //         //     .map(|s| s.trim().to_string())
            //         //     .filter(|s| !s.is_empty())
            //         //     .collect();
            //         // Controller::add_expression(
            //         //     &mut state.write(),
            //         //     expr,
            //         //     current_state.current_name,
            //         //     current_state.current_description,
            //         //     tags
            //         // );
            //     }
            // },
            "🚀 Create Expression"
        }
    }
}

#[component]
pub fn SearchInput(props: StateProps) -> Element {
    #[allow(unused_variables)]
    //let mut state = props.state;
    let state = props.state;

    rsx! {
        div {
            "fixme"
        }
    }
    // rsx! {
    //     div {
    //         style: "margin-top: 20px;",
    //         input {
    //             style: "{Styles::search_input()}",
    //             placeholder: "Search expressions...",
    //             value: state.read().search_query.clone(),
    //             oninput: move |evt| {
    //                 let query = evt.value().clone();
    //                 state.with_mut(|s| s.search_query = query.clone());
    //                 Controller::search_expressions(&mut state.write(), query);
    //             },
    //         }
    //     }
    //}
}
#[allow(dead_code)]
pub fn liftexpression(expr: Expression) -> LiftedExpression {
    LiftedExpression {
        expr,
        id: "expr.id".to_string(),
        name: "expr.name".to_string(),
        description: "expr.expr".to_string(),
    }
}

// ============================================================================
// EXPRESSION LIST COMPONENT
// ============================================================================

#[component]
pub fn ExpressionList(props: StateProps) -> Element {
    let state = props.state;

    let expression_ids = if state.read().search_query.is_empty() {
        state.read().expressions.keys().cloned().collect::<Vec<_>>()
    } else {
        state.read().filtered_expressions.clone()
    };

    rsx! {
        section {
            style: "margin-bottom: 30px;",
            h2 {
                style: "{Styles::text_white()} {Styles::margin_bottom(\"20px\")}",
                "📝 Expression Library ({expression_ids.len()})"
            }
            div {
                style: "{Styles::grid_auto_fill(\"400px\")}",
                for id in expression_ids {
                    if let Some(expr) = state.read().expressions.get(&id) {
                        ExpressionCard { expression: liftexpression(expr.clone()), state }
                    }
                }
            }
        }
    }
}

// ============================================================================
// EXPRESSION CARD COMPONENT
// ============================================================================

//complexity

// #[derive(Props, PartialEq, Clone, Debug)]
// pub struct LiftedExpression2 {

// }

#[derive(Props, PartialEq, Clone, Debug)]
pub struct LiftedExpression {
    id: String,
    name: String,
    description: String,
    expr: Expression,
}
#[derive(Props, PartialEq, Clone)]
pub struct ExpressionCardProps {
    pub expression: LiftedExpression,
    pub state: Signal<MemesAppState>,
}

#[component]
pub fn ExpressionCard(props: ExpressionCardProps) -> Element {
    let expr = props.expression.clone();
    let state = props.state;
    //let expr_id = expr.id.clone();

    rsx! {
        div {
            style: "{Styles::card_hover()}",

            CardHeader { expression: expr.clone(), state }
            CodeDisplay { expression: expr.clone() }
            ExpressionMetadata { expression: expr.clone() }
            SimilaritySection { expression: expr.clone(), state }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ExpressionProps {
    pub expression: LiftedExpression,
}

#[derive(Props, PartialEq, Clone)]
pub struct ExpressionWithStateProps {
    pub expression: LiftedExpression,
    pub state: Signal<MemesAppState>,
}
#[component]
pub fn CardHeader(props: ExpressionWithStateProps) -> Element {
    let expr = props.expression.clone();
    //let mut state = props.state;
    //let expr_id = expr.id.clone();
    rsx! {
        div {
            style: "{Styles::flex_between()} {Styles::margin_bottom(\"15px\")}",
            h3 {
                style: "margin: 0; color: #333; font-size: 1.2rem;",
                "{expr.name}"
            }
            button {
                style: "{Styles::delete_button()}",
                onclick: move |_| {
                   // Controller::delete_expression(state.write(), expr_id.clone());
                },
                "×"
            }
        }
    }
}

#[component]
pub fn CodeDisplay(props: ExpressionProps) -> Element {
    let expr = props.expression;

    rsx! {
        div {
            style: "{Styles::margin_bottom(\"15px\")}",
            div {
                style: "{Styles::code_block()}",
                code { "{expr.expr:?}" }
            }

            if !expr.description.is_empty() {
                p {
                    style: "margin: 10px 0; color: #666; line-height: 1.4;",
                    "{expr.description}"
                }
            }
        }
    }
}

#[component]
pub fn ExpressionMetadata(props: ExpressionProps) -> Element {
    #[allow(unused_variables)]
    let expr = props.expression;

    rsx! {
         div {
    //         style: "display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin-bottom: 15px; font-size: 14px;",
    //         div {
    //             strong { "Type: " }
    //             span {
    //                 style: "{Styles::text_accent()}",
    //                 {match &expr.expr {
    //                     ExpressionType::BVar { .. } => "BVar",
    //                     ExpressionType::Sort { .. } => "Sort",
    //                     ExpressionType::Const { .. } => "Const",
    //                     ExpressionType::App { .. } => "App",
    //                     ExpressionType::Lam { .. } => "Lambda",
    //                     ExpressionType::ForallE { .. } => "Forall",
    //                 }}
    //             }
    //         }
    //         div {
    //             strong { "Complexity: " }
    //             span {
    //                 style: "{Styles::text_accent()}",
    //                 "{expr.expr.complexity():.1}"
    //             }
    //         }
         }
     }
}

#[component]
pub fn SimilaritySection(props: ExpressionWithStateProps) -> Element {
    #[allow(unused_variables)]
    let expr = props.expression;
    #[allow(unused_variables)]
    let state = props.state;

    //let similar_expressions = Controller::get_similar_expressions(&state.read(), &expr, 3);

    rsx! {
    "SimilaritySection"
        // if !similar_expressions.is_empty() {
             div {
        //         style: "border-top: 1px solid #eee; padding-top: 15px;",
        //         h4 {
        //             style: "margin: 0 0 10px 0; font-size: 14px; color: #666;",
        //             "Similar Expressions:"
        //         }
        //         div {
        //             style: "display: flex; flex-wrap: wrap; gap: 8px;",
        //             //for (similar_id, similarity) in similar_expressions {
        //                 //if let Some(similar_expr) = state.read().expressions.get(&similar_id) {
        //                     // rsx! {
        //                     //     span {
        //                     //         style: "background: #f0f0f0; padding: 4px 8px; border-radius: 4px; font-size: 12px;",
        //                     //         {similar_expr.name} ({similarity:.2})
        //                     //     }
        //                     // }
        //                 //}
        //             //}

        //         }
        //     }
         }
    }
}

// ============================================================================
// VECTOR SPACE VISUALIZATION
// ============================================================================

#[component]
pub fn VectorSpace(props: StateProps) -> Element {
    let state = props.state;

    rsx! {
        section {
            style: "{Styles::section()}",
            h2 {
                style: "{Styles::section_title()}",
                "🎯 Vector Space Analysis"
            }

            div {
                style: "text-align: center; padding: 40px; color: #666;",
                p { "Vector space visualization coming soon..." }
                p {
                    style: "font-size: 14px; margin-top: 10px;",
                    "Total expressions: {state.read().expressions.len()}"
                }
            }
        }
    }
}

// ============================================================================
// FOOTER COMPONENT
// ============================================================================

#[component]
pub fn MemesFooter() -> Element {
    rsx! {
        footer {
            style: "text-align: center; padding: 20px; color: rgba(255,255,255,0.7); font-size: 14px;",
            p { "Built with 🍄 Solfunmeme & 🦀 Rust & Dioxus" }
        }
    }
}

// }}
//             value: state.read().current_input.clone(),
//             oninput: move |evt| {
//                 state.with_mut(|s| s.current_input = evt.value().clone());
//             },
//     },
//     SimpleExpr::Sort => rsx! {
//         input {
//             // style: Styles::input(),
//             // placeholder: "Universe level (e.g., 0, 1, 2...)",
//             // value: state.read().current_input.clone(),
//             // oninput: move |evt| {
//             //     state.with_mut(|s| s.level_input = evt.value().clone());
//             // },
//         }
//     },
