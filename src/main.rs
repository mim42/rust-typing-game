use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;
use std::fs;
use std::ops::RangeFrom;
use std::time::SystemTime;

struct Game {
    words: Vec<String>,
    counter: usize,
    time: SystemTime,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            words: self.words.clone(),
            counter: self.counter,
            time: SystemTime::now(),
        }
    }
}
impl Game {
    fn new() -> Self {
        Game {
            words: Vec::new(),
            counter: 0,
            time: SystemTime::now(),
        }
    }

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
        self.words = self.words.clone();
    }
}
fn main() {
    let mut game: Game = Game::new();
    game.create_word_list("/home/mim/rust-typing-speed/src/text.txt".to_string());
    game.start_typing();
    let game2 = game.clone();
    let mut siv = cursive::default();
    siv.set_user_data(game2);
    add_name(&mut siv, game.words.clone());
    siv.run();
}

fn check(s: &mut Cursive, text: &str, _: usize) {
    if text.contains(" ") {
        print!("{}", text);
        let game = match s.user_data::<Game>() {
            Some(v) => v,
            None => panic!(),
        };
        let mut text = text.to_string();
        text.pop();
        let word = match game.words.pop() {
            Some(v) => v,
            None => panic!(game.measure_speed().to_string()),
        };
        if text == word {
            game.counter += 1;
            println!("{}", text)
        }
        add_name(s, game.words.clone());
    }
}

fn add_name(s: &mut Cursive, words: Vec<String>) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new(words.join(" ")))
                .child(
                    EditView::new()
                        .on_edit(check)
                        .with_name("type")
                        .fixed_width(40),
                ),
        )
        .title("Type as fast as possible")
        .button("Cancel", |s| {
            s.quit();
        }),
    );
}
