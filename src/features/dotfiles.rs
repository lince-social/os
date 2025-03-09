// // use dirs;
// // use serde::Deserialize;
// // use std::io::ErrorKind;
// // use std::{collections::HashMap, fs};

// // #[derive(Deserialize, Debug)]
// // struct Config {
// //     dotfiles: Dotfiles,
// // }

// // #[derive(Deserialize, Debug)]
// // struct Dotfiles {
// //     method: String,
// //     #[serde(flatten)]
// //     other: HashMap<String, String>,
// // }

// // // pub fn dotfiles() -> std::io::Result<()> {
// // //     let config_dir = dirs::config_dir().unwrap();
// // //     let config_path = String::from(config_dir.to_str().unwrap()) + "/lince/os.toml";
// // //     let config_contents: Config = toml::from_str(&fs::read_to_string(config_path)?).unwrap();

// // //     let dotfiles_method = &config_contents.dotfiles.method;
// // //     println!("dotfiles method: {dotfiles_method}");
// // //     let dotfiles_other = &config_contents.dotfiles.other;

// // //     for (destination, origin) in dotfiles_other {
// // //         println!("Processing: destination={destination}, origin={origin}");
// // //         match dotfiles_method.as_str() {
// // //             "remove and copy over" => {
// // //                 let _ = match fs::remove_dir_all(&*destination) {
// // //                     Ok(dir) => {
// // //                         println!("deleted f:{dir:?} ||| destination: {destination}");
// // //                         fs::create_dir_all(&destination)?;
// // //                     },
// // //                     Err(error) => match error.kind() {
// // //                         ErrorKind::NotADirectory => {
// // //                             match fs::remove_file(destination.to_string()) {
// // //                                 Ok(file) =>{
// // //                                  println!("deleted file: {file:?}||{destination}");
// // //                                          fs::File::create(&destination)?;
// // //                                 }
// // //                                 Err(e) => panic!("Problem deleting file: {e:?}"),
// // //                             }
// // //                         }
// // //                         ErrorKind::NotFound => {
// // //                                 println!("Error not found hayaa: {destination}");
// // //                                 println!("origin: {origin}");
// // //                             let md = fs::metadata(origin).expect("problem with path");
// // //                             println!("{:?}", md.is_file());
// // //                             if md.is_file() {
// // //                                 println!("its a file, creating...");
// // //                                 fs::File::create(&destination)?;
// // //                                 println!("created: {destination}");
// // //                             } else {
// // //                                 println!("its a dir, creating...");
// // //                                 fs::create_dir_all(&destination)?;
// // //                                 println!("created: {destination}");
// // //                             }
// // //                         },
// // //                         other_error => {
// // //                             panic!("Other problem: {other_error:?}");
// // //                         }
// // //                     },
// // //                 };

// // //                 let pre = destination.split("/");
// // //                 let last = pre.clone().last().unwrap();

// // //                 let mut joined_pre = String::new();

// // //                 for part in pre {
// // //                     if part != last {
// // //                         joined_pre.push_str(part);
// // //                         joined_pre.push_str("/");
// // //                     }
// // //                 }

// // //                 if let Err(copy_error) = fs::copy(&origin, &joined_pre) {
// // //                     eprintln!("Failed to copy {origin} to {joined_pre}: {copy_error:?}");
// // //                 } else {
// // //                     println!("Copied {origin} to {joined_pre}");
// // //                 }
// // //             }
// // //             _ => println!("Unknown method: {dotfiles_method}"),
// // //         }
// // //     }
// // //     Ok(())
// // // }

// // use std::path::{Path, PathBuf};

// // fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
// //     fs::create_dir_all(&dst)?;
// //     for entry in fs::read_dir(src)? {
// //         let entry = entry?;
// //         let ty = entry.file_type()?;
// //         if ty.is_dir() {
// //             copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
// //         } else {
// //             fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
// //         }
// //     }
// //     Ok(())
// // }

// // pub fn dotfiles() -> std::io::Result<()> {
// //     let config_dir = dirs::config_dir().unwrap();
// //     let config_path = String::from(config_dir.to_str().unwrap()) + "/lince/os.toml";
// //     let config_contents: Config = toml::from_str(&fs::read_to_string(config_path)?).unwrap();

// //     let dotfiles_method = &config_contents.dotfiles.method;
// //     println!("dotfiles method: {dotfiles_method}");
// //     let dotfiles_other = &config_contents.dotfiles.other;

// //     for (destination, origin) in dotfiles_other {
// //         println!("Processing: destination={destination}, origin={origin}");
// //         match dotfiles_method.as_str() {
// //             "remove and copy over" => {
// //                 let _ = match fs::remove_dir_all(&*destination) {
// //                     Ok(dir) => {
// //                         println!("deleted f:{dir:?} ||| destination: {destination}");
// //                         fs::create_dir_all(&destination)?;
// //                     },
// //                     Err(error) => match error.kind() {
// //                         ErrorKind::NotADirectory => {
// //                             match fs::remove_file(destination.to_string()) {
// //                                 Ok(file) =>{
// //                                  println!("deleted file: {file:?}||{destination}");
// //                                          fs::File::create(&destination)?;
// //                                 }
// //                                 Err(e) => panic!("Problem deleting file: {e:?}"),
// //                             }
// //                         }
// //                         ErrorKind::NotFound => {
// //                                 println!("Error not found hayaa: {destination}");
// //                                 println!("origin: {origin}");
// //                             // let md = fs::metadata(origin).expect("problem with path");
// //                             // println!("{:?}", md.is_file());
// //                             // if md.is_file() {
// //                             //     println!("its a file, creating...");
// //                             //     fs::File::create(&destination)?;
// //                             //     println!("created: {destination}");
// //                             // } else {
// //                             //     println!("its a dir, creating...");
// //                             //     fs::create_dir_all(&destination)?;
// //                             //     println!("created: {destination}");
// //                             // }
// //                         },
// //                         other_error => {
// //                             panic!("Other problem: {other_error:?}");
// //                         }
// //                     },
// //                 };

// //                 if let Err(copy_error) = copy_dir_all(&origin, &destination) {
// //                     eprintln!("Failed to copy {origin} to {destination}: {copy_error:?}");
// //                 } else {
// //                     println!("Copied {origin} to {destination}");
// //                 }
// //             }
// //             _ => println!("Unknown method: {dotfiles_method}"),
// //         }
// //     }
// //     Ok(())
// // }
// use dirs;
// use serde::Deserialize;
// use std::io::ErrorKind;
// use std::{collections::HashMap, fs};
// use std::path::Path;

// #[derive(Deserialize, Debug)]
// struct Config {
//     dotfiles: Dotfiles,
// }

// #[derive(Deserialize, Debug)]
// struct Dotfiles {
//     method: String,
//     #[serde(flatten)]
//     other: HashMap<String, String>,
// }

// // Function to recursively copy directories
// fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
//     fs::create_dir_all(&dst)?;
//     for entry in fs::read_dir(src)? {
//         let entry = entry?;
//         let ty = entry.file_type()?;
//         if ty.is_dir() {
//             copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
//         } else {
//             fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
//         }
//     }
//     Ok(())
// }

// pub fn dotfiles() -> std::io::Result<()> {
//     let config_dir = dirs::config_dir().unwrap();
//     let config_path = String::from(config_dir.to_str().unwrap()) + "/lince/os.toml";
//     let config_contents: Config = toml::from_str(&fs::read_to_string(config_path)?).unwrap();

//     let dotfiles_method = &config_contents.dotfiles.method;
//     println!("dotfiles method: {dotfiles_method}");
//     let dotfiles_other = &config_contents.dotfiles.other;

//     for (destination, origin) in dotfiles_other {
//         println!("Processing: destination={destination}, origin={origin}");
//         match dotfiles_method.as_str() {
//             "remove and copy over" => {
//                 // Remove the destination if it exists
//                 let _ = match fs::remove_dir_all(&*destination) {
//                     Ok(_) => {
//                         println!("Deleted directory: {destination}");
//                     },
//                     Err(error) => match error.kind() {
//                         ErrorKind::NotADirectory => {
//                             match fs::remove_file(destination) {
//                                 Ok(_) => {
//                                     println!("Deleted file: {destination}");
//                                 },
//                                 Err(e) => panic!("Problem deleting file: {e:?}"),
//                             }
//                         }
//                         ErrorKind::NotFound => {
//                             println!("Destination not found: {destination}");
//                         },
//                         other_error => {
//                             panic!("Other problem: {other_error:?}");
//                         }
//                     },
//                 };

//                 // Check if the origin is a file or directory
//                 let md = fs::metadata(origin).expect("Problem with origin path");
//                 if md.is_file() {
//                     println!("It's a file, copying...");
//                     fs::copy(origin, destination)?;
//                     println!("Copied file: {origin} to {destination}");
//                 } else if md.is_dir() {
//                     println!("It's a directory, copying...");
//                     copy_dir_all(origin, destination)?;
//                     println!("Copied directory: {origin} to {destination}");
//                 } else {
//                     panic!("Origin is neither a file nor a directory: {origin}");
//                 }
//             }
//             _ => println!("Unknown method: {dotfiles_method}"),
//         }
//     }
//     Ok(())
// }
use dirs;
use serde::Deserialize;
use std::io::ErrorKind;
use std::{collections::HashMap, fs};
use std::path::Path;

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

// Function to recursively copy directories
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
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
                // Remove the destination if it exists
                let _ = match fs::remove_dir_all(&*destination) {
                    Ok(_) => {
                        println!("Deleted directory: {destination}");
                    },
                    Err(error) => match error.kind() {
                        ErrorKind::NotADirectory => {
                            match fs::remove_file(destination) {
                                Ok(_) => {
                                    println!("Deleted file: {destination}");
                                },
                                Err(e) => panic!("Problem deleting file: {e:?}"),
                            }
                        }
                        ErrorKind::NotFound => {
                            println!("Destination not found: {destination}");
                        },
                        other_error => {
                            panic!("Other problem: {other_error:?}");
                        }
                    },
                };

                // Check if the origin is a file or directory
                let md = fs::metadata(origin).expect("Problem with origin path");
                if md.is_file() {
                    println!("It's a file, copying...");

                    // Ensure the parent directory of the destination exists
                    let dest_path = Path::new(destination);
                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent)?;
                        println!("Created parent directory: {}", parent.display());
                    }

                    // Copy the file
                    fs::copy(origin, destination)?;
                    println!("Copied file: {origin} to {destination}");
                } else if md.is_dir() {
                    println!("It's a directory, copying...");
                    copy_dir_all(origin, destination)?;
                    println!("Copied directory: {origin} to {destination}");
                } else {
                    panic!("Origin is neither a file nor a directory: {origin}");
                }
            }
            _ => println!("Unknown method: {dotfiles_method}"),
        }
    }
    Ok(())
}
