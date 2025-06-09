use clap::{Parser, Subcommand};
use std::process::Command;
use std::path::PathBuf;
use std::fs;
use std::env;

#[derive(Parser)]
#[command(name = "teashop")]
#[command(about = "🫖 Manage and run Tea containers", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Flavors,
    Brews,
    Brew {
        flavor: String,
    },
    Drink {
        flavor: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Flavors => list_flavors(),
        Commands::Brews => list_brews(),
        Commands::Brew { flavor } => brew_flavor(flavor),
        Commands::Drink { flavor } => drink_flavor(flavor),
    }
}

fn list_flavors() {
    // Print logo first (equivalent to cat ./misc/logo_redstraw_nocap)
    print_logo("logo_redstraw_nocap");
    
    println!("Available flavors:");
    
    let flavors_dir = get_tea_path().join("flavors");
    
    // Check if flavors directory exists
    if !flavors_dir.exists() {
        eprintln!("Flavors directory not found at: {}", flavors_dir.display());
        return;
    }
    
    // Read directory entries
    let entries = match fs::read_dir(&flavors_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading flavors directory: {}", e);
            return;
        }
    };
    
    // Collect and sort flavor directories
    let mut flavors = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(flavor_name) = path.file_name().and_then(|n| n.to_str()) {
                    flavors.push((flavor_name.to_string(), path));
                }
            }
        }
    }
    
    // Sort flavors alphabetically
    flavors.sort_by(|a, b| a.0.cmp(&b.0));
    
    // Display each flavor with its description
    for (flavor_name, flavor_path) in flavors {
        println!();
        let description_file = flavor_path.join("description.txt");
        
        if description_file.exists() {
            println!("🍵 Flavor: {}", flavor_name);
            match fs::read_to_string(&description_file) {
                Ok(description) => print!("{}", description.trim_end()),
                Err(e) => println!("Error reading description: {}", e),
            }
        } else {
            println!("🍵 Flavor: {} (no description.txt found)", flavor_name);
        }
    }
}

fn print_logo(logo_name: &str) {
    let logo_path = get_tea_path().join("misc").join(logo_name);
    if let Ok(logo_content) = fs::read_to_string(&logo_path) {
        print!("{}", logo_content);
    }
    // Silently continue if logo doesn't exist
}

fn get_tea_path() -> PathBuf {
    // Always use ~/.tea since teashop will be run from anywhere in the OS
    if let Ok(home) = env::var("HOME") {
        PathBuf::from(home).join(".tea")
    } else {
        // Fallback if HOME environment variable isn't set
        PathBuf::from(".tea")
    }
}

// Stub implementations
fn list_brews() {
    // Print logo first (equivalent to cat ./misc/logo_bearcup from drink.sh)
    print_logo("logo_bearcup");
    
    println!("Available brews:");
    
    // Run docker images command to get all images
    let output = match Command::new("docker")
        .args(&["images", "--format", "table {{.Repository}}:{{.Tag}}\t{{.CreatedAt}}\t{{.Size}}"])
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error running docker command: {}", e);
            eprintln!("Make sure Docker is installed and running.");
            return;
        }
    };
    
    if !output.status.success() {
        eprintln!("Docker command failed:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();
    
    // Filter for tea:* images (skip header line)
    let mut tea_brews = Vec::new();
    for line in lines.iter().skip(1) {
        if line.starts_with("tea:") && !line.contains("tea:base") {
            tea_brews.push(*line);
        }
    }
    
    if tea_brews.is_empty() {
        println!("No tea brews found. Use 'teashop brew <flavor>' to create one.");
        return;
    }
    
    // Print header
    println!("🍵 REPOSITORY:TAG\t\t\tCREATED\t\t\tSIZE");
    println!("─────────────────────────────────────────────────────────────");
    
    // Print each tea brew
    for brew in tea_brews {
        println!("🫖 {}", brew);
    }
}

fn brew_flavor(flavor: &str) {
    // Print logo first (equivalent to cat ./misc/logo_redstraw_nocap from brew.sh)
    print_logo("logo_redstraw_nocap");
    
    let tea_path = get_tea_path();
    let flavor_dir = tea_path.join("flavors").join(flavor);
    let dockerfile_path = flavor_dir.join("Dockerfile");
    
    // Check if the flavor directory exists
    if !flavor_dir.exists() {
        println!("Flavor '{}' not found.", flavor);
        println!();
        list_available_flavors_brief();
        return;
    }
    
    // Check if Dockerfile exists
    if !dockerfile_path.exists() {
        println!("Dockerfile not found for flavor '{}'.", flavor);
        return;
    }
    
    // Build the Docker image
    println!("Brewing flavor: '{}'...", flavor);
    
    let status = Command::new("docker")
        .args(&[
            "build",
            "-t", &format!("tea:{}", flavor),
            "-f", dockerfile_path.to_str().unwrap(),
            tea_path.to_str().unwrap()
        ])
        .status();
    
    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("✅ Successfully brewed tea:{}", flavor);
            } else {
                println!("❌ Failed to brew flavor '{}'", flavor);
            }
        }
        Err(e) => {
            eprintln!("Error running docker build: {}", e);
            eprintln!("Make sure Docker is installed and running.");
        }
    }
}

fn drink_flavor(flavor: &str) {
    // Print logo first (equivalent to cat ./misc/logo_bearcup from drink.sh)
    print_logo("logo_bearcup");
    
    let image_name = format!("tea:{}", flavor);
    let container_name = format!("tea-{}-container", flavor);
    
    // Check if the image exists
    let image_check = Command::new("docker")
        .args(&["images", "-q", &image_name])
        .output();
    
    let image_exists = match image_check {
        Ok(output) => !String::from_utf8_lossy(&output.stdout).trim().is_empty(),
        Err(_) => false,
    };
    
    if !image_exists {
        println!("Brew for flavor '{}' not found. But you have these ones:", flavor);
        
        // Show available tea images
        if let Ok(output) = Command::new("docker")
            .args(&["images", "--format", "{{.Tag}}", "tea"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let available: Vec<&str> = output_str
                .lines()
                .filter(|line| !line.is_empty() && *line != "base")
                .collect();
            
            if available.is_empty() {
                println!("(none - use 'teashop brew <flavor>' first)");
            } else {
                println!("{}", available.join(" "));
            }
        }
        return;
    }
    
    // Check if container already exists
    let container_check = Command::new("docker")
        .args(&["ps", "-a", "-q", "-f", &format!("name={}", container_name)])
        .output();
    
    let container_exists = match container_check {
        Ok(output) => !String::from_utf8_lossy(&output.stdout).trim().is_empty(),
        Err(_) => false,
    };
    
    if !container_exists {
        // Create and start new container
        println!("Tea for brew '{}' not found. Creating and starting tea...", flavor);
        
        let status = Command::new("docker")
            .args(&[
                "run", "-it",
                "--name", &container_name,
                "--hostname", flavor,
                &image_name,
                "zsh"
            ])
            .status();
            
        match status {
            Ok(_) => println!("Tea '{}' started.", container_name),
            Err(e) => {
                eprintln!("Error starting container: {}", e);
                eprintln!("Make sure Docker is installed and running.");
            }
        }
    } else {
        // Start existing container and attach
        println!("Tea for brew '{}' already exists. Starting and attaching...", flavor);
        
        // Start the container
        let _ = Command::new("docker")
            .args(&["start", &container_name])
            .status();
        
        // Attach to the container
        let status = Command::new("docker")
            .args(&["attach", &container_name])
            .status();
            
        if let Err(e) = status {
            eprintln!("Error attaching to container: {}", e);
        }
    }
}

fn list_available_flavors_brief() {
    let tea_path = get_tea_path();
    let flavors_dir = tea_path.join("flavors");
    
    if let Ok(entries) = fs::read_dir(&flavors_dir) {
        let mut flavors = Vec::new();
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    flavors.push(name.to_string());
                }
            }
        }
        flavors.sort();
        
        if !flavors.is_empty() {
            println!("Available flavors: {}", flavors.join(", "));
        }
    }
}