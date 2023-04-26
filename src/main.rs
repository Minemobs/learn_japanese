extern crate directories;

use directories::BaseDirs;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::fs::{read_to_string, write};
use std::io::stdin;

struct Hiragana {
    japanese: char,
    romanized: &'static str,
    #[allow(dead_code)]
    vowel: char,
}

impl Hiragana {
    fn new(japanese: char, romanized: &'static str) -> Self {
        Hiragana {
            japanese,
            romanized,
            vowel: match romanized {
                "n" => 'a',
                _ => romanized.chars().last().unwrap(),
            },
        }
    }

    fn from(tuple: (char, &'static str)) -> Self {
        Hiragana::new(tuple.0, tuple.1)
    }
}

fn config_check() -> Result<Vec<char>, std::io::Error> {
    const DEFAULT_VOWELS: [char; 5] = ['a', 'e', 'i', 'u', 'o'];
    let path = BaseDirs::new()
        .unwrap()
        .config_dir()
        .join("learn-japanese.conf");
    println!("Config path : {}", path.as_path().to_str().unwrap());
    let file = if path.exists() {
        std::fs::File::open(&path)
    } else {
        let f = std::fs::File::create(&path);
        if let Err(e) = write(
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
    if file.is_err() {
        return Err(file.err().unwrap());
    }
    let config = read_to_string(&path);
    if config.is_err() {
        eprintln!(
            "An error occured while reading the config\n{:#?}",
            config.as_ref().err().unwrap()
        );
        return Err(config.err().unwrap());
    }
    //TODO: ask if user want to change config
    Ok(config
        .ok()
        .unwrap()
        .lines()
        .filter(|it| it.starts_with("vowels:"))
        .map(|it| it.split(":").last().unwrap())
        .flat_map(|it| it.chars())
        .collect())
}

fn main() {
    let mut points = 0;
    let mut rng = thread_rng();

    let mut hiraganas = [
        //A
        ('あ', "a"),
        ('か', "ka"),
        ('さ', "sa"),
        ('た', "ta"),
        ('な', "na"),
        ('は', "ha"),
        ('ま', "ma"),
        ('や', "ya"),
        ('ら', "ra"),
        ('わ', "wa"),
        ('ん', "n"),
        //I
        ('い', "i"),
        ('き', "ki"),
        ('し', "shi"),
        ('ち', "chi"),
        ('に', "ni"),
        ('ひ', "hi"),
        ('み', "mi"),
        ('り', "ri"),
        //U
        ('う', "u"),
        ('く', "ku"),
        ('す', "su"),
        ('つ', "tu"),
        ('ぬ', "nu"),
        ('ふ', "hu"),
        ('む', "mu"),
        ('ゆ', "yu"),
        ('る', "ru"),
        //E
        ('え', "e"),
        ('け', "ke"),
        ('せ', "se"),
        ('て', "te"),
        ('ね', "ne"),
        ('へ', "he"),
        ('め', "me"),
        ('れ', "re"),
        //O
        ('お', "o"),
        ('こ', "ko"),
        ('そ', "so"),
        ('と', "to"),
        ('の', "no"),
        ('ほ', "ho"),
        ('も', "mo"),
        ('よ', "yo"),
        ('ろ', "ro"),
        ('を', "wo"),
    ];
    hiraganas.shuffle(&mut rng);
    // let vowel_choosed = ['a', 'e', 'i', 'u', 'o'].choose(&mut rng).unwrap();
    // let _vowel_choosed = ['a', 'i'];

    let _vowel_choosed = match config_check() {
        Ok(vowels) => vowels,
        Err(_) => vec!['a', 'e', 'i', 'u', 'o'],
    };

    let mut hiraganas: Vec<Hiragana> = hiraganas
        .map(|it| Hiragana::from(it))
        .into_iter()
        .filter(|it| _vowel_choosed.contains(&it.vowel) || it.vowel.to_string() == it.romanized)
        .collect();
    let len = rng.gen_range(0..hiraganas.len());
    hiraganas.truncate(len);

    for hiragana in hiraganas {
        let mut line = String::new();
        println!("What's that character: {} ?", hiragana.japanese);
        stdin().read_line(&mut line).unwrap();
        line = line.trim_end().to_string();
        if line.eq_ignore_ascii_case(hiragana.romanized) {
            println!("Nice");
            points += 1;
        } else {
            println!("You already forgot ? It's '{}' !", hiragana.romanized);
        }
    }
    println!("You have {}/{} points.", points, len);
}
