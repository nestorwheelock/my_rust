use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::collections::BTreeMap;
use toml::Value;
use ctrlc;
use dirs;
use clap::{Arg, Command};

/// Struct to store project information.
#[derive(Debug)]
struct ProjectInfo {
    name: String,
    description: Option<String>,
    path: PathBuf,
}

/// Parse the `Cargo.toml` file to extract project information.
fn parse_cargo_toml(path: &Path) -> Option<ProjectInfo> {
    let cargo_toml = fs::read_to_string(path).ok()?;
    let parsed: Value = cargo_toml.parse().ok()?;

    let package = parsed.get("package")?;
    let name = package.get("name")?.as_str()?.to_string();
    let description = package.get("description").and_then(|d| d.as_str()).map(|d| d.to_string());

    Some(ProjectInfo {
        name,
        description,
        path: path.parent()?.to_path_buf(),
    })
}

/// Recursively searches for Rust projects in the given directory.
fn find_projects(root: &Path) -> BTreeMap<String, ProjectInfo> {
    let mut projects = BTreeMap::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            if path.is_dir() {
                let cargo_toml_path = path.join("Cargo.toml");
                if cargo_toml_path.exists() {
                    if let Some(info) = parse_cargo_toml(&cargo_toml_path) {
                        projects.insert(info.name.clone(), info);
                    }
                }
            }
        }
    } else {
        eprintln!("Could not read directory: {:?}", root);
    }
    projects
}

/// Displays the list of found projects and allows selection for details.
fn display_projects(projects: &BTreeMap<String, ProjectInfo>) {
    if projects.is_empty() {
        println!("No Rust projects found.");
        return;
    }

    for (index, (name, info)) in projects.iter().enumerate() {
        println!("{}. {} - {}", index + 1, name, info.description.as_deref().unwrap_or("No description"));
    }

    println!("Enter the number of the project to view details, or 'q' to quit:");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.to_lowercase() == "q" {
            println!("Exiting program...");
            break;
        }

        if let Ok(index) = input.parse::<usize>() {
            if let Some((_, info)) = projects.iter().nth(index - 1) {
                display_project_details(info);
            } else {
                println!("Invalid selection. Please enter a valid project number.");
            }
        } else {
            println!("Please enter a valid number or 'q' to quit.");
        }
    }
}

/// Displays detailed information about a specific project.
fn display_project_details(info: &ProjectInfo) {
    println!("\nProject Details:");
    println!("Project Name: {}", info.name);
    println!("Description: {}", info.description.as_deref().unwrap_or("No description"));
    println!("Path: {:?}", info.path);
    println!("You can run this project from: {:?}", info.path.join("target/release").to_str());
}

/// Prints the manual page explaining how to use the program.
fn print_manual() {
    println!("My Rust Manager v0.1.0");
    println!();
    println!("USAGE:");
    println!("    my_rust [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --help      Prints this help manual page.");
    println!("    --list      Lists all the Rust projects found in your '~/rust' directory.");
    println!("    q           Quit the program.");
    println!();
    println!("EXAMPLES:");
    println!("    my_rust --list         # Lists all available projects");
}

/// Main function to parse arguments and start the program.
fn main() {
    // Handle Ctrl+C to exit the program
    ctrlc::set_handler(move || {
        println!("\nProgram interrupted. Exiting...");
        std::process::exit(0);
    }).expect("Error setting Ctrl+C handler");

    let matches = Command::new("My Rust Manager")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("A manual and manager of my Rust projects")
        .arg(Arg::new("help")
             .short('h')
             .long("help")
             .help("Displays the manual page"))
        .arg(Arg::new("list")
             .short('l')
             .long("list")
             .help("Lists all available projects"))
        .get_matches();

    // Parse arguments
    if matches.contains_id("help") {
        print_manual();
        return;
    }

    if matches.contains_id("list") {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let root_path = home_dir.join("rust");

        if !root_path.exists() {
            println!("Sorry, no Rust projects found.");
            return;
        }

        let projects = find_projects(&root_path);
        display_projects(&projects);
    } else {
        println!("Use '--list' to view available projects.");
    }
}

