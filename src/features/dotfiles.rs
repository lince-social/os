use dirs;
use serde::Deserialize;
use std::io::{ErrorKind, Result};
use std::path::Path;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug)]
struct Config {
    dotfiles: Dotfiles,
}

#[derive(Deserialize, Debug)]
struct Dotfiles {
    method: String,
    #[serde(flatten)]
    other: HashMap<String, String>,
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn dotfiles() -> Result<()> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = String::from(config_dir.to_str().unwrap()) + "/lince/os.toml";
    let config_contents: Config = toml::from_str(&fs::read_to_string(config_path)?).unwrap();

    let dotfiles_method = &config_contents.dotfiles.method;
    let dotfiles_other = &config_contents.dotfiles.other;

    for (destination, origin) in dotfiles_other {
        match dotfiles_method.as_str() {
            "remove and copy over" => {
                let _ = match fs::remove_dir_all(&*destination) {
                    Ok(_) => {
                        println!("Deleted directory: {destination}");
                    }
                    Err(error) => match error.kind() {
                        ErrorKind::NotADirectory => match fs::remove_file(destination) {
                            Ok(_) => {
                                println!("Deleted file: {destination}");
                            }
                            Err(e) => panic!("Problem deleting file: {e:?}"),
                        },
                        ErrorKind::NotFound => {
                            println!("Destination not found: {destination}");
                        }
                        other_error => {
                            panic!("Other problem: {other_error:?}");
                        }
                    },
                };

                let md = fs::metadata(origin).expect("Problem with origin path");
                if md.is_file() {
                    println!("It's a file, copying...");

                    let dest_path = Path::new(destination);
                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent)?;
                        println!("Created parent directory: {}", parent.display());
                    }

                    fs::copy(origin, destination)?;
                } else if md.is_dir() {
                    copy_dir_all(origin, destination)?;
                } else {
                    panic!("Origin is neither a file nor a directory: {origin}");
                }
            }
            _ => println!("Unknown method: {dotfiles_method}"),
        }
    }
    Ok(())
}
