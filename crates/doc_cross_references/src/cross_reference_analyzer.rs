use crate::types::*;
use anyhow::Result;
use std::collections::HashMap;

pub struct CrossReferenceAnalyzer {
    documents: Vec<Document>,
}

impl CrossReferenceAnalyzer {
    pub fn new() -> Result<Self> {
        let index = crate::documentation_index::DocumentationIndex::new()?;
        Ok(Self {
            documents: index.get_documents().to_vec(),
        })
    }

    pub fn analyze_all(&self) -> Result<CrossReferenceAnalysis> {
        let mut analysis = CrossReferenceAnalysis::new();

        // Add all documents to analysis
        for doc in &self.documents {
            analysis.add_document(doc.clone());
        }

        // Analyze categories
        self.analyze_categories(&mut analysis)?;

        // Analyze navigation patterns
        self.analyze_navigation_patterns(&mut analysis)?;

        Ok(analysis)
    }

    fn analyze_categories(&self, analysis: &mut CrossReferenceAnalysis) -> Result<()> {
        let mut categories: HashMap<String, Vec<String>> = HashMap::new();

        for doc in &self.documents {
            let category_name = match doc.category {
                DocumentCategory::SystemsDesign => "Systems Design",
                DocumentCategory::Architecture => "Architecture",
                DocumentCategory::UMLDiagram => "UML Diagrams",
                DocumentCategory::Ontology => "Ontologies",
                DocumentCategory::Workflow => "Workflow & Tools",
                DocumentCategory::Code => "Code Implementation",
                DocumentCategory::External => "External References",
            };

            categories
                .entry(category_name.to_string())
                .or_insert_with(Vec::new)
                .push(doc.path.clone());
        }

        analysis.categories = categories;
        Ok(())
    }

    fn analyze_navigation_patterns(&self, analysis: &mut CrossReferenceAnalysis) -> Result<()> {
        let index = crate::documentation_index::DocumentationIndex::new()?;
        analysis.navigation_patterns = index.get_navigation_patterns().clone();
        Ok(())
    }

    pub fn generate_plantuml_diagram(&self, analysis: &CrossReferenceAnalysis) -> Result<String> {
        let mut diagram = String::new();

        // Header
        diagram.push_str("@startuml Cross_References\n");
        diagram.push_str("!theme plain\n");
        diagram.push_str("skinparam rectangle {\n");
        diagram.push_str("    BackgroundColor LightGreen\n");
        diagram.push_str("    BorderColor DarkGreen\n");
        diagram.push_str("}\n");
        diagram.push_str("skinparam database {\n");
        diagram.push_str("    BackgroundColor LightYellow\n");
        diagram.push_str("    BorderColor DarkYellow\n");
        diagram.push_str("}\n");
        diagram.push_str("skinparam file {\n");
        diagram.push_str("    BackgroundColor LightBlue\n");
        diagram.push_str("    BorderColor DarkBlue\n");
        diagram.push_str("}\n");
        diagram.push_str("skinparam cloud {\n");
        diagram.push_str("    BackgroundColor LightCyan\n");
        diagram.push_str("    BorderColor DarkCyan\n");
        diagram.push_str("}\n\n");

        diagram.push_str("title Solfunmeme-Dioxus Documentation Cross-References\n\n");

        // Group documents by category
        let mut category_groups: HashMap<String, Vec<&Document>> = HashMap::new();
        for doc in &analysis.documents {
            let category = match doc.category {
                DocumentCategory::SystemsDesign => "Systems Design",
                DocumentCategory::Architecture => "Architecture Documentation",
                DocumentCategory::UMLDiagram => "UML Diagrams",
                DocumentCategory::Ontology => "Ontologies",
                DocumentCategory::Workflow => "Workflow & Tools",
                DocumentCategory::Code => "Code Implementation",
                DocumentCategory::External => "External References",
            };
            category_groups
                .entry(category.to_string())
                .or_insert_with(Vec::new)
                .push(doc);
        }

        // Create rectangles for each category
        for (category_name, docs) in &category_groups {
            diagram.push_str(&format!("rectangle \"{}\" as {} {{\n", category_name, self.sanitize_name(category_name)));
            
            for doc in docs {
                let doc_name = self.sanitize_name(&doc.path);
                diagram.push_str(&format!("    file \"{}\" as {}\n", doc.path, doc_name));
            }
            
            diagram.push_str("}\n\n");
        }

        // Add cross-references
        for doc in &analysis.documents {
            for reference in &doc.references {
                let source_name = self.sanitize_name(&doc.path);
                let target_name = self.sanitize_name(&reference.target);
                
                let arrow = if reference.bidirectional { "<-->" } else { "-->" };
                let label = self.format_reference_label(&reference.reference_type, &reference.description);
                
                diagram.push_str(&format!("{} {} {} : {}\n", source_name, arrow, target_name, label));
            }
        }

        // Add notes
        diagram.push_str("\n");
        diagram.push_str("note right of SystemsDesign : \"Systems Design Documents:\\n• Define the 8 factorial steps\\n• Establish autopoetic rewrites\\n• Set architectural principles\"\n");
        diagram.push_str("note right of Architecture : \"Architecture Documentation:\\n• Explains system structure\\n• Documents component purposes\\n• Provides implementation guidance\"\n");
        diagram.push_str("note right of UMLDiagrams : \"UML Diagrams:\\n• Visualize system architecture\\n• Show data flows and interactions\\n• Provide multiple abstraction levels\"\n");
        diagram.push_str("note right of Ontologies : \"Ontologies:\\n• Define semantic relationships\\n• Model system concepts\\n• Enable semantic search\"\n");
        diagram.push_str("note right of WorkflowTools : \"Workflow & Tools:\\n• Implement the 8 factorial steps\\n• Provide CLI and automation\\n• Enable system interaction\"\n");
        diagram.push_str("note right of CodeImplementation : \"Code Implementation:\\n• Realizes the architecture\\n• Implements components\\n• Provides testing framework\"\n");

        diagram.push_str("\n@enduml\n");

        Ok(diagram)
    }

    pub fn validate_references(&self, check_links: bool, check_orphans: bool) -> Result<ValidationResult> {
        let mut validation = ValidationResult {
            broken_links: Vec::new(),
            orphaned_documents: Vec::new(),
            total_checked: 0,
            valid_links: 0,
        };

        if check_links {
            self.validate_links(&mut validation)?;
        }

        if check_orphans {
            self.find_orphaned_documents(&mut validation)?;
        }

        Ok(validation)
    }

    fn validate_links(&self, validation: &mut ValidationResult) -> Result<()> {
        for doc in &self.documents {
            for reference in &doc.references {
                validation.total_checked += 1;

                // Check if target document exists
                let target_exists = self.documents.iter().any(|d| d.path == reference.target);

                if target_exists {
                    validation.valid_links += 1;
                } else {
                    validation.broken_links.push(BrokenLink {
                        source: doc.path.clone(),
                        target: reference.target.clone(),
                        reference_type: reference.reference_type.clone(),
                        error: "Target document not found".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    fn find_orphaned_documents(&self, validation: &mut ValidationResult) -> Result<()> {
        for doc in &self.documents {
            // Skip external documents and certain categories that might not have references
            if matches!(doc.category, DocumentCategory::External) {
                continue;
            }

            // Check if document is referenced by any other document
            let is_referenced = self.documents.iter().any(|other_doc| {
                other_doc.references.iter().any(|ref_| ref_.target == doc.path)
            });

            if !is_referenced && doc.references.is_empty() {
                validation.orphaned_documents.push(doc.path.clone());
            }
        }

        Ok(())
    }

    fn sanitize_name(&self, name: &str) -> String {
        name.replace('/', "_")
            .replace('.', "_")
            .replace('-', "_")
            .replace(' ', "")
    }

    fn format_reference_label(&self, reference_type: &ReferenceType, description: &str) -> String {
        let icon = match reference_type {
            ReferenceType::Implements => "↗️ Implements",
            ReferenceType::Visualizes => "📊 Visualizes",
            ReferenceType::Defines => "📝 Defines",
            ReferenceType::References => "🔗 References",
            ReferenceType::Models => "🧠 Models",
            ReferenceType::Documents => "📚 Documents",
            ReferenceType::Influences => "💡 Influences",
            ReferenceType::DrillsDown => "🔍 Drills down",
            ReferenceType::Shows => "👁️ Shows",
            ReferenceType::Explains => "💬 Explains",
            ReferenceType::Details => "🔍 Details",
            ReferenceType::Lists => "📋 Lists",
            ReferenceType::Describes => "📝 Describes",
            ReferenceType::Provides => "⚙️ Provides",
            ReferenceType::Uses => "🔧 Uses",
            ReferenceType::Contains => "📦 Contains",
            ReferenceType::Tests => "🧪 Tests",
        };

        format!("{} {}", icon, description)
    }
} 