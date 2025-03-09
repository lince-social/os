use dirs;
use serde::Deserialize;
use std::io::ErrorKind;
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

pub fn dotfiles() -> std::io::Result<()> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = String::from(config_dir.to_str().unwrap()) + "/lince/os.toml";
    let config_contents: Config = toml::from_str(&fs::read_to_string(config_path)?).unwrap();

    let dotfiles_method = &config_contents.dotfiles.method;
    println!("dotfiles method: {dotfiles_method}");
    let dotfiles_other = &config_contents.dotfiles.other;

    for (destination, origin) in dotfiles_other {
        println!("Processing: destination={destination}, origin={origin}");
        match dotfiles_method.as_str() {
            "remove and copy over" => {
                let _ = match fs::remove_dir_all(&*destination) {
                    Ok(dir) => {
                        println!("deleted f:{dir:?} ||| destination: {destination}");
                        fs::create_dir_all(&destination)?;
                    },
                    Err(error) => match error.kind() {
                        ErrorKind::NotADirectory => {
                            match fs::remove_file(destination.to_string()) {
                                Ok(file) =>{
                                 println!("deleted file: {file:?}||{destination}");
                                         fs::File::create(&destination)?;
                                }
                                Err(e) => panic!("Problem deleting file: {e:?}"),
                            }
                        }
                        ErrorKind::NotFound => {
                            println!("Error not found hayaa: {destination}");
                            let md = match fs::metadata(destination).unwrap();
                            if md.is_file() {
                                fs::File::create(&destination)?;
                            } else {
                                fs::create_dir_all(&destination)?;
                            }
                        },
                        other_error => {
                            panic!("Other problem: {other_error:?}");
                        }
                    },
                };

                let pre = destination.split("/");
                let last = pre.clone().last().unwrap();

                let mut joined_pre = String::new();

                for part in pre {
                    if part != last {
                        joined_pre.push_str(part);
                        joined_pre.push_str("/");
                    }
                }

                if let Err(copy_error) = fs::copy(&origin, &joined_pre) {
                    eprintln!("Failed to copy {origin} to {joined_pre}: {copy_error:?}");
                } else {
                    println!("Copied {origin} to {joined_pre}");
                }
            }
            _ => println!("Unknown method: {dotfiles_method}"),
        }
    }
    Ok(())
}
