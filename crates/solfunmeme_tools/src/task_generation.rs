use anyhow::{Result, anyhow};
use solfunmeme_models::{LlmTaskGroup, LlmTaskPayload, CliffordOperationRequest, CodeEvolutionTask, CodeReflectionTask};
use solfunmeme_clifford::SolMultivector;

pub fn generate_llm_task_groups(
    task_type: &str,
    code_snippet: Option<String>,
    groups: Vec<(Vec<String>, SolMultivector)>,
) -> Result<Vec<LlmTaskGroup>> {
    if task_type == "clifford_operation" {
        Ok(vec![LlmTaskGroup {
            task_type: "clifford_operation".to_string(),
            payload: LlmTaskPayload::CliffordOperation(CliffordOperationRequest {
                operation: "create_scalar_multivector".to_string(),
                scalar_value: 42.0,
                vector_values: Vec::new(),
                input_multivector: None,
            }),
        }])
    } else if task_type == "code_evolution" {
        let code_snippet = code_snippet.ok_or_else(|| anyhow!("Code snippet is required for code_evolution task."))?;
        Ok(vec![LlmTaskGroup {
            task_type: "code_evolution".to_string(),
            payload: LlmTaskPayload::CodeEvolution(CodeEvolutionTask {
                code_snippet,
                meme_token: None, // MemeToken can be populated later or passed as input
            }),
        }])
    } else {
        Ok(groups.into_iter().map(|(group_contents, group_multivector)| {
            LlmTaskGroup {
                task_type: "code_reflection".to_string(),
                payload: LlmTaskPayload::CodeReflection(CodeReflectionTask {
                    code_chunks: group_contents,
                    multivector: group_multivector,
                }),
            }
        }).collect())
    }
}
