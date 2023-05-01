mod config;
mod hiraganas;

use hiraganas::get_hiraganas;
use rand::thread_rng;
use std::io::stdin;

fn main() -> Result<(), std::io::Error> {
    let mut points = 0;
    let mut rng = thread_rng();
    let hiraganas = get_hiraganas(&mut rng);

    for hiragana in &hiraganas {
        let mut line = String::new();
        println!("What's that character: {} ?", hiragana.get_hiragana());
        stdin().read_line(&mut line).unwrap();
        line = line.trim_end().to_string();
        if line.eq_ignore_ascii_case(hiragana.get_roumanji()) {
            println!("Nice");
            points += 1;
        } else {
            println!("You already forgot ? It's '{}' !", hiragana.get_roumanji());
        }
    }
    println!("You have {}/{} points.", points, hiraganas.len());
    Ok(())
}
