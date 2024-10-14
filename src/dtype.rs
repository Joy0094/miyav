use std::{env::args, io::{self, BufRead}, process::{exit, Command, Stdio}};
use colored::*; // Import the colored crate

pub fn handleargs() -> CMD {
    let args: Vec<String> = args().collect();

    // Check if there are at least two arguments: the command and at least one option
    if args.len() < 2 {
        display_help(); // Show help if no command is provided
        exit(0);
    }

    let pcmd = &args[1];
    let rocmd = match pcmd.as_str() {
        "update" | "upgrade" => String::new(), // No additional arguments needed for these commands
        "help" => { display_help(); String::new() },
        "install" | "remove" | "download" | "search" | "info" | "version" | "fix" => {
            if args.len() < 3 {
                eprintln!("{}", "ERROR: Please specify one or more package names.".red());
                exit(1);
            }
            args[2..].join(" ") // Join remaining arguments for the package names
        }
        _ => {
            eprintln!("{}: {}", "ERROR".red(), format!("Unknown command: {}", pcmd).yellow());
            eprintln!("{}", "Hint: Type 'miyav help' for a list of available commands.".green());
            exit(1);
        }
    };

    let cmds = CMD {
        pcmd: match pcmd.as_str() {
            "install" => Pcmds::Install,
            "remove" => Pcmds::Remove,
            "download" => Pcmds::Download,
            "update" => Pcmds::Update,
            "upgrade" => Pcmds::Upgrade,
            "search" => Pcmds::Search,
            "info" => Pcmds::Info,
            "version" => Pcmds::Version,
            "fix" => Pcmds::Fix,
            _ => Pcmds::EMP,
        },
        roa: rocmd,
    };
    cmds
}

#[allow(unused)]
pub struct CMD {
    pcmd: Pcmds,
    roa: String,
}

#[allow(unused)]
pub enum Pcmds {
    Remove,
    Install,
    Download,
    Update,
    Upgrade,
    Search,
    Info,
    Version, // Renamed from Policy to Version
    Fix,
    EMP, // Empty or unknown command
}

pub fn translate(command: &CMD) -> String {
    let apk_cmd = match command.pcmd {
        Pcmds::Install => format!("apk add {}", command.roa),
        Pcmds::Remove => format!("apk del {}", command.roa),
        Pcmds::Download => format!("apk fetch {}", command.roa),
        Pcmds::Update => "apk update".to_string(),
        Pcmds::Upgrade => format!("apk upgrade {}", command.roa),
        Pcmds::Search => format!("apk search {}", command.roa),
        Pcmds::Info => format!("apk info {}", command.roa),
        Pcmds::Version => format!("apk policy {}", command.roa),
        Pcmds::Fix => format!("apk fix {}", command.roa),
        Pcmds::EMP => "ERROR: Unknown command.".to_string(),
    };
    apk_cmd
}
//DEBUG
pub fn translate_debian(command: &CMD) -> String {
    let debian_cmd = match command.pcmd {
        Pcmds::Install => format!("apt install {}", command.roa),
        Pcmds::Remove => format!("apt remove {}", command.roa),
        Pcmds::Download => format!("apt download {}", command.roa),
        Pcmds::Update => "apt update".to_string(),
        Pcmds::Upgrade => format!("apt upgrade {}", command.roa),
        Pcmds::Search => format!("apt search {}", command.roa),
        Pcmds::Info => format!("apt show {}", command.roa),
        Pcmds::Version => format!("apt-cache policy {}", command.roa),
        Pcmds::Fix => format!("apt install --reinstall {}", command.roa),
        Pcmds::EMP => "ERROR: Unknown command.".to_string(),
    };
    debian_cmd
}
//END
pub fn display_help() {
    println!("Miyav Package Manager Help");
    println!("Available Commands:");
    println!("  install <pkg1> <pkg2> ... : Installs the specified packages.");
    println!("  remove <pkg1> <pkg2> ...  : Removes the specified packages.");
    println!("  upgrade [<pkg1> <pkg2> ...]: Updates the specified packages or all installed packages if none are specified.");
    println!("  update                     : Updates the package list from the repositories.");
    println!("  search <package-name>      : Searches for the specified package by name.");
    println!("  info <package-name>        : Provides detailed information about the specified package.");
    println!("  fetch <pkg1> <pkg2> ...    : Downloads the specified packages without installing them.");
    println!("  version <package-name>     : Displays the installed and available versions of the specified package.");
    println!("  fix [<package-name>]       : Reinstalls or repairs the specified package or all packages if none is specified.");
    println!("  stats                      : Shows statistics about installed packages and their usage.");
    println!();
    println!("Usage Examples:");
    println!("  miyav install vim");
    println!("  miyav remove vim");
    println!("  miyav upgrade");
    println!("  miyav search neovim");
    println!("  miyav info git");
    println!("  miyav fetch curl");
    println!("  miyav version openssh");
    println!("  miyav fix");
    println!("  miyav stats");
}

pub fn run_command(command: &str) {
    // Create a command to execute
    let mut parts = command.split_whitespace();
    let cmd = parts.next().unwrap();
    let args = parts;

    // Start the command and handle output
    let mut child = Command::new("sudo")
    .arg(cmd)
        .args(args)
        .stdout(Stdio::piped()) 
        .stderr(Stdio::piped()) 
        .spawn()
        .expect("Failed to start command.");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    // Create a reader for stdout
    let stdout_reader = io::BufReader::new(stdout);
    // Create a reader for stderr
    let stderr_reader = io::BufReader::new(stderr);

    // Read stdout in real-time
    for line in stdout_reader.lines() {
        match line {
            Ok(output) => println!("{}", output),
            Err(err) => eprintln!("Error reading stdout: {}", err),
        }
    }

    // Read stderr in real-time
    for line in stderr_reader.lines() {
        match line {
            Ok(output) => eprintln!("{}", output),
            Err(err) => eprintln!("Error reading stderr: {}", err),
        }
    }

    // Wait for the command to finish
    let _ = child.wait().expect("Command wasn't running");
}