use crate::types::*;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_text_index(&self, index: &crate::documentation_index::DocumentationIndex) -> String {
        let mut output = String::new();

        output.push_str("📚 Solfunmeme-Dioxus Documentation Index\n");
        output.push_str("=======================================\n\n");

        // Systems Design Documents
        output.push_str("🎯 Systems Design Documents\n");
        output.push_str("=========================\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::SystemsDesign) {
                output.push_str(&format!("• {} - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Architecture Documentation
        output.push_str("🏗️ Architecture Documentation\n");
        output.push_str("============================\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::Architecture) {
                output.push_str(&format!("• {} - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // UML Diagrams
        output.push_str("📊 UML Diagrams\n");
        output.push_str("==============\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::UMLDiagram) {
                output.push_str(&format!("• {} - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Ontologies
        output.push_str("🧠 Ontologies\n");
        output.push_str("============\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::Ontology) {
                output.push_str(&format!("• {} - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Workflow & Tools
        output.push_str("⚙️ Workflow & Tools\n");
        output.push_str("==================\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::Workflow) {
                output.push_str(&format!("• {} - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Code Implementation
        output.push_str("💻 Code Implementation\n");
        output.push_str("====================\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::Code) {
                output.push_str(&format!("• {} - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Navigation Patterns
        output.push_str("🎯 Navigation Patterns\n");
        output.push_str("=====================\n\n");

        for (user_type, pattern) in index.get_navigation_patterns() {
            output.push_str(&format!("For {} users:\n", user_type));
            for step in &pattern.steps {
                output.push_str(&format!("{}. {} - {}\n", step.order, step.document, step.purpose));
                output.push_str(&format!("   {}\n\n", step.description));
            }
        }

        // Cross-Reference Summary
        output.push_str("🔗 Cross-Reference Summary\n");
        output.push_str("=========================\n");
        output.push_str("• Systems Design -> Architecture -> Implementation -> Code\n");
        output.push_str("• Context -> Container -> Component -> Deployment\n");
        output.push_str("• Use Cases -> Data Flow -> Sequence -> Execution\n");
        output.push_str("• Ontologies -> Semantic Models -> Component Relationships\n\n");

        output.push_str("📖 For detailed cross-references, see: docs/uml/CROSS_REFERENCES.md\n");
        output.push_str("📊 For visual relationships, see: docs/uml/09-cross-references.puml\n");

        output
    }

    pub fn generate_json_index(&self, index: &crate::documentation_index::DocumentationIndex) -> String {
        let mut json_data = serde_json::Map::new();

        // Documents
        let documents: Vec<serde_json::Value> = index
            .get_documents()
            .iter()
            .map(|doc| {
                let mut doc_obj = serde_json::Map::new();
                doc_obj.insert("path".to_string(), serde_json::Value::String(doc.path.clone()));
                doc_obj.insert("title".to_string(), serde_json::Value::String(doc.title.clone()));
                doc_obj.insert("category".to_string(), serde_json::Value::String(format!("{:?}", doc.category)));
                doc_obj.insert("description".to_string(), serde_json::Value::String(doc.description.clone()));
                
                let references: Vec<serde_json::Value> = doc.references.iter().map(|ref_| {
                    let mut ref_obj = serde_json::Map::new();
                    ref_obj.insert("target".to_string(), serde_json::Value::String(ref_.target.clone()));
                    ref_obj.insert("type".to_string(), serde_json::Value::String(format!("{:?}", ref_.reference_type)));
                    ref_obj.insert("description".to_string(), serde_json::Value::String(ref_.description.clone()));
                    ref_obj.insert("bidirectional".to_string(), serde_json::Value::Bool(ref_.bidirectional));
                    serde_json::Value::Object(ref_obj)
                }).collect();
                
                doc_obj.insert("references".to_string(), serde_json::Value::Array(references));
                serde_json::Value::Object(doc_obj)
            })
            .collect();

        json_data.insert("documents".to_string(), serde_json::Value::Array(documents));

        // Categories
        let categories: Vec<serde_json::Value> = index
            .get_categories()
            .iter()
            .map(|(_name, cat)| {
                let mut cat_obj = serde_json::Map::new();
                cat_obj.insert("name".to_string(), serde_json::Value::String(cat.name.clone()));
                cat_obj.insert("description".to_string(), serde_json::Value::String(cat.description.clone()));
                cat_obj.insert("color".to_string(), serde_json::Value::String(cat.color.clone()));
                
                let docs: Vec<serde_json::Value> = cat.documents.iter().map(|d| serde_json::Value::String(d.clone())).collect();
                cat_obj.insert("documents".to_string(), serde_json::Value::Array(docs));
                
                serde_json::Value::Object(cat_obj)
            })
            .collect();

        json_data.insert("categories".to_string(), serde_json::Value::Array(categories));

        // Navigation Patterns
        let patterns: Vec<serde_json::Value> = index
            .get_navigation_patterns()
            .iter()
            .map(|(_user_type, pattern)| {
                let mut pattern_obj = serde_json::Map::new();
                pattern_obj.insert("user_type".to_string(), serde_json::Value::String(pattern.user_type.clone()));
                pattern_obj.insert("description".to_string(), serde_json::Value::String(pattern.description.clone()));
                
                let steps: Vec<serde_json::Value> = pattern.steps.iter().map(|step| {
                    let mut step_obj = serde_json::Map::new();
                    step_obj.insert("order".to_string(), serde_json::Value::Number(step.order.into()));
                    step_obj.insert("document".to_string(), serde_json::Value::String(step.document.clone()));
                    step_obj.insert("purpose".to_string(), serde_json::Value::String(step.purpose.clone()));
                    step_obj.insert("description".to_string(), serde_json::Value::String(step.description.clone()));
                    serde_json::Value::Object(step_obj)
                }).collect();
                
                pattern_obj.insert("steps".to_string(), serde_json::Value::Array(steps));
                serde_json::Value::Object(pattern_obj)
            })
            .collect();

        json_data.insert("navigation_patterns".to_string(), serde_json::Value::Array(patterns));

        serde_json::to_string_pretty(&serde_json::Value::Object(json_data)).unwrap()
    }

    pub fn generate_markdown_index(&self, index: &crate::documentation_index::DocumentationIndex) -> String {
        let mut output = String::new();

        output.push_str("# Solfunmeme-Dioxus Documentation Index\n\n");
        output.push_str("This document provides a comprehensive index of all documentation in the Solfunmeme-Dioxus system.\n\n");

        // Systems Design Documents
        output.push_str("## 🎯 Systems Design Documents\n\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::SystemsDesign) {
                output.push_str(&format!("- **{}** - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Architecture Documentation
        output.push_str("## 🏗️ Architecture Documentation\n\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::Architecture) {
                output.push_str(&format!("- **{}** - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // UML Diagrams
        output.push_str("## 📊 UML Diagrams\n\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::UMLDiagram) {
                output.push_str(&format!("- **{}** - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Ontologies
        output.push_str("## 🧠 Ontologies\n\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::Ontology) {
                output.push_str(&format!("- **{}** - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Workflow & Tools
        output.push_str("## ⚙️ Workflow & Tools\n\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::Workflow) {
                output.push_str(&format!("- **{}** - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Code Implementation
        output.push_str("## 💻 Code Implementation\n\n");
        for doc in index.get_documents() {
            if matches!(doc.category, DocumentCategory::Code) {
                output.push_str(&format!("- **{}** - {}\n", doc.path, doc.description));
            }
        }
        output.push_str("\n");

        // Navigation Patterns
        output.push_str("## 🎯 Navigation Patterns\n\n");

        for (user_type, pattern) in index.get_navigation_patterns() {
            output.push_str(&format!("### For {} users\n\n", user_type));
            output.push_str(&format!("**Description**: {}\n\n", pattern.description));
            
            for step in &pattern.steps {
                output.push_str(&format!("{}. **{}** - {}\n", step.order, step.document, step.purpose));
                output.push_str(&format!("   {}\n\n", step.description));
            }
        }

        // Cross-Reference Summary
        output.push_str("## 🔗 Cross-Reference Summary\n\n");
        output.push_str("- **Systems Design** → **Architecture** → **Implementation** → **Code**\n");
        output.push_str("- **Context** → **Container** → **Component** → **Deployment**\n");
        output.push_str("- **Use Cases** → **Data Flow** → **Sequence** → **Execution**\n");
        output.push_str("- **Ontologies** → **Semantic Models** → **Component Relationships**\n\n");

        output.push_str("## 📖 Related Documentation\n\n");
        output.push_str("- [Cross-References](docs/uml/CROSS_REFERENCES.md) - Comprehensive documentation relationships\n");
        output.push_str("- [Cross-Reference Diagram](docs/uml/09-cross-references.puml) - Visual documentation relationships\n");

        output
    }
} 