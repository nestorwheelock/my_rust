
# My Rust Manager

**My Rust Manager** is a command-line tool for managing and displaying information about your local Rust projects. It scans a specified directory (e.g., your `~/rust` folder) for Rust projects and allows you to list, view details, and manage them directly from the terminal.

## Features

- Lists all Rust projects in a specified directory
- Displays project details including name, description, and path
- Provides a simple command-line interface
- Handles graceful shutdowns with Ctrl+C

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/my_rust.git
   cd my_rust
   ```

2. Build the project using `cargo`:
   ```bash
   cargo build --release
   ```

3. Optionally, install the binary globally:
   ```bash
   cargo install --path .
   ```

## Usage

Once you have built or installed the tool, you can run it from the command line. By default, the tool will search for Rust projects in the `~/rust` directory and list them.

### Listing Projects

To list all available Rust projects, simply run:

```bash
./my_rust
```

or

```bash
./my_rust --list
```

This will display a list of Rust projects found in the specified directory.

### Viewing Project Details

When projects are listed, you can enter the number corresponding to the project to view more detailed information, including:

- Project name
- Description (if available)
- Path to the project

Example interaction:

```bash
1. project_one - My first Rust project
2. project_two - Another Rust project
3. project_three - No description

Enter the number of the project to view details, or 'q' to quit:
> 1
Project Name: project_one
Description: My first Rust project
Path: /home/user/rust/project_one
You can run this project from: /home/user/rust/project_one/target/release
```

### Help Menu

For help, you can use the `--help` flag:

```bash
./my_rust --help
```

This will display a list of available options.

## Configuration

By default, `My Rust Manager` searches for Rust projects in the `~/rust` directory. You can change the search directory by modifying the code in the `main.rs` file. In future releases, we may introduce a configuration file to allow this to be set without modifying code.

## Contribution

Contributions are welcome! Feel free to submit a pull request or open an issue if you encounter a bug or have a feature request.

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Commit your changes: `git commit -m 'Add some feature'`
4. Push to the branch: `git push origin feature/your-feature-name`
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

This project was built using Rust and relies on the following crates:

- [clap](https://crates.io/crates/clap) for argument parsing
- [toml](https://crates.io/crates/toml) for parsing `Cargo.toml` files
- [dirs](https://crates.io/crates/dirs) for retrieving the home directory
- [ctrlc](https://crates.io/crates/ctrlc) for handling Ctrl+C gracefully
