use crate::types::*;
use anyhow::Result;
use std::collections::HashMap;

pub struct DocumentationIndex {
    documents: Vec<Document>,
    categories: HashMap<String, DocumentationCategory>,
    navigation_patterns: HashMap<String, NavigationPattern>,
}

impl DocumentationIndex {
    pub fn new() -> Result<Self> {
        let mut index = Self {
            documents: Vec::new(),
            categories: HashMap::new(),
            navigation_patterns: HashMap::new(),
        };

        index.initialize_documents()?;
        index.initialize_categories()?;
        index.initialize_navigation_patterns()?;
        index.establish_cross_references()?;

        Ok(index)
    }

    fn initialize_documents(&mut self) -> Result<()> {
        // Systems Design Documents
        self.documents.push(Document::new(
            "vendor/meta-meme.wiki/SystemsDesign.md".to_string(),
            "Systems Design".to_string(),
            DocumentCategory::SystemsDesign,
            "Core 8 factorial steps concept and autopoetic rewrites".to_string(),
        ));

        self.documents.push(Document::new(
            "founding_documents/ALGORITHM_DOCUMENTATION.md".to_string(),
            "Algorithm Documentation".to_string(),
            DocumentCategory::SystemsDesign,
            "Algorithm details and mathematical foundations".to_string(),
        ));

        self.documents.push(Document::new(
            "founding_documents/chat-solfunmeme.md".to_string(),
            "Chat System Design".to_string(),
            DocumentCategory::SystemsDesign,
            "Chat system design and user interactions".to_string(),
        ));

        self.documents.push(Document::new(
            "founding_documents/GEMINI.md".to_string(),
            "Gemini Integration".to_string(),
            DocumentCategory::SystemsDesign,
            "Gemini integration and L2 architecture".to_string(),
        ));

        // Architecture Documentation
        self.documents.push(Document::new(
            "doc/architecture.md".to_string(),
            "System Architecture".to_string(),
            DocumentCategory::Architecture,
            "System architecture overview and component descriptions".to_string(),
        ));

        self.documents.push(Document::new(
            "doc/cli_tools_guide.md".to_string(),
            "CLI Tools Guide".to_string(),
            DocumentCategory::Architecture,
            "CLI tools documentation and usage".to_string(),
        ));

        self.documents.push(Document::new(
            "doc/chat_processing.md".to_string(),
            "Chat Processing".to_string(),
            DocumentCategory::Architecture,
            "Chat processing details and workflows".to_string(),
        ));

        self.documents.push(Document::new(
            "doc/development_setup.md".to_string(),
            "Development Setup".to_string(),
            DocumentCategory::Architecture,
            "Development environment setup and configuration".to_string(),
        ));

        // UML Diagrams
        let uml_diagrams = vec![
            ("docs/uml/01-context-diagram.puml", "System Context", "System context and boundaries"),
            ("docs/uml/02-container-diagram.puml", "Container Architecture", "Container architecture and technology choices"),
            ("docs/uml/03-component-diagram.puml", "Component Interactions", "Component interactions and implementation details"),
            ("docs/uml/04-deployment-diagram.puml", "Deployment Architecture", "Deployment architecture and infrastructure"),
            ("docs/uml/05-use-case-diagram.puml", "User Interactions", "User interactions and system functionality"),
            ("docs/uml/06-data-flow-diagram.puml", "Data Flow", "Data transformation pipeline"),
            ("docs/uml/07-sequence-diagram.puml", "Component Interactions", "Component interactions during execution"),
            ("docs/uml/08-l2-sidechain-architecture.puml", "L2 Sidechain", "L2 sidechain design and mainnet rollup"),
            ("docs/uml/09-cross-references.puml", "Cross References", "Documentation relationships and navigation"),
        ];

        for (path, title, description) in uml_diagrams {
            self.documents.push(Document::new(
                path.to_string(),
                title.to_string(),
                DocumentCategory::UMLDiagram,
                description.to_string(),
            ));
        }

        self.documents.push(Document::new(
            "docs/uml/README.md".to_string(),
            "UML Diagrams Guide".to_string(),
            DocumentCategory::Architecture,
            "UML diagrams guide and usage instructions".to_string(),
        ));

        self.documents.push(Document::new(
            "docs/uml/CROSS_REFERENCES.md".to_string(),
            "Cross References".to_string(),
            DocumentCategory::Architecture,
            "Comprehensive documentation relationships".to_string(),
        ));

        // Ontologies
        self.documents.push(Document::new(
            "ontologies/project_ontology.ttl".to_string(),
            "Project Ontology".to_string(),
            DocumentCategory::Ontology,
            "Project semantic model and concept definitions".to_string(),
        ));

        self.documents.push(Document::new(
            "ontologies/solfunmem.jsonld".to_string(),
            "Solfunmeme Concepts".to_string(),
            DocumentCategory::Ontology,
            "Solfunmeme concepts and semantic relationships".to_string(),
        ));

        self.documents.push(Document::new(
            "ontologies/introspector/".to_string(),
            "Introspection Ontology".to_string(),
            DocumentCategory::Ontology,
            "Introspection ontology and use cases".to_string(),
        ));

        self.documents.push(Document::new(
            "ontologies/zos/".to_string(),
            "ZOS CLI Ontology".to_string(),
            DocumentCategory::Ontology,
            "ZOS CLI concepts and command structure".to_string(),
        ));

        // Workflow & Tools
        self.documents.push(Document::new(
            "Makefile".to_string(),
            "Makefile".to_string(),
            DocumentCategory::Workflow,
            "Automated workflow implementation".to_string(),
        ));

        self.documents.push(Document::new(
            "WORKFLOW_DIAGRAM.md".to_string(),
            "Workflow Diagram".to_string(),
            DocumentCategory::Workflow,
            "Workflow visualization and pipeline".to_string(),
        ));

        self.documents.push(Document::new(
            "QUICK_REFERENCE.md".to_string(),
            "Quick Reference".to_string(),
            DocumentCategory::Workflow,
            "Essential commands and quick reference".to_string(),
        ));

        self.documents.push(Document::new(
            "zos/src/main.rs".to_string(),
            "ZOS CLI".to_string(),
            DocumentCategory::Code,
            "ZOS CLI implementation".to_string(),
        ));

        // Code Implementation
        self.documents.push(Document::new(
            "Cargo.toml".to_string(),
            "Cargo Configuration".to_string(),
            DocumentCategory::Code,
            "Workspace configuration and dependencies".to_string(),
        ));

        self.documents.push(Document::new(
            "src/".to_string(),
            "Source Code".to_string(),
            DocumentCategory::Code,
            "Main source code implementation".to_string(),
        ));

        self.documents.push(Document::new(
            "crates/".to_string(),
            "Architectural Crates".to_string(),
            DocumentCategory::Code,
            "Architectural crates and modules".to_string(),
        ));

        self.documents.push(Document::new(
            "tests/".to_string(),
            "Test Suite".to_string(),
            DocumentCategory::Code,
            "Test suite and validation".to_string(),
        ));

        Ok(())
    }

    fn initialize_categories(&mut self) -> Result<()> {
        let categories = vec![
            DocumentationCategory {
                name: "Systems Design".to_string(),
                description: "Core design documents and principles".to_string(),
                documents: vec![
                    "vendor/meta-meme.wiki/SystemsDesign.md".to_string(),
                    "founding_documents/ALGORITHM_DOCUMENTATION.md".to_string(),
                    "founding_documents/chat-solfunmeme.md".to_string(),
                    "founding_documents/GEMINI.md".to_string(),
                ],
                color: "LightGreen".to_string(),
            },
            DocumentationCategory {
                name: "Architecture".to_string(),
                description: "System architecture and design documentation".to_string(),
                documents: vec![
                    "doc/architecture.md".to_string(),
                    "doc/cli_tools_guide.md".to_string(),
                    "doc/chat_processing.md".to_string(),
                    "doc/development_setup.md".to_string(),
                    "docs/uml/README.md".to_string(),
                    "docs/uml/CROSS_REFERENCES.md".to_string(),
                ],
                color: "LightBlue".to_string(),
            },
            DocumentationCategory {
                name: "UML Diagrams".to_string(),
                description: "Visual architecture representations".to_string(),
                documents: vec![
                    "docs/uml/01-context-diagram.puml".to_string(),
                    "docs/uml/02-container-diagram.puml".to_string(),
                    "docs/uml/03-component-diagram.puml".to_string(),
                    "docs/uml/04-deployment-diagram.puml".to_string(),
                    "docs/uml/05-use-case-diagram.puml".to_string(),
                    "docs/uml/06-data-flow-diagram.puml".to_string(),
                    "docs/uml/07-sequence-diagram.puml".to_string(),
                    "docs/uml/08-l2-sidechain-architecture.puml".to_string(),
                    "docs/uml/09-cross-references.puml".to_string(),
                ],
                color: "LightYellow".to_string(),
            },
            DocumentationCategory {
                name: "Ontologies".to_string(),
                description: "Semantic models and relationships".to_string(),
                documents: vec![
                    "ontologies/project_ontology.ttl".to_string(),
                    "ontologies/solfunmem.jsonld".to_string(),
                    "ontologies/introspector/".to_string(),
                    "ontologies/zos/".to_string(),
                ],
                color: "LightCyan".to_string(),
            },
            DocumentationCategory {
                name: "Workflow & Tools".to_string(),
                description: "Automation and workflow tools".to_string(),
                documents: vec![
                    "Makefile".to_string(),
                    "WORKFLOW_DIAGRAM.md".to_string(),
                    "QUICK_REFERENCE.md".to_string(),
                ],
                color: "LightMagenta".to_string(),
            },
            DocumentationCategory {
                name: "Code Implementation".to_string(),
                description: "Source code and implementation".to_string(),
                documents: vec![
                    "zos/src/main.rs".to_string(),
                    "Cargo.toml".to_string(),
                    "src/".to_string(),
                    "crates/".to_string(),
                    "tests/".to_string(),
                ],
                color: "LightGray".to_string(),
            },
        ];

        for category in categories {
            self.categories.insert(category.name.clone(), category);
        }

        Ok(())
    }

    fn initialize_navigation_patterns(&mut self) -> Result<()> {
        // New Users Navigation Pattern
        let new_users = NavigationPattern {
            user_type: "new".to_string(),
            description: "For users new to the system".to_string(),
            steps: vec![
                NavigationStep {
                    order: 1,
                    document: "vendor/meta-meme.wiki/SystemsDesign.md".to_string(),
                    purpose: "High-level understanding".to_string(),
                    description: "Start with the core 8 factorial steps concept".to_string(),
                },
                NavigationStep {
                    order: 2,
                    document: "docs/uml/01-context-diagram.puml".to_string(),
                    purpose: "System boundaries".to_string(),
                    description: "Understand system context and external dependencies".to_string(),
                },
                NavigationStep {
                    order: 3,
                    document: "doc/architecture.md".to_string(),
                    purpose: "Detailed architecture".to_string(),
                    description: "Learn about system components and structure".to_string(),
                },
                NavigationStep {
                    order: 4,
                    document: "QUICK_REFERENCE.md".to_string(),
                    purpose: "Practical commands".to_string(),
                    description: "Get started with essential commands".to_string(),
                },
            ],
        };

        // Developers Navigation Pattern
        let developers = NavigationPattern {
            user_type: "developer".to_string(),
            description: "For developers working on the system".to_string(),
            steps: vec![
                NavigationStep {
                    order: 1,
                    document: "doc/architecture.md".to_string(),
                    purpose: "Component understanding".to_string(),
                    description: "Understand the component architecture".to_string(),
                },
                NavigationStep {
                    order: 2,
                    document: "docs/uml/03-component-diagram.puml".to_string(),
                    purpose: "Implementation details".to_string(),
                    description: "See how components interact".to_string(),
                },
                NavigationStep {
                    order: 3,
                    document: "Cargo.toml".to_string(),
                    purpose: "Crate dependencies".to_string(),
                    description: "Understand workspace structure".to_string(),
                },
                NavigationStep {
                    order: 4,
                    document: "Makefile".to_string(),
                    purpose: "Development workflows".to_string(),
                    description: "Use automated development workflows".to_string(),
                },
            ],
        };

        // Architects Navigation Pattern
        let architects = NavigationPattern {
            user_type: "architect".to_string(),
            description: "For system architects and designers".to_string(),
            steps: vec![
                NavigationStep {
                    order: 1,
                    document: "vendor/meta-meme.wiki/SystemsDesign.md".to_string(),
                    purpose: "Design principles".to_string(),
                    description: "Understand the core design principles".to_string(),
                },
                NavigationStep {
                    order: 2,
                    document: "docs/uml/".to_string(),
                    purpose: "All UML diagrams".to_string(),
                    description: "Review all architectural views".to_string(),
                },
                NavigationStep {
                    order: 3,
                    document: "ontologies/".to_string(),
                    purpose: "Semantic models".to_string(),
                    description: "Understand semantic relationships".to_string(),
                },
                NavigationStep {
                    order: 4,
                    document: "docs/uml/08-l2-sidechain-architecture.puml".to_string(),
                    purpose: "L2 design".to_string(),
                    description: "Review L2 sidechain architecture".to_string(),
                },
            ],
        };

        // System Administrators Navigation Pattern
        let admins = NavigationPattern {
            user_type: "admin".to_string(),
            description: "For system administrators".to_string(),
            steps: vec![
                NavigationStep {
                    order: 1,
                    document: "docs/uml/04-deployment-diagram.puml".to_string(),
                    purpose: "Infrastructure".to_string(),
                    description: "Understand deployment architecture".to_string(),
                },
                NavigationStep {
                    order: 2,
                    document: "doc/development_setup.md".to_string(),
                    purpose: "Setup procedures".to_string(),
                    description: "Follow setup and configuration procedures".to_string(),
                },
                NavigationStep {
                    order: 3,
                    document: "Makefile".to_string(),
                    purpose: "Deployment commands".to_string(),
                    description: "Use deployment automation".to_string(),
                },
                NavigationStep {
                    order: 4,
                    document: "QUICK_REFERENCE.md".to_string(),
                    purpose: "Operational commands".to_string(),
                    description: "Access operational commands".to_string(),
                },
            ],
        };

        self.navigation_patterns.insert("new".to_string(), new_users);
        self.navigation_patterns.insert("developer".to_string(), developers);
        self.navigation_patterns.insert("architect".to_string(), architects);
        self.navigation_patterns.insert("admin".to_string(), admins);

        Ok(())
    }

    fn establish_cross_references(&mut self) -> Result<()> {
        // Systems Design → Implementation references
        self.add_reference(
            "vendor/meta-meme.wiki/SystemsDesign.md",
            "doc/architecture.md",
            ReferenceType::Implements,
            "Implements the 8 factorial steps architecture",
            true,
        );

        self.add_reference(
            "vendor/meta-meme.wiki/SystemsDesign.md",
            "docs/uml/01-context-diagram.puml",
            ReferenceType::Visualizes,
            "Visualizes system context and external dependencies",
            false,
        );

        self.add_reference(
            "vendor/meta-meme.wiki/SystemsDesign.md",
            "Makefile",
            ReferenceType::Defines,
            "Defines the 8 factorial steps pipeline",
            false,
        );

        self.add_reference(
            "founding_documents/ALGORITHM_DOCUMENTATION.md",
            "docs/uml/03-component-diagram.puml",
            ReferenceType::Details,
            "Details component interactions and algorithms",
            false,
        );

        self.add_reference(
            "founding_documents/chat-solfunmeme.md",
            "docs/uml/05-use-case-diagram.puml",
            ReferenceType::Defines,
            "Defines user interactions and system functionality",
            false,
        );

        self.add_reference(
            "founding_documents/GEMINI.md",
            "docs/uml/08-l2-sidechain-architecture.puml",
            ReferenceType::Influences,
            "Influences L2 sidechain architecture design",
            false,
        );

        // Architecture Documentation → Visualizations
        self.add_reference(
            "doc/architecture.md",
            "docs/uml/01-context-diagram.puml",
            ReferenceType::References,
            "References context diagram for system overview",
            false,
        );

        self.add_reference(
            "doc/architecture.md",
            "docs/uml/02-container-diagram.puml",
            ReferenceType::References,
            "References container diagram for technology choices",
            false,
        );

        self.add_reference(
            "doc/architecture.md",
            "docs/uml/03-component-diagram.puml",
            ReferenceType::References,
            "References component diagram for implementation details",
            false,
        );

        self.add_reference(
            "doc/architecture.md",
            "Cargo.toml",
            ReferenceType::Lists,
            "Lists all architectural crates and dependencies",
            false,
        );

        self.add_reference(
            "doc/architecture.md",
            "crates/",
            ReferenceType::Describes,
            "Describes the purpose of each architectural crate",
            false,
        );

        // UML Diagram Hierarchy
        self.add_reference(
            "docs/uml/01-context-diagram.puml",
            "docs/uml/02-container-diagram.puml",
            ReferenceType::DrillsDown,
            "Drills down from system context to containers",
            true,
        );

        self.add_reference(
            "docs/uml/02-container-diagram.puml",
            "docs/uml/03-component-diagram.puml",
            ReferenceType::DrillsDown,
            "Drills down from containers to components",
            true,
        );

        self.add_reference(
            "docs/uml/03-component-diagram.puml",
            "docs/uml/04-deployment-diagram.puml",
            ReferenceType::Shows,
            "Shows how components are deployed",
            true,
        );

        self.add_reference(
            "docs/uml/06-data-flow-diagram.puml",
            "docs/uml/07-sequence-diagram.puml",
            ReferenceType::Shows,
            "Shows static data flow vs dynamic behavior",
            true,
        );

        self.add_reference(
            "docs/uml/08-l2-sidechain-architecture.puml",
            "docs/uml/06-data-flow-diagram.puml",
            ReferenceType::Details,
            "Details L2 data flow within the pipeline",
            false,
        );

        // UML Documentation → Diagrams
        for diagram in &[
            "docs/uml/01-context-diagram.puml",
            "docs/uml/02-container-diagram.puml",
            "docs/uml/03-component-diagram.puml",
            "docs/uml/04-deployment-diagram.puml",
            "docs/uml/05-use-case-diagram.puml",
            "docs/uml/06-data-flow-diagram.puml",
            "docs/uml/07-sequence-diagram.puml",
            "docs/uml/08-l2-sidechain-architecture.puml",
        ] {
            self.add_reference(
                "docs/uml/README.md",
                diagram,
                ReferenceType::Explains,
                "Explains diagram purpose and usage",
                false,
            );
        }

        // Ontologies → Implementation
        self.add_reference(
            "ontologies/project_ontology.ttl",
            "doc/architecture.md",
            ReferenceType::Defines,
            "Defines system concepts and relationships",
            false,
        );

        self.add_reference(
            "ontologies/project_ontology.ttl",
            "docs/uml/01-context-diagram.puml",
            ReferenceType::Models,
            "Models system entities and boundaries",
            false,
        );

        self.add_reference(
            "ontologies/solfunmem.jsonld",
            "docs/uml/03-component-diagram.puml",
            ReferenceType::Describes,
            "Describes component semantics and relationships",
            false,
        );

        self.add_reference(
            "ontologies/introspector/",
            "docs/uml/05-use-case-diagram.puml",
            ReferenceType::Defines,
            "Defines introspection use cases and workflows",
            false,
        );

        self.add_reference(
            "ontologies/zos/",
            "zos/src/main.rs",
            ReferenceType::Models,
            "Models CLI concepts and command structure",
            false,
        );

        // Workflow & Tools → Documentation
        self.add_reference(
            "Makefile",
            "WORKFLOW_DIAGRAM.md",
            ReferenceType::Implements,
            "Implements the workflow visualization",
            true,
        );

        self.add_reference(
            "Makefile",
            "QUICK_REFERENCE.md",
            ReferenceType::Provides,
            "Provides commands for quick reference",
            true,
        );

        self.add_reference(
            "WORKFLOW_DIAGRAM.md",
            "docs/uml/06-data-flow-diagram.puml",
            ReferenceType::Visualizes,
            "Visualizes the data flow pipeline",
            false,
        );

        self.add_reference(
            "WORKFLOW_DIAGRAM.md",
            "docs/uml/07-sequence-diagram.puml",
            ReferenceType::Shows,
            "Shows the execution sequence",
            false,
        );

        self.add_reference(
            "QUICK_REFERENCE.md",
            "zos/src/main.rs",
            ReferenceType::Documents,
            "Documents CLI commands and usage",
            false,
        );

        self.add_reference(
            "zos/src/main.rs",
            "Cargo.toml",
            ReferenceType::Uses,
            "Uses workspace crates and dependencies",
            false,
        );

        // Code Implementation → Architecture
        self.add_reference(
            "Cargo.toml",
            "doc/architecture.md",
            ReferenceType::Lists,
            "Lists all architectural components",
            false,
        );

        self.add_reference(
            "Cargo.toml",
            "crates/",
            ReferenceType::Defines,
            "Defines workspace structure and relationships",
            false,
        );

        self.add_reference(
            "src/",
            "docs/uml/03-component-diagram.puml",
            ReferenceType::Implements,
            "Implements the component architecture",
            false,
        );

        self.add_reference(
            "crates/",
            "doc/architecture.md",
            ReferenceType::Contains,
            "Contains architectural crates and their purposes",
            false,
        );

        self.add_reference(
            "tests/",
            "doc/architecture.md",
            ReferenceType::Tests,
            "Tests architectural components and integration",
            false,
        );

        Ok(())
    }

    fn add_reference(
        &mut self,
        source: &str,
        target: &str,
        reference_type: ReferenceType,
        description: &str,
        bidirectional: bool,
    ) {
        if let Some(source_doc) = self.documents.iter_mut().find(|d| d.path == source) {
            source_doc.add_reference(
                target.to_string(),
                reference_type.clone(),
                description.to_string(),
                bidirectional,
            );
        }

        if bidirectional {
            if let Some(target_doc) = self.documents.iter_mut().find(|d| d.path == target) {
                target_doc.add_referenced_by(source.to_string());
            }
        }
    }

    pub fn get_navigation_pattern(&self, user_type: &str) -> Result<String> {
        if let Some(pattern) = self.navigation_patterns.get(user_type) {
            let mut output = format!("Navigation pattern for {} users:\n", user_type);
            output.push_str(&format!("Description: {}\n\n", pattern.description));
            
            for step in &pattern.steps {
                output.push_str(&format!("{}. {} - {}\n", step.order, step.document, step.purpose));
                output.push_str(&format!("   {}\n\n", step.description));
            }
            
            Ok(output)
        } else {
            Err(anyhow::anyhow!("Unknown user type: {}", user_type))
        }
    }

    pub fn get_documents(&self) -> &[Document] {
        &self.documents
    }

    pub fn get_categories(&self) -> &HashMap<String, DocumentationCategory> {
        &self.categories
    }

    pub fn get_navigation_patterns(&self) -> &HashMap<String, NavigationPattern> {
        &self.navigation_patterns
    }
} 