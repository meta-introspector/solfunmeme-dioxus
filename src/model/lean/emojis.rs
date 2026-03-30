use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

use crate::model::{lean::level::level_to_string, simple_expr::SimpleExprType};

// Define Rust structs to match the JSON schema
#[derive(Debug, Deserialize, Serialize)]
pub struct AsyncConstB<'a> {
    kind: String,
    #[serde(rename = "cnstInfB")]
    cnst_inf_b: CnstInfB<'a>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CnstInfB<'a> {
    sig: Sig<'a>,
    name: String,
    #[serde(rename = "levelParams")]
    level_params: Vec<String>,
    kind: ConstantKind,
    #[serde(rename = "cnstInf")]
    cnst_inf: Option<CnstInf<'a>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Type {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sig<'a> {
    #[serde(rename = "type")]
    typ: SimpleExprType<'a>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConstantKind {
    value: String,
    kind: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CnstInf<'a> {
    #[serde(rename = "type")]
    typ: Type,
    #[serde(rename = "numParams")]
    num_params: u32,
    #[serde(rename = "numMotives")]
    num_motives: u32,
    #[serde(rename = "numMinors")]
    num_minors: u32,
    #[serde(rename = "numIndices")]
    num_indices: u32,
    name: String,
    #[serde(rename = "levelParams")]
    level_params: Vec<String>,
    kind: String,
    k: bool,
    #[serde(rename = "isUnsafe")]
    is_unsafe: bool,
    all: Vec<String>,
    #[serde(rename = "Rules")]
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rule<'a> {
    rhs: SimpleExprType<'a>,
    nfields: u32,
    name: String,
    kind: String,
}
// Function to convert a Type node to an emoji string
#[allow(dead_code)]
pub fn type_to_emoji(
    typ: &SimpleExprType,
    depth: usize,
    emoji_map: &HashMap<&str, &str>,
) -> String {
    let indent = "  ".repeat(depth);
    match typ {
        SimpleExprType::ForallE {
            forbndr_typ_b,
            forbndr_typ,
            forbd_b,
            forbd,
            binder_name,
            binder_info,
        } => {
            let typ_str = match (forbndr_typ_b, forbndr_typ) {
                (Some(t), _) | (_, Some(t)) => type_to_emoji(t, depth + 1, emoji_map),
                _ => String::new(),
            };
            let forbd_str = match (forbd_b, forbd) {
                (Some(t), _) | (_, Some(t)) => type_to_emoji(t, depth + 1, emoji_map),
                _ => String::new(),
            };
            format!(
                "{}{} {} ({}: {})\n{}\n{}",
                indent,
                emoji_map.get("forallE").unwrap_or(&"∀"),
                binder_name,
                binder_info,
                typ_str.trim_end(),
                forbd_str.trim_end(),
                indent
            )
        }
        SimpleExprType::Const { levels, decl_name } => {
            let levels_str = levels
                .iter()
                .map(level_to_string)
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{}{} {} [{}]",
                indent,
                emoji_map.get("const").unwrap_or(&"🔖"),
                decl_name,
                levels_str
            )
        }
        SimpleExprType::Sort { level } => format!(
            "{}{} {}",
            indent,
            emoji_map.get("sort").unwrap_or(&"📏"),
            level_to_string(level)
        ),
        SimpleExprType::BVar { .. } => {
            format!("{}{}", indent, emoji_map.get("bvar").unwrap_or(&"📍"))
        }
        SimpleExprType::App {
            //func,
            arg,
            fn_expr,
        } => format!(
            "{}{} (\n{}\n{}\n{})",
            indent,
            emoji_map.get("app").unwrap_or(&"➡️"),
            type_to_emoji(fn_expr, depth + 1, emoji_map),
            type_to_emoji(arg, depth + 1, emoji_map),
            indent
        ),
        SimpleExprType::Lam {
            binder_name,
            binder_info,
            binder_type,
            body,
        } => {
            let typ_str = type_to_emoji(binder_type, depth + 1, emoji_map);
            let body_str = type_to_emoji(body, depth + 1, emoji_map);
            format!(
                "{}{} {} ({}: {})\n{}\n{}",
                indent,
                emoji_map.get("lam").unwrap_or(&"λ"),
                binder_name,
                binder_info,
                typ_str.trim_end(),
                body_str.trim_end(),
                indent
            )
        }
    }
}

// Function to convert a Rule to an emoji string
#[allow(dead_code)]
pub fn rule_to_emoji<'a>(rule: &Rule<'a>, depth: usize, emoji_map: &HashMap<&str, &str>) -> String {
    let indent = "  ".repeat(depth);
    format!(
        "{}📋 {} (fields: {})\n{}\n",
        indent,
        rule.name,
        rule.nfields,
        type_to_emoji(&rule.rhs, depth + 1, emoji_map)
    )
}

// Main translation function
#[allow(dead_code)]
pub fn json_to_emoji<'a>(json_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Define emoji mappings
    let mut emoji_map = HashMap::new();
    emoji_map.insert("forallE", "∀");
    emoji_map.insert("const", "🔖");
    emoji_map.insert("sort", "📏");
    emoji_map.insert("bvar", "📍");
    emoji_map.insert("app", "➡️");
    emoji_map.insert("lam", "λ");
    emoji_map.insert("SimpleExpr", "📖");

    // Parse JSON
    let async_const: AsyncConstB<'a> = serde_json::from_str(json_str)?;

    // Start with the root node
    let mut result = format!(
        "{} {}\n",
        emoji_map.get("SimpleExpr").unwrap_or(&"📖"),
        async_const.cnst_inf_b.name
    );

    // Process the signature's type
    result += &type_to_emoji(&async_const.cnst_inf_b.sig.typ, 1, &emoji_map);

    // Process the rules if present
    if let Some(cnst_inf) = &async_const.cnst_inf_b.cnst_inf {
        result += "\n  📜 Rules:\n";
        for rule in &cnst_inf.rules {
            result += &rule_to_emoji(rule, 2, &emoji_map);
        }
    }

    Ok(result)
}

// fn main() {
//     // Use the full JSON (trimmed here for brevity; use full JSON in practice)
//     let json_str = include_str!("full.json"); // Assume JSON is saved in a file
//     match json_to_emoji(json_str) {
//         Ok(emoji_str) => println!("{}", emoji_str),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }
