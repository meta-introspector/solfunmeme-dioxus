pub mod plugins;

use clap::{Parser, Subcommand};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use plugins::PluginManager;

#[derive(Parser)]
#[command(name = "zos")]
#[command(about = "ZOS: Unified CLI for Solfunmeme Project", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Start interactive mode")] 
    Interactive,
    #[command(about = "Run a pipeline of commands: grep | sort | uniq | head")]
    Pipeline {
        #[arg(help = "Pattern to grep for")] 
        grep: String,
        #[arg(long, help = "Sort the output")] 
        sort: bool,
        #[arg(long, help = "Deduplicate lines (uniq)")]
        uniq: bool,
        #[arg(long, help = "Show only the first N lines (head)")]
        head: Option<usize>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Interactive) | None => run_repl(),
        Some(Commands::Pipeline { grep, sort, uniq, head }) => run_pipeline(grep, *sort, *uniq, *head),
    }
}

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn run_pipeline(grep_pattern: &str, do_sort: bool, do_uniq: bool, head: Option<usize>) {
    // Run 'grep' on the workspace recursively
    let grep = Command::new("grep")
        .arg("-r")
        .arg(grep_pattern)
        .arg(".")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start grep");

    let mut prev_output = grep.stdout.expect("No grep output");

    // Optionally sort
    if do_sort {
        let sort = Command::new("sort")
            .stdin(Stdio::from(prev_output))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start sort");
        prev_output = sort.stdout.expect("No sort output");
    }

    // Optionally uniq
    if do_uniq {
        let uniq = Command::new("uniq")
            .stdin(Stdio::from(prev_output))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start uniq");
        prev_output = uniq.stdout.expect("No uniq output");
    }

    // Optionally head
    if let Some(n) = head {
        let head = Command::new("head")
            .arg(format!("-n{}", n))
            .stdin(Stdio::from(prev_output))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start head");
        prev_output = head.stdout.expect("No head output");
    }

    // Print the final output
    let reader = BufReader::new(prev_output);
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line);
        }
    }
}

fn run_repl() {
    let mut rl = Editor::<(), _>::new().unwrap();
    let mut plugin_manager = PluginManager::new();
    plugin_manager.load_plugins("plugins");

    println!("Welcome to zos interactive CLI! Type 'help' for commands, 'exit' to quit.");
    loop {
        let readline = rl.readline("zos> ");
        match readline {
            Ok(line) => {
                let cmd = line.trim();
                match cmd {
                    "exit" | "quit" => {
                        println!("Exiting zos CLI. Goodbye!");
                        break;
                    }
                    "help" => print_help(),
                    "" => {},
                    _ => plugin_manager.execute(cmd),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting zos CLI. Goodbye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn print_help() {
    println!("Available commands:");
    println!("  help         Show this help message");
    println!("  exit, quit   Exit the CLI");
    // Add more commands as you integrate tools
}