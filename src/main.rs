use cursive::traits::*;
use cursive::views::{Dialog, EditView};
use cursive::Cursive;
use std::fs;
use std::ops::RangeFrom;
use std::time::SystemTime;

struct Game {
    words: Vec<String>,
    counter: usize,
    time: SystemTime,
}
impl Game {
    fn start_typing(&mut self) {
        self.time = SystemTime::now();
    }

    fn measure_speed(&mut self) -> f32 {
        let u: Vec<String> = self
            .words
            .drain(RangeFrom {
                start: self.counter,
            })
            .collect();
        let char_counter: usize = u.iter().fold(0, |acc, word| acc + word.len());
        let secs = match self.time.elapsed() {
            Ok(t) => t.as_secs(),
            Err(_) => 100,
        };
        char_counter as f32 / secs as f32
    }

    fn create_word_list(&mut self, filename: String) {
        let contents =
            fs::read_to_string(filename).expect("something went terribly wrong reading the file");
        self.words = contents
            .split("\n")
            .into_iter()
            .map(|x| x.to_string())
            .collect();
    }
}
fn main() {
    let mut list_of_words: Vec<String> = Vec::new();
    list_of_words.push("hi".to_string());
    list_of_words.push("two".to_string());
    let mut siv = cursive::default();

    siv.set_user_data(list_of_words);
    add_name(&mut siv);
    siv.run();
}

fn check(s: &mut Cursive, text: &str, size: usize) {
    if text.contains(" ") {
        print!("{}", text);
        let list = match s.user_data::<Vec<String>>() {
            Some(v) => v,
            None => panic!(),
        };
        let mut text = text.to_string();
        text.pop();
        let word = match list.pop() {
            Some(v) => v,
            None => "".to_string(),
        };
        if text == word {
            println!("{}", text)
        }
        s.pop_layer();
        add_name(s);
    }
}

fn add_name(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_edit(check)
                .with_name("type")
                .fixed_width(40),
        )
        .title("Type as fast as possible")
        .button("Cancel", |s| {
            s.quit();
        }),
    );
}
