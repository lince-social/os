use dirs;
use std::io::ErrorKind;
use std::{collections::HashMap, fs};

fn main() -> std::io::Result<()> {
    let config_dir = dirs::config_dir().unwrap();
    let anicca_config = String::from(config_dir.to_str().unwrap()) + "/anicca/anicca";
    let contents = fs::read_to_string(anicca_config).unwrap();

    let mut v: Vec<&str> = Vec::new();
    for line in contents.lines() {
        v.push(line);
    }

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in v {
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
            if !contents.contains(&path_str.as_ref()) {
                let _ = match fs::remove_dir_all(&*path_str) {
                    Ok(file) => file,
                    Err(error) => match error.kind() {
                        ErrorKind::NotADirectory => match fs::remove_file(&*path_str) {
                            Ok(fd) => fd,
                            Err(e) => panic!("Problem deleting file: {e:?}"),
                        },
                        other_error => {
                            panic!("Other problem: {other_error:?}");
                        }
                    },
                };
            }
        }
    }

    Ok(())
}
