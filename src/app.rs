use crate::hiraganas::Hiragana;

pub struct App<'a> {
    hiraganas: &'a mut Vec<Hiragana>,
    input: String,
    index: usize,
}

impl App<'_> {
    pub fn new(hiraganas: &mut Vec<Hiragana>) -> App {
        App {
            hiraganas,
            input: String::new(),
            index: 0,
        }
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn get_input_mut(&mut self) -> &mut String {
        &mut self.input
    }

    pub fn current_hiragana(&self) -> Option<&Hiragana> {
        self.hiraganas.get(self.index)
    }

    pub fn next_hiragana(&mut self) -> Option<&Hiragana> {
        self.index += 1;
        self.current_hiragana()
    }
}
