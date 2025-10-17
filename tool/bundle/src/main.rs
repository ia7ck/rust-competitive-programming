use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;

fn main() -> Result<()> {
    let args = Args::parse();

    let workspace_path = if args.remote {
        download_remote_repository()?
    } else {
        args.workspace.into()
    };

    let bundled_code = bundle_crate(&args.crate_name, &workspace_path)
        .with_context(|| format!("Failed to bundle crate '{}'", args.crate_name))?;

    if args.skip_compile {
        println!("{}", bundled_code);
    } else {
        match check_compilation(&bundled_code) {
            Ok(()) => {
                println!("{}", bundled_code);
            }
            Err(e) => {
                eprintln!("❌ Compilation check failed: {}", e);
                eprintln!("Generated code:");
                println!("{}", bundled_code);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Option<Package>,
    dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Dependency {
    Simple(String),
    Detailed { path: Option<String> },
}

#[derive(Parser)]
#[clap(about = "Bundle Rust crates for competitive programming")]
struct Args {
    /// The name of the crate to bundle
    crate_name: String,

    /// Path to the workspace root
    #[clap(short, long, default_value = ".", conflicts_with = "remote")]
    workspace: String,

    /// Use remote repository (https://github.com/ia7ck/rust-competitive-programming)
    #[clap(long, conflicts_with = "workspace")]
    remote: bool,

    /// Skip compilation check
    #[clap(long)]
    skip_compile: bool,
}

fn download_remote_repository() -> Result<std::path::PathBuf> {
    eprintln!("📥 Cloning remote repository...");

    let temp_dir = std::env::temp_dir().join(format!(
        "rust-competitive-programming-{}",
        std::process::id()
    ));

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).context("Failed to remove existing temp directory")?;
    }

    let output = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "https://github.com/ia7ck/rust-competitive-programming.git",
        ])
        .arg(&temp_dir)
        .output()
        .context("Failed to run git clone. Make sure git is installed and available in PATH.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git clone failed:\n{}", stderr);
    }

    eprintln!("✅ Repository cloned to temporary directory");

    Ok(temp_dir)
}

struct CrateInfo {
    content: String,
    dependencies: Vec<String>,
    external_dependencies: Vec<String>,
}

fn list_available_crates(crates: &HashMap<String, CrateInfo>) -> String {
    let mut crate_names: Vec<_> = crates.keys().collect();
    crate_names.sort();

    let mut result = String::new();
    result.push_str("Available crates:\n");
    for name in crate_names {
        result.push_str(&format!("  - {}\n", name));
    }
    result
}

fn bundle_crate(crate_name: &str, workspace_path: &Path) -> Result<String> {
    let libs_path = workspace_path.join("libs");

    let mut crates = HashMap::new();
    collect_crates(&libs_path, &mut crates)?;

    let target_crate_info = crates.get(crate_name).ok_or_else(|| {
        let available_crates = list_available_crates(&crates);
        anyhow::anyhow!(
            "Crate '{}' not found in workspace.\n\n{}",
            crate_name,
            available_crates
        )
    })?;

    check_external_dependencies(crate_name, &crates);

    let mut all_dependencies = HashSet::new();
    collect_all_dependencies(crate_name, &crates, &mut all_dependencies);

    let mut bundled_code = String::new();
    bundled_code.push_str("// Bundled\n");

    bundled_code.push_str("#[rustfmt::skip]\n");
    bundled_code.push_str("#[allow(unused)]\n");
    bundled_code.push_str(&format!("mod {} {{\n", crate_name));
    let final_content = process_crate_content(&target_crate_info.content);

    for line in final_content.lines() {
        if line.trim().is_empty() {
            bundled_code.push('\n');
        } else {
            bundled_code.push_str(&format!("    {}\n", line));
        }
    }

    for dep_crate_name in &all_dependencies {
        if dep_crate_name != crate_name
            && let Some(dep_crate_info) = crates.get(dep_crate_name) {
                bundled_code.push_str(&format!("\n    mod {} {{\n", dep_crate_name));

                let processed_content = process_crate_content(&dep_crate_info.content);

                for line in processed_content.lines() {
                    if line.trim().is_empty() {
                        bundled_code.push('\n');
                    } else {
                        bundled_code.push_str(&format!("        {}\n", line));
                    }
                }

                bundled_code.push_str("    }\n");
            }
    }

    bundled_code.push_str("}\n");

    Ok(bundled_code)
}

fn collect_crates(libs_path: &Path, crates: &mut HashMap<String, CrateInfo>) -> Result<()> {
    if !libs_path.exists() {
        anyhow::bail!(
            "Libraries directory '{}' does not exist. Please check your workspace path.",
            libs_path.display()
        );
    }

    if !libs_path.is_dir() {
        anyhow::bail!(
            "'{}' is not a directory. Expected a directory containing crate libraries.",
            libs_path.display()
        );
    }

    for entry in fs::read_dir(libs_path)
        .with_context(|| format!("Failed to read directory '{}'", libs_path.display()))?
    {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let crate_path = entry.path();
            let cargo_toml_path = crate_path.join("Cargo.toml");

            if cargo_toml_path.exists() {
                let cargo_content = fs::read_to_string(&cargo_toml_path)?;
                let cargo_toml: CargoToml = toml::from_str(&cargo_content)?;

                if let Some(package) = cargo_toml.package {
                    let lib_path = crate_path.join("src").join("lib.rs");
                    if lib_path.exists() {
                        let lib_content = fs::read_to_string(&lib_path)?;

                        let mut dependencies = Vec::new();
                        let mut external_dependencies = Vec::new();
                        if let Some(deps) = cargo_toml.dependencies {
                            for (name, dep) in deps {
                                match dep {
                                    Dependency::Detailed { path: Some(_) } => {
                                        dependencies.push(name);
                                    }
                                    Dependency::Simple(_) | Dependency::Detailed { path: None } => {
                                        external_dependencies.push(name);
                                    }
                                }
                            }
                        }

                        crates.insert(
                            package.name.clone(),
                            CrateInfo {
                                content: lib_content,
                                dependencies,
                                external_dependencies,
                            },
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn collect_all_dependencies(
    crate_name: &str,
    crates: &HashMap<String, CrateInfo>,
    all_dependencies: &mut HashSet<String>,
) {
    if all_dependencies.contains(crate_name) {
        return;
    }

    all_dependencies.insert(crate_name.to_string());

    if let Some(crate_info) = crates.get(crate_name) {
        for dep in &crate_info.dependencies {
            collect_all_dependencies(dep, crates, all_dependencies);
        }
    }
}

fn check_external_dependencies(crate_name: &str, crates: &HashMap<String, CrateInfo>) {
    let mut external_deps = HashSet::new();
    collect_external_dependencies(crate_name, crates, &mut external_deps, &mut HashSet::new());

    if !external_deps.is_empty() {
        eprintln!("⚠️  Warning: External dependencies detected:");
        for dep in &external_deps {
            eprintln!("   - {}", dep);
        }
        eprintln!("   Consider using --skip-compile flag if compilation fails.");
        eprintln!();
    }
}

fn collect_external_dependencies(
    crate_name: &str,
    crates: &HashMap<String, CrateInfo>,
    external_deps: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) {
    if visited.contains(crate_name) {
        return;
    }
    visited.insert(crate_name.to_string());

    if let Some(crate_info) = crates.get(crate_name) {
        for dep in &crate_info.external_dependencies {
            external_deps.insert(dep.clone());
        }
        for dep in &crate_info.dependencies {
            collect_external_dependencies(dep, crates, external_deps, visited);
        }
    }
}

fn process_crate_content(content: &str) -> String {
    let mut processed_lines = Vec::new();
    let mut in_test_section = false;
    let mut brace_depth = 0;
    let mut skip_until_closing_brace = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("//") || trimmed.starts_with("///") {
            continue;
        }

        if trimmed.starts_with("#[cfg(test)]") {
            skip_until_closing_brace = true;
            continue;
        }

        if trimmed.starts_with("#[test]") {
            skip_until_closing_brace = true;
            continue;
        }

        if trimmed.contains("mod tests") && trimmed.contains('{') {
            in_test_section = true;
            brace_depth = 1;
            continue;
        }

        if skip_until_closing_brace || in_test_section {
            for ch in line.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => {
                        brace_depth -= 1;
                        if brace_depth == 0 {
                            skip_until_closing_brace = false;
                            in_test_section = false;
                        }
                    }
                    _ => {}
                }
            }
            continue;
        }

        processed_lines.push(line.to_string());
    }

    processed_lines.join("\n")
}

fn check_compilation(code: &str) -> Result<()> {
    let temp_dir = std::env::temp_dir().join("bundle_check");
    fs::create_dir_all(&temp_dir)?;

    let temp_file = temp_dir.join("main.rs");
    fs::write(&temp_file, code)?;

    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2024")
        .arg("-o")
        .arg(temp_dir.join("check"))
        .arg(&temp_file)
        .output()
        .context("Failed to run rustc")?;

    let _ = fs::remove_file(&temp_file);
    let _ = fs::remove_dir_all(&temp_dir);

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Compilation failed:\n{}", stderr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_crate_content_basic() {
        let input = r#"pub fn hello() {
    println!("hello");
}"#;
        let result = process_crate_content(input);
        assert_eq!(
            result,
            r#"pub fn hello() {
    println!("hello");
}"#
        );
    }

    #[test]
    fn test_process_crate_content_removes_comments() {
        let input = r#"// This is a comment
pub fn hello() {
    // Another comment
    println!("hello");
}
/// Doc comment"#;
        let result = process_crate_content(input);
        assert_eq!(
            result,
            r#"pub fn hello() {
    println!("hello");
}"#
        );
    }

    #[test]
    fn test_process_crate_content_removes_cfg_test() {
        let input = r#"#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert_eq!(1, 1);
    }
}
pub fn hello() {}"#;
        let result = process_crate_content(input);
        assert_eq!(result, "pub fn hello() {}");
    }

    #[test]
    fn test_process_crate_content_removes_test_attribute() {
        let input = r#"#[test]
fn test_function() {
    assert!(true);
}
pub fn normal_function() {}"#;
        let result = process_crate_content(input);
        assert_eq!(result, "pub fn normal_function() {}");
    }

    #[test]
    fn test_process_crate_content_removes_mod_tests() {
        let input = r#"pub fn hello() {}
mod tests {
    use super::*;
    #[test]
    fn test_hello() {
        hello();
    }
}"#;
        let result = process_crate_content(input);
        assert_eq!(result, "pub fn hello() {}");
    }

    #[test]
    fn test_process_crate_content_nested_braces() {
        let input = r#"#[cfg(test)]
mod tests {
    fn helper() {
        if true {
            println!("nested");
        }
    }
}
pub fn main_fn() {}"#;
        let result = process_crate_content(input);
        assert_eq!(result, "pub fn main_fn() {}");
    }

    #[test]
    fn test_process_crate_content_empty_lines() {
        let input = r#"pub fn hello() {}

// Comment

pub fn world() {}"#;
        let result = process_crate_content(input);

        let expected = r#"pub fn hello() {}


pub fn world() {}"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_list_available_crates() {
        let mut crates = HashMap::new();
        crates.insert(
            "crate_a".to_string(),
            CrateInfo {
                content: "pub fn a() {}".to_string(),
                dependencies: vec![],
                external_dependencies: vec![],
            },
        );
        crates.insert(
            "crate_b".to_string(),
            CrateInfo {
                content: "pub fn b() {}".to_string(),
                dependencies: vec![],
                external_dependencies: vec![],
            },
        );

        let result = list_available_crates(&crates);
        assert!(result.contains("Available crates:"));
        assert!(result.contains("  - crate_a"));
        assert!(result.contains("  - crate_b"));
    }
}
