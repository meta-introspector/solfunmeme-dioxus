use anyhow::Result;
use std::process::{Command, Child};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Host {
    Local,
    Remote(String), // hostname or IP address
}

pub struct ProcessConfig {
    pub command: String,
    pub args: Vec<String>,
    pub host: Host,
    pub env: HashMap<String, String>,
}

pub trait Workflow: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self) -> Result<()>;
    fn get_process_configs(&self) -> Vec<ProcessConfig>;
}

pub struct WorkflowManager {
    workflows: Vec<Box<dyn Workflow>>,
    running_processes: HashMap<String, Child>,
}

impl WorkflowManager {
    pub fn new() -> Self {
        WorkflowManager {
            workflows: Vec::new(),
            running_processes: HashMap::new(),
        }
    }

    pub fn register_workflow(&mut self, workflow: Box<dyn Workflow>) {
        self.workflows.push(workflow);
    }

    pub fn execute_workflow(&mut self, name: &str) -> Result<()> {
        let workflow = self.workflows.iter()
            .find(|w| w.name() == name)
            .ok_or_else(|| anyhow::anyhow!("Workflow not found: {}", name))?;

        // Get process configurations from the workflow
        let configs = workflow.get_process_configs();

        // Spawn processes based on configurations
        for config in configs {
            let process = match config.host {
                Host::Local => {
                    Command::new(&config.command)
                        .args(&config.args)
                        .envs(config.env)
                        .spawn()?
                }
                Host::Remote(host) => {
                    // For remote execution, we wrap the command with SSH
                    let remote_cmd = format!("{} {}", 
                        config.command,
                        config.args.join(" ")
                    );
                    
                    Command::new("ssh")
                        .arg(host)
                        .arg(remote_cmd)
                        .envs(config.env)
                        .spawn()?
                }
            };

            self.running_processes.insert(
                format!("{}-{}", name, config.command),
                process
            );
        }

        // Execute the workflow logic
        workflow.execute()
    }

    pub fn check_process_status(&mut self) -> Result<()> {
        let mut finished = Vec::new();
        
        for (name, process) in &mut self.running_processes {
            match process.try_wait()? {
                Some(status) => {
                    if !status.success() {
                        return Err(anyhow::anyhow!(
                            "Process {} failed with status: {}", 
                            name, status
                        ));
                    }
                    finished.push(name.clone());
                }
                None => continue, // Process still running
            }
        }

        // Remove finished processes
        for name in finished {
            self.running_processes.remove(&name);
        }

        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<()> {
        for (name, mut process) in self.running_processes.drain() {
            process.kill()?;
            process.wait()?;
            println!("Killed process: {}", name);
        }
        Ok(())
    }
}

// Implement Drop to ensure cleanup on manager destruction
impl Drop for WorkflowManager {
    fn drop(&mut self) {
        if let Err(e) = self.cleanup() {
            eprintln!("Error during cleanup: {}", e);
        }
    }
}
