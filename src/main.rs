use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::theme::Effect;
use cursive::theme::Style;
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;
use std::fs;
use std::time::SystemTime;

struct Game {
    words: Vec<String>,
    display: Vec<String>,
    display_counter: usize,
    counter: usize,
    time: SystemTime,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            words: self.words.clone(),
            display: self.display.clone(),
            display_counter: self.display_counter,
            counter: self.counter,
            time: self.time,
        }
    }
}
impl Game {
    fn new() -> Self {
        Game {
            words: Vec::new(),
            display: Vec::new(),
            display_counter: 1,
            counter: 0,
            time: SystemTime::now(),
        }
    }

    fn start_typing(&mut self) {
        self.time = SystemTime::now();
    }

    fn measure_speed(&mut self) -> f32 {
        let secs = match self.time.elapsed() {
            Ok(t) => t.as_secs(),
            Err(_) => 100,
        };
        self.counter as f32 / secs as f32 * 60.0
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
        self.display = self.words.clone();
    }
}
fn main() {
    let mut game: Game = Game::new();
    game.create_word_list("/home/mim/rust-typing-speed/src/text.txt".to_string());
    game.start_typing();
    let game2 = game.clone();
    let mut siv = cursive::default();
    siv.set_user_data(game2);
    typing_view(&mut siv, game.words.clone(), game.display_counter);
    siv.run();
}

fn check(s: &mut Cursive, text: &str, _: usize) {
    if text.contains(" ") {
        let mut game = match s.user_data::<Game>() {
            Some(v) => v,
            None => panic!("what the fuck"),
        }
        .clone();
        let mut text = text.to_string();
        text.pop();
        if game.words.len() == 1 {
            show_end(s, &mut game);
        } else {
            let word = game.words.remove(0);
            if text == word {
                game.counter += text.len() + 1;
            }
            game.display_counter += 1;
            s.set_user_data(game.clone());
            typing_view(s, game.display.clone(), game.display_counter);
        }
    }
}

fn typing_view(s: &mut Cursive, mut words: Vec<String>, word_counter: usize) {
    let words2 = words.split_off(word_counter);
    let word_to_type = words.pop();
    let mut styled = StyledString::plain(words.join(" "));
    if words.len() != 0 {
        styled.append(StyledString::plain(" "));
    }
    styled.append(StyledString::styled(
        word_to_type.unwrap() + " ",
        Style::from(Color::Light(BaseColor::Red)).combine(Effect::Bold),
    ));
    styled.append(StyledString::plain(words2.join(" ")));
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new(styled).fixed_width(40))
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

fn show_end(s: &mut Cursive, game: &mut Game) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(TextView::new(game.measure_speed().to_string() + " cpm"))
            .title("End results")
            .button("Exit", |s| {
                s.quit();
            }),
    );
}
