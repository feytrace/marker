use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Parser, Debug)]
#[command(version, about = "Persistent directory markers CLI")]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Mode::Retrieve)]
    mode: Mode,

    flag: String,

    directory: Option<String>,

    #[arg(short, long)]
    list: bool,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Set,
    Delete,
    Retrieve,
}

#[derive(Serialize, Deserialize, Debug)]
struct Markers {
    map: HashMap<String, String>,
}

impl Markers {
    fn load(path: &PathBuf) -> Self {
        if path.exists() {
            let content = fs::read_to_string(path).expect("Failed to read markers file");
            serde_json::from_str(&content).unwrap_or(Self { map: HashMap::new() })
        } else {
            Self { map: HashMap::new() }
        }
    }

    fn save(&self, path: &PathBuf) {
        let content = serde_json::to_string_pretty(&self).expect("Failed to serialize markers");
        fs::write(path, content).expect("Failed to write markers file");
    }
}

fn get_config_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "example", "marker_cli") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir).expect("Failed to create config directory");
        config_dir.join("markers.json")
    } else {
        panic!("Could not determine config directory");
    }
}

fn main() {
    let args = Args::parse();
    let config_path = get_config_path();
    
    if args.verbose {
        println!("Using markers file: {}", config_path.display());
    }

    let mut markers = Markers::load(&config_path);

    match args.mode {
        Mode::Set => {
            let dir = args.directory.expect("Directory is required when using set mode");
            markers.map.insert(args.flag.clone(), dir.clone());
            println!("Set marker '{}' -> '{}'", args.flag, dir);
        }
        Mode::Delete => {
            if markers.map.remove(&args.flag).is_some() {
                println!("Deleted marker '{}'", args.flag);
            } else {
                println!("Marker '{}' does not exist", args.flag);
            }
        }
        Mode::Retrieve => {
            if let Some(dir) = markers.map.get(&args.flag) {
                println!("{}", dir);
            } else {
                println!("Marker '{}' does not exist", args.flag);
            }
        }
    }

    if args.list {
        println!("All markers: {:#?}", markers.map);
    }

    markers.save(&config_path);
}
