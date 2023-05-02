use std::{fs, io, path::Path, process::Command};

use directories::BaseDirs;

fn create_config(path: &Path) -> Result<(), io::Error> {
    const DEFAULT_VOWELS: [char; 5] = ['a', 'e', 'i', 'u', 'o'];
    let file = if path.exists() {
        fs::File::open(path)
    } else {
        let f = fs::File::create(path);
        if let Err(e) = fs::write(
            path,
            format!("vowels:{}", String::from_iter(DEFAULT_VOWELS)),
        ) {
            eprintln!(
                "An error occured while writing the default config\n{:#?}",
                e
            );
            return Err(e);
        }
        f
    };
    file.map(|_| Ok(())).unwrap()
}

fn read_config(path: &Path) -> Result<String, io::Error> {
    match fs::read_to_string(path) {
        Ok(str) => Ok(str),
        Err(e) => {
            eprintln!("An error occured while reading the config\n{:#?}", e);
            Err(e)
        }
    }
}

pub fn config_check() -> Result<Vec<char>, io::Error> {
    let path = BaseDirs::new()
        .unwrap()
        .config_dir()
        .join("learn-japanese.conf");
    println!("Config path : {}", path.as_path().to_str().unwrap());
    create_config(&path)?;
    let mut config = read_config(&path);
    println!("Do you want to edit your config ? [Y/N]");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    if "Y".eq_ignore_ascii_case(answer.trim_end()) {
        if cfg!(target_os = "windows") {
            Command::new("powershell.exe")
                .arg("-c")
                .arg("start")
                .arg("-Wait")
                .arg("notepad.exe")
                .arg(path.as_path().to_str().unwrap())
                .spawn()
                .expect("failed to execute process")
                .wait()
                .expect("Error: Editor returned a non-zero value");
        } else {
            let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
            Command::new("/usr/bin/sh")
                .arg("-c")
                .arg(format!("{} {}", editor, path.as_path().to_str().unwrap()))
                .spawn()
                .expect("failed to execute process")
                .wait()
                .expect("Error: Editor returned a non-zero value");
        };
        config = read_config(&path);
    }
    Ok(config
        .ok()
        .unwrap()
        .lines()
        .filter(|it| it.starts_with("vowels:"))
        .map(|it| it.split(':').last().unwrap())
        .flat_map(|it| it.chars())
        .collect())
}
