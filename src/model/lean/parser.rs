// use nom::{
//     branch::alt,
//     bytes::complete::{tag, take_while1},
//     character::complete::{char, digit1, newline, space0, space1},
//     combinator::{map, opt},
//     multi::{many0, separated_list0},
//     sequence::{delimited, pair},
// //    sequence::{delimited, pair, preceded, terminated, tuple},
//     IResult,
// };
// //use std::collections::HashMap;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Type {}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Rule {
//     name: String,
//     nfields: i8,
//     //    rhs: Option<Rule>
//     rhs: String
// }
// #[derive(Debug, Clone, PartialEq)]
// pub struct Root {
//     name : String
// }
// #[derive(Debug, Clone, PartialEq)]
// pub struct Program {
// //--    name : String,
// root:  Root,
// types:  Vec<Type>,
// rules  : String
// }
// // Parser for the entire program


// // Parse ForallE (e.g., "  ∀ t (default: 🔖 SimpleExpr [u_1,u_2])")
// fn parse_forall_e(input: &str) -> IResult<&str, Type> {
//     let (input, indent) = parse_indent(input)?;
//     let (input, _) = tag("∀")(input)?;
//     let (input, _) = space1(input)?;
//     let (input, binder_name) = parse_identifier(input)?;
//     let (input, _) = space0(input)?;
//     let (input, binder_info) = delimited(
//         char('('),
//         parse_identifier,
//         pair(char(':'), space0),
//     )(input)?;
//     let (input, binder_type) = parse_type(input)?;
//     let (input, _) = char(')')(input)?;
//     let (input, _) = newline(input)?;
//     let (input, body) = parse_type(input)?;
//     let (input, _) = newline(input)?;
//     let (input, _) = tag(indent)(input)?;
//     Ok((
//         input,
//         Type::ForallE {
//             binder_name,
//             binder_info,
//             binder_type: Box::new(binder_type),
//             body: Box::new(body),
//         },
//     ))
// }

// // Parse Const (e.g., "  🔖 SimpleExpr [u_1,u_2]")
// fn parse_const(input: &str) -> IResult<&str, Type> {
//     let (input, indent) = parse_indent(input)?;
//     let (input, _) = tag("🔖")(input)?;
//     let (input, _) = space1(input)?;
//     let (input, decl_name) = parse_identifier(input)?;
//     let (input, _) = space0(input)?;
//     let (input, levels) = delimited(
//         char('['),
//         separated_list0(tag(","), parse_identifier),
//         char(']'),
//     )(input)?;
//     Ok((
//         input,
//         Type::Const {
//             decl_name,
//             levels,
//         },
//     ))
// }

// // Parse Sort (e.g., "  📏 u")
// fn parse_sort(input: &str) -> IResult<&str, Type> {
//     let (input, indent) = parse_indent(input)?;
//     let (input, _) = tag("📏")(input)?;
//     let (input, _) = space1(input)?;
//     let (input, level) = parse_identifier(input)?;
//     Ok((input, Type::Sort { level }))
// }

// // Parse Bvar (e.g., "  📍")
// fn parse_bvar(input: &str) -> IResult<&str, Type> {
//     let (input, indent) = parse_indent(input)?;
//     let (input, _) = tag("📍")(input)?;
//     Ok((input, Type::Bvar))
// }

// // Parse App (e.g., "  ➡️ (\n    <type>\n    <type>\n  )")
// fn parse_app(input: &str) -> IResult<&str, Type> {
//     let (input, indent) = parse_indent(input)?;
//     let (input, _) = tag("➡️")(input)?;
//     let (input, _) = space0(input)?;
//     let (input, _) = char('(')(input)?;
//     let (input, _) = newline(input)?;
//     let (input, func) = parse_type(input)?;
//     let (input, _) = newline(input)?;
//     let (input, arg) = parse_type(input)?;
//     let (input, _) = newline(input)?;
//     let (input, _) = tag(indent)(input)?;
//     let (input, _) = char(')')(input)?;
//     Ok((
//         input,
//         Type::App {
//             func: Box::new(func),
//             arg: Box::new(arg),
//         },
//     ))
// }

// // Parse Lam (e.g., "  λ t (default: 🔖 SimpleExpr [u_1,u_2])")
// fn parse_lam(input: &str) -> IResult<&str, Type> {
//     let (input, indent) = parse_indent(input)?;
//     let (input, _) = tag("λ")(input)?;
//     let (input, _) = space1(input)?;
//     let (input, binder_name) = parse_identifier(input)?;
//     let (input, _) = space0(input)?;
//     let (input, binder_info) = delimited(
//         char('('),
//         parse_identifier,
//         pair(char(':'), space0),
//     )(input)?;
//     let (input, binder_type) = parse_type(input)?;
//     let (input, _) = char(')')(input)?;
//     let (input, _) = newline(input)?;
//     let (input, body) = parse_type(input)?;
//     let (input, _) = newline(input)?;
//     let (input, _) = tag(indent)(input)?;
//     Ok((
//         input,
//         Type::Lam {
//             binder_name,
//             binder_info,
//             binder_type: Box::new(binder_type),
//             body: Box::new(body),
//         },
//     ))
// }

// // Parse RulesSection (e.g., "📜 Rules:\n  📋 SimpleExpr.bvar (fields: 2)\n...")
// fn parse_rules_section(input: &str) -> IResult<&str, Vec<Rule>> {
//     let (input, _) = tag("📜")(input)?;
//     let (input, _) = space1(input)?;
//     let (input, _) = tag("Rules:")(input)?;
//     let (input, _) = newline(input)?;
//     many0(parse_rule)(input)
// }

// // Parse Rule (e.g., "  📋 SimpleExpr.bvar (fields: 2)\n    <type>")
// fn parse_rule(input: &str) -> IResult<&str, Rule> {
//     let (input, indent) = parse_indent(input)?;
//     let (input, _) = tag("📋")(input)?;
//     let (input, _) = space1(input)?;
//     let (input, name) = parse_identifier(input)?;
//     let (input, _) = space1(input)?;
//     let (input, nfields) = delimited(
//         tag("(fields:"),
//         map(digit1, |s: &str| s.parse::<u32>().unwrap()),
//         char(')'),
//     )(input)?;
//     let (input, _) = newline(input)?;
//     let (input, rhs) = parse_type(input)?;
//     let (input, _) = newline(input)?;
//     Ok((input, Rule { name, nfields, rhs }))
// }

// // Parse Identifier (e.g., "SimpleExpr", "u_1", "default")
// fn parse_identifier(input: &str) -> IResult<&str, String> {
//     map(
//         take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '.'),
//         |s: &str| s.to_string(),
//     )(input)
// }

// // Parse Indent (e.g., "  ", "    ")
// fn parse_indent(input: &str) -> IResult<&str, &str> {
//     take_while1(|c: char| c == ' ')(input)
// }


// // Parse the root (e.g., "📖 SimpleExpr.rec")
// fn parse_root(input: &str) -> IResult<&str, Root> {
//     let (input, _) = tag("📖")(input)?;
//     let (input, _) = space1(input)?;
//     let (input, name) = parse_identifier(input)?;
//     let (input, _) = newline(input)?;
//     Ok((input, Root { name }))
// }

// // Parse a type (ForallE, Const, Sort, Bvar, App, Lam)
// fn parse_type(input: &str) -> IResult<&str, Type> {
//     alt((
//         parse_forall_e,
//         parse_const,
//         parse_sort,
//         parse_bvar,
//         parse_app,
//         parse_lam,
//     ))(input)
// }


// fn parse_program(input: &str) -> IResult<&str, Program> {
//     let (input, root) = parse_root(input)?;
//     let (input, types) = many0(parse_type)(input)?;
//     let (input, rules) = opt(parse_rules_section)(input)?;
//     Ok((input, Program { root, types, rules }))
// }

// fn main() {
//     // Example emoji string (trimmed for brevity)
//     let input = r#"
// 📖 SimpleExpr.rec
//   ∀ t (default: 🔖 SimpleExpr [u_1,u_2,u_3,u_4,u_5,u_6,u_7,u_8])
//     ∀ motive (implicit: 📏 u)
//       📍
// 📜 Rules:
//     📋 SimpleExpr.bvar (fields: 2)
//       λ t (default: 🔖 SimpleExpr [u_1,u_2,u_3,u_4,u_5,u_6,u_7,u_8])
//         📍
// "#;

//     match parse_program(input) {
//         Ok((remaining, program)) => {
//             println!("Parsed: {:#?}", program);
//             println!("Remaining: {:?}", remaining);
//             // Optionally serialize to JSON
//             let json = serde_json::to_string_pretty(&program).unwrap();
//             println!("JSON: {}", json);
//         }
//         Err(e) => println!("Error: {:?}", e),
//     }
// }
