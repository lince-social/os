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
    // println!("dotfiles other: {dotfiles_other:#?}");
    for (destination, origin) in dotfiles_other {
        // println!("destination: {destination}");
        // println!("origin: {origin}");
        // println!("---");
        match dotfiles_method.as_str() {
            "remove and copy over" => {
                let _ = match fs::remove_dir_all(&*destination) {
                    Ok(file) => file,
                    Err(error) => match error.kind() {
                        ErrorKind::NotADirectory => {
                            match fs::remove_file(destination.to_string()) {
                                Ok(fd) => fd,
                                Err(e) => panic!("Problem deleting file: {e:?}"),
                            }
                        }
                        ErrorKind::NotFound => println!("Error not found hayaa: {destination}"),
                        other_error => {
                            panic!("Other problem: {other_error:?}");
                        }
                    },
                };

                fs::copy(origin, destination)?;
                println!("The remove and copy over method was used")
            }
            "copy over" => {
                println!("The copy over method was used")
            }
            "symlink" => {
                println!("The symlink method was used")
            }
            "cat" => {
                println!("The cat method was used")
            }
            &_ => panic!(
                "Method was not specified correctly, refer to documentation for further details."
            ),
        }
    }

    // for (key, _values) in &map {
    //     let dir = fs::read_dir(key)?;
    //     for entry in dir {
    //         let path = entry?.path();
    //         let path_str = path.to_string_lossy();
    //         if !nicca.contains(&path_str.to_string()) {
    //             let _ = match fs::remove_dir_all(&*path_str) {
    //                 Ok(file) => file,
    //                 Err(error) => match error.kind() {
    //                     ErrorKind::NotADirectory => match fs::remove_file(path_str.to_string()) {
    //                         Ok(fd) => fd,
    //                         Err(e) => panic!("Problem deleting file: {e:?}"),
    //                     },
    //                     ErrorKind::NotFound => println!("Error not found hayaa: {path_str}"),
    //                     other_error => {
    //                         panic!("Other problem: {other_error:?}");
    //                     }
    //                 },
    //             };
    //         }
    //     }
    // }

    Ok(())
}
