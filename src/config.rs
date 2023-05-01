use std::process::Command;

use directories::BaseDirs;

fn create_config(path: &std::path::Path) -> Result<(), std::io::Error> {
    const DEFAULT_VOWELS: [char; 5] = ['a', 'e', 'i', 'u', 'o'];
    let file = if path.exists() {
        std::fs::File::open(&path)
    } else {
        let f = std::fs::File::create(&path);
        if let Err(e) = std::fs::write(
            &path,
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

fn read_config(path: &std::path::Path) -> Result<String, std::io::Error> {
    match std::fs::read_to_string(&path) {
        Ok(str) => Ok(str),
        Err(e) => {
            eprintln!("An error occured while reading the config\n{:#?}", e);
            Err(e)
        }
    }
}

pub fn config_check() -> Result<Vec<char>, std::io::Error> {
    let path = BaseDirs::new()
        .unwrap()
        .config_dir()
        .join("learn-japanese.conf");
    println!("Config path : {}", path.as_path().to_str().unwrap());
    match create_config(&path) {
        Ok(_) => {}
        Err(e) => return Err(e),
    };
    let mut config = read_config(&path);
    println!("Do you want to edit your config ? [Y/N]");
    let mut answer = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut answer) {
        return Err(e);
    }
    if "Y".eq_ignore_ascii_case(&answer.trim_end()) {
        if cfg!(target_os = "windows") {
            Command::new("powershell.exe")
                .arg("-c")
                .arg("start")
                .arg("-Wait")
                .arg("notepad.exe")
                .arg(&path.as_path().to_str().unwrap())
                .spawn()
                .expect("failed to execute process")
                .wait()
                .expect("Error: Editor returned a non-zero value");
        } else {
            let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
            Command::new("/usr/bin/sh")
                .arg("-c")
                .arg(format!("{} {}", editor, &path.as_path().to_str().unwrap()))
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
        .map(|it| it.split(":").last().unwrap())
        .flat_map(|it| it.chars())
        .collect())
}
