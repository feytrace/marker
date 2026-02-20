use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Parser, Debug)]
#[command(
    name = "marker_cli",
    version,
    about = "Persistent directory markers CLI"
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Set {
        #[arg(short, long)]
        flag: String,
        #[arg(short, long)]
        directory: PathBuf,
    },
    Delete {
        #[arg(short, long)]
        flag: Option<String>,

        #[arg(short)]
        recursive: bool,
    },
    Retrieve {
        #[arg(short, long)]
        flag: String,
    },
    List,
}

#[derive(Serialize, Deserialize, Debug)]
struct Markers {
    map: HashMap<String, PathBuf>,
}

impl Markers {
    fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self {
                map: HashMap::new(),
            })
        }
    }

    fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(&self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}

fn get_config_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "example", "marker_cli") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir).expect("Unable to create Makrer directory");
        config_dir.join("markers.json")
    } else {
        panic!("Could not determine config directory");
    }
}

fn handle_set(
    config_path: &Path,
    flag: String,
    dir: PathBuf,
    markers: &mut Markers,
) -> Result<(), Box<dyn std::error::Error>> {
    markers.map.insert(flag, dir);
    markers.save(config_path)?;
    Ok(())
}

fn handle_delete(
    config_path: &Path,
    flag: Option<String>,
    recursive: bool,
    markers: &mut Markers,
) -> Result<(), Box<dyn std::error::Error>> {
    if recursive {
        markers.map.clear();
    } else if let Some(f) = flag {
        markers.map.remove(&f);
    } else {
        return Err("Either a flag or recursive must be specified".into());
    }
    markers.save(config_path)?;
    Ok(())
}

fn handle_retrieve(flag: String, markers: &Markers) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(dir) = markers.map.get(&flag) {
        println!("{}", dir.display());
        Ok(())
    } else {
        Err(format!("Marker '{}' does not exist", flag).into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_path = get_config_path();
    let mut markers = Markers::load(&config_path)?;

    match args.command {
        Command::Set { flag, directory } => {
            handle_set(&config_path, flag, directory, &mut markers)?
        }
        Command::Delete { flag, recursive } => {
            handle_delete(&config_path, flag, recursive, &mut markers)?
        }
        Command::Retrieve { flag } => handle_retrieve(flag, &markers)?,
        Command::List => {
            if markers.map.is_empty() {
                println!("No markers set.");
            } else {
                println!("All markers: {:#?}", markers.map);
            }
        }
    }

    Ok(())
}
