use dirs;
use serde::Deserialize;
use std::fmt::Display;
use std::io::ErrorKind;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug)]
struct Config {
    nicca: Nicca,
    dotfiles: Dotfiles,
}

#[derive(Deserialize, Debug)]
struct Nicca {
    list: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Dotfiles {
    method: String,
    #[serde(flatten)]
    other: HashMap<String, String>,
}

fn main() -> std::io::Result<()> {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = String::from(config_dir.to_str().unwrap()) + "/lince/os.toml";
    let config_contents: Config = toml::from_str(&fs::read_to_string(config_path)?).unwrap();

    let nicca = &config_contents.nicca.list;
    // println!("nicca: {:?}", nicca);

    // let mut v: Vec<&str> = Vec::new();
    // for line in nicca.lines() {
    //     v.push(line);
    // }

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in nicca {
        let pre = line.split("/");
        let last = pre.clone().last().unwrap();

        let mut joined_pre = String::new();

        for part in pre {
            if part != last {
                joined_pre.push_str(part);
                joined_pre.push_str("/");
            }
        }
        map.entry(joined_pre).or_default().push(last.to_string());
    }
    for (key, _values) in &map {
        let dir = fs::read_dir(key)?;
        for entry in dir {
            let path = entry?.path();
            let path_str = path.to_string_lossy();
            if !nicca.contains(&path_str.to_string()) {
                let _ = match fs::remove_dir_all(&*path_str) {
                    Ok(file) => file,
                    Err(error) => match error.kind() {
                        ErrorKind::NotADirectory => match fs::remove_file(path_str.to_string()) {
                            Ok(fd) => fd,
                            Err(e) => panic!("Problem deleting file: {e:?}"),
                        },
                        ErrorKind::NotFound => println!("Error not found hayaa: {path_str}"),
                        other_error => {
                            panic!("Other problem: {other_error:?}");
                        }
                    },
                };
            }
        }
    }
    // // let dotfiles_method = &config_contents.dotfiles.method;
    // let dotfiles_other = &config_contents.dotfiles.other;
    // // println!("{dotfiles_other:#?}");
    // for (key, value) in dotfiles_other {}

    Ok(())
}
