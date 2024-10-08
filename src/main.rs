use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::collections::BTreeMap;
use toml::Value;
use ctrlc;
use dirs;
use clap::{Arg, Command};

/// Struct representing information about a Rust project.
///
/// This struct holds the project's name, an optional description, and the path to the project.
#[derive(Debug)]
struct ProjectInfo {
    /// The name of the project.
    name: String,
    /// An optional description of the project.
    description: Option<String>,
    /// The path where the project is located.
    path: PathBuf,
}

/// Parses a `Cargo.toml` file and extracts project information.
///
/// This function reads a `Cargo.toml` file from the specified path, extracts the package name
/// and description (if available), and returns it as a `ProjectInfo` struct.
///
/// # Arguments
///
/// * `path` - The path to the `Cargo.toml` file.
///
/// # Returns
///
/// An `Option<ProjectInfo>` with the project name, description, and path.
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

/// Recursively searches for Rust projects in the specified directory.
///
/// This function looks for directories containing `Cargo.toml` files within
/// the given root directory and returns a map of project names to their respective `ProjectInfo`.
///
/// # Arguments
///
/// * `root` - The root directory to search for Rust projects.
///
/// # Returns
///
/// A `BTreeMap` where the keys are project names and the values are `ProjectInfo` structs.
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

/// Displays a list of found Rust projects and allows selection for more details.
///
/// This function lists all the projects found in the specified directory, displaying their
/// index, name, and description. The user can then select a project by entering its index
/// to view more detailed information.
///
/// # Arguments
///
/// * `projects` - A reference to a map of project names and their respective `ProjectInfo`.
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

/// Displays detailed information about a specific Rust project.
///
/// This function prints the project name, description (if available), and path.
/// It also provides the location where the compiled project can be run.
///
/// # Arguments
///
/// * `info` - A reference to the `ProjectInfo` struct for the selected project.
fn display_project_details(info: &ProjectInfo) {
    println!("\nProject Details:");
    println!("Project Name: {}", info.name);
    println!("Description: {}", info.description.as_deref().unwrap_or("No description"));
    println!("Path: {:?}", info.path);
    println!("You can run this project from: {:?}", info.path.join("target/release").to_str());
}

/// Main function to handle the execution of the program.
///
/// This function sets up the argument parsing, handles Ctrl+C interrupts, and
/// defaults to listing projects if no arguments are passed. The available arguments are:
/// - `--help`: Displays the help manual.
/// - `--list`: Lists all available projects in the user's Rust projects directory.
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
             .action(clap::ArgAction::Help)
             .help("Displays the manual page"))
        .arg(Arg::new("list")
             .short('l')
             .long("list")
             .help("Lists all available projects"))
        .get_matches();

    // Default to listing projects if no arguments are provided
    if matches.contains_id("list") || !matches.args_present() {
        // List projects by default, or explicitly if `--list` is passed
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let root_path = home_dir.join("rust");

        if !root_path.exists() {
            println!("Sorry, no Rust projects found.");
            return;
        }

        let projects = find_projects(&root_path);
        display_projects(&projects);
    }
}

