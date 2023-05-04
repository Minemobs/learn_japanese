use crate::config::config_check;
use rand::{rngs::ThreadRng, seq::SliceRandom};

pub struct Hiragana {
    japanese: char,
    romanized: &'static str,
    vowel: char,
}

impl Hiragana {
    pub fn new(japanese: char, romanized: &'static str) -> Self {
        Hiragana {
            japanese,
            romanized,
            vowel: match romanized {
                "n" => 'a',
                _ => romanized.chars().last().unwrap(),
            },
        }
    }

    pub fn from(tuple: (char, &'static str)) -> Self {
        Hiragana::new(tuple.0, tuple.1)
    }

    pub fn get_vowel(&self) -> char {
        self.vowel
    }

    pub fn get_hiragana(&self) -> char {
        self.japanese
    }

    pub fn get_roumanji(&self) -> &'static str {
        self.romanized
    }
}

pub fn get_hiraganas(rng: &mut ThreadRng) -> Vec<Hiragana> {
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
    hiraganas.shuffle(rng);
    let vowel_choosed = match config_check() {
        Ok(vowels) => vowels,
        Err(_) => vec!['a', 'e', 'i', 'u', 'o'],
    };

    let hiraganas: Vec<Hiragana> = hiraganas
        .map(Hiragana::from)
        .into_iter()
        .filter(|it| {
            vowel_choosed.contains(&it.get_vowel())
                || it.get_vowel().to_string() == it.get_roumanji()
        })
        .collect();
    // let len = rng.gen_range(4..hiraganas.len());
    // hiraganas.truncate(len);
    hiraganas
}
