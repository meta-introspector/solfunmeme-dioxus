use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use nalgebra::DVector;

// ============================================================================
// CORE TYPES - Lambda Calculus Expression System
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub value: u64,
}

impl Level {
    pub fn new(value: u64) -> Self {
        Level { value }
    }
    
    pub fn zero() -> Self {
        Level { value: 0 }
    }
    
    pub fn succ(&self) -> Self {
        Level { value: self.value + 1 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new(name: String) -> Self {
        Name { name }
    }
    
    pub fn anonymous() -> Self {
        Name { name: "_".to_string() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinderInfo {
    pub implicit: bool,
    pub strict: bool,
}

impl BinderInfo {
    pub fn default() -> Self {
        BinderInfo {
            implicit: false,
            strict: false,
        }
    }
    
    pub fn implicit() -> Self {
        BinderInfo {
            implicit: true,
            strict: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimpleExpr {
    BVar { index: u64 },
    Sort { level: Level },
    Const { name: Name, levels: Vec<Level> },
    App { func: Box<SimpleExpr>, arg: Box<SimpleExpr> },
    Lam {
        binder_name: Name,
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfo,
    },
    ForallE {
        binder_name: Name,
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfo,
    },
}

impl SimpleExpr {
    // Constructor methods
    pub fn bvar(index: u64) -> Self {
        SimpleExpr::BVar { index }
    }

    pub fn sort(level: Level) -> Self {
        SimpleExpr::Sort { level }
    }

    pub fn const_expr(name: Name, levels: Vec<Level>) -> Self {
        SimpleExpr::Const { name, levels }
    }

    pub fn app(func: SimpleExpr, arg: SimpleExpr) -> Self {
        SimpleExpr::App {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    pub fn lam(name: Name, binder_type: SimpleExpr, body: SimpleExpr, info: BinderInfo) -> Self {
        SimpleExpr::Lam {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }

    pub fn forall_e(name: Name, binder_type: SimpleExpr, body: SimpleExpr, info: BinderInfo) -> Self {
        SimpleExpr::ForallE {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }
    
    // Display methods
    pub fn to_string(&self) -> String {
        match self {
            SimpleExpr::BVar { index } => format!("#{}", index),
            SimpleExpr::Sort { level } => format!("Sort({})", level.value),
            SimpleExpr::Const { name, levels } => {
                if levels.is_empty() {
                    name.name.clone()
                } else {
                    format!("{}.{{{}}}", name.name, 
                           levels.iter().map(|l| l.value.to_string()).collect::<Vec<_>>().join(", "))
                }
            },
            SimpleExpr::App { func, arg } => {
                format!("({} {})", func.to_string(), arg.to_string())
            },
            SimpleExpr::Lam { binder_name, binder_type, body, binder_info } => {
                let implicit_marker = if binder_info.implicit { "{" } else { "(" };
                let implicit_end = if binder_info.implicit { "}" } else { ")" };
                format!("λ {}{} : {}{} → {}", 
                       implicit_marker, binder_name.name, binder_type.to_string(), implicit_end,
                       body.to_string())
            },
            SimpleExpr::ForallE { binder_name, binder_type, body, binder_info } => {
                let implicit_marker = if binder_info.implicit { "{" } else { "(" };
                let implicit_end = if binder_info.implicit { "}" } else { ")" };
                format!("∀ {}{} : {}{}, {}", 
                       implicit_marker, binder_name.name, binder_type.to_string(), implicit_end,
                       body.to_string())
            }
        }
    }
    
    // Calculate complexity based on AST depth and node count
    pub fn complexity(&self) -> f64 {
        match self {
            SimpleExpr::BVar { .. } | SimpleExpr::Sort { .. } | SimpleExpr::Const { .. } => 1.0,
            SimpleExpr::App { func, arg } => 1.0 + func.complexity() + arg.complexity(),
            SimpleExpr::Lam { binder_type, body, .. } => 2.0 + binder_type.complexity() + body.complexity(),
            SimpleExpr::ForallE { binder_type, body, .. } => 2.0 + binder_type.complexity() + body.complexity(),
        }
    }
    
    // Vectorize expression for similarity calculations
    pub fn vectorize(&self) -> DVector<f64> {
        let mut features = vec![0.0; 10]; // Feature vector
        
        // Node type features
        match self {
            SimpleExpr::BVar { index } => {
                features[0] = 1.0; // BVar indicator
                features[5] = *index as f64; // Index value
            },
            SimpleExpr::Sort { level } => {
                features[1] = 1.0; // Sort indicator
                features[6] = level.value as f64; // Level value
            },
            SimpleExpr::Const { levels, .. } => {
                features[2] = 1.0; // Const indicator
                features[7] = levels.len() as f64; // Number of levels
            },
            SimpleExpr::App { func, arg } => {
                features[3] = 1.0; // App indicator
                let func_complexity = func.complexity();
                let arg_complexity = arg.complexity();
                features[8] = func_complexity + arg_complexity;
            },
            SimpleExpr::Lam { binder_info, .. } => {
                features[4] = 1.0; // Lam indicator
                features[9] = if binder_info.implicit { 1.0 } else { 0.0 };
            },
            SimpleExpr::ForallE { binder_info, .. } => {
                features[4] = 2.0; // ForallE indicator (distinct from Lam)
                features[9] = if binder_info.implicit { 1.0 } else { 0.0 };
            }
        }
        
        DVector::from_vec(features)
    }
}

// ============================================================================
// LIFTED EXPRESSION - Wrapper for GUI Management
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LiftedExpression {
    pub id: String,
    pub expr: SimpleExpr,
    pub name: String,
    pub description: String,
    pub lifted_at: String,
    pub vector_representation: Vec<f64>,
    pub tags: Vec<String>,
}

impl LiftedExpression {
    pub fn new(expr: SimpleExpr, name: String, description: String, tags: Vec<String>) -> Self {
        let vector_representation = expr.vectorize().data.as_vec().clone();
        
        LiftedExpression {
            id: Uuid::new_v4().to_string(),
            expr,
            name,
            description,
            lifted_at: "2024-01-01T00:00:00Z".to_string(), // Would use chrono in real app
            vector_representation,
            tags,
        }
    }
}

// ============================================================================
// APPLICATION STATE
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub expressions: HashMap<String, LiftedExpression>,
    pub current_input: String,
    pub current_name: String,
    pub current_description: String,
    pub current_tags: String,
    pub expression_type: ExpressionType,
    pub search_query: String,
    pub filtered_expressions: Vec<String>,
    
    // For building complex expressions
    pub binder_name: String,
    pub index_input: String,
    pub level_input: String,
    pub const_name: String,
    pub implicit_binder: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    BVar,
    Sort,
    Const,
    App,
    Lambda,
    Forall,
    FromString,
}

impl Default for ExpressionType {
    fn default() -> Self {
        ExpressionType::FromString
    }
}