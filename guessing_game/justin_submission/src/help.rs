use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView, TextView, LinearLayout};
use cursive::traits::*;


use rand::rngs::OsRng;
use rand::Rng;

enum GuessHeat {
    Higher,
    Lower,
    NoGuess,
}

pub fn generate_number(num: u8) -> u8 {
    let mut rng = OsRng;
    rng.gen_range(1..=num)
}

pub fn you_win(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("")
        .title("You win!")
        .button("Finish", |s| s.quit()));

}

pub fn you_lose(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("")
        .title("Sucker, you lose!")
        .button("Finish", |s| s.quit()));

}

pub fn show_rules(s: &mut cursive::Cursive) {

    s.add_layer(Dialog::info("The rules of this game are simple.\n\nThis is a guessing game.\n\nYou will have to select a difficulty (East, Medium, or Hard).\n\nEasy is a choice between 1-20\n\nMedium is a choice between 1-50\n\nHard is a choice between 1-100 with only 5 guesses possible").title("Game Directions"));
}

pub fn show_next(s: &mut Cursive) {
    s.pop_layer();
    let words = Dialog::text("Select a difficulty (East, Medium, or Hard).\n\nEasy is a choice between 1-20\n\nMedium is a choice between 1-50\n\nHard is a choice between 1-100 with only 5 guesses possible");

    let buttons = LinearLayout::vertical()
        .child(DummyView)
        .child(DummyView)
        .child(Button::new("Easy", |s| {
            try_easy_guess(s, None, GuessHeat::NoGuess);
        }))
        .child(DummyView)
        .child(Button::new("Medium", |s| {
            try_medium_guess(s, None, GuessHeat::NoGuess);
        }))
        .child(DummyView)
        .child(Button::new("Hard", |s| {
            try_hard_guess(s, None,GuessHeat::NoGuess, 5);
        }));

    s.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(words)
            .child(DummyView)
            .child(buttons))
    .title("Pick a level"));
}

fn try_easy_guess(s: &mut Cursive, number: Option<u8>, retry: GuessHeat) {
    let random_number: u8 = match number {
        Some(num) => num,
        None => generate_number(20),
    };

    let text = match retry {
        GuessHeat::NoGuess => "guess a number between 1 and 20\n\n",
        GuessHeat::Higher => "Incorrect guess is to high.\n\nTry again, guess a lower number between 1 and 20\n\n",
        GuessHeat::Lower => "Incorrect guess is to low.\n\nTry again, guess a higher number between 1 and 20\n\nMake sure you're guessing a number\n\n",
    };

    let layout = LinearLayout::vertical()
        .child(TextView::new(text))
        .child(
                EditView::new()
                    .on_submit(move |s, guess| {
                        let guess: u8 = guess.trim().parse().unwrap_or(0);
                        if guess == random_number {
                            you_win(s);
                        } else if guess > random_number {
                            try_easy_guess(s, Some(random_number), GuessHeat::Higher);
                        } else if guess < random_number {
                            try_easy_guess(s, Some(random_number), GuessHeat::Lower);
                        }
                    })
                    .with_name("Guess")
                    .fixed_width(10));

    s.pop_layer();
    s.add_layer(
        Dialog::around(layout)
            .title("Playing Easy Mode")
            .button("Guess", move |s| {
                let guess = s.call_on_name("Guess", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
                let guess: u8 = guess.trim().parse().unwrap_or(0);
                if guess == random_number {
                    you_win(s);
                } else if guess > random_number {
                    try_easy_guess(s, Some(random_number), GuessHeat::Higher);
                } else if guess < random_number {
                    try_easy_guess(s, Some(random_number), GuessHeat::Lower);
                }
            })
            .button("Quit", |s| {
                s.quit();
            }),
    );
}

fn try_medium_guess(s: &mut Cursive, number: Option<u8>, retry: GuessHeat) {
    let random_number: u8 = match number {
        Some(num) => num,
        None => generate_number(50),
    };

    let text = match retry {
        GuessHeat::NoGuess => "guess a number between 1 and 50\n\n",
        GuessHeat::Higher => "Incorrect guess is to high.\n\nTry again, guess a lower number between 1 and 50\n\n",
        GuessHeat::Lower => "Incorrect guess is to low.\n\nTry again, guess a higher number between 1 and 50\n\nMake sure you're guessing a number\n\n",
    };

    let layout = LinearLayout::vertical()
        .child(TextView::new(text))
        .child(
                EditView::new()
                    .on_submit(move |s, guess| {
                        let guess: u8 = guess.trim().parse().unwrap_or(0);
                        if guess == random_number {
                            you_win(s);
                        } else if guess > random_number {
                            try_medium_guess(s, Some(random_number), GuessHeat::Higher);
                        } else if guess < random_number {
                            try_medium_guess(s, Some(random_number), GuessHeat::Lower);
                        }
                    })
                    .with_name("Guess")
                    .fixed_width(10));

    s.pop_layer();
    s.add_layer(
        Dialog::around(layout)
            .title("Playing Medium Mode")
            .button("Guess", move |s| {
                let guess = s.call_on_name("Guess", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
                let guess: u8 = guess.trim().parse().unwrap_or(0);
                if guess == random_number {
                    you_win(s);
                } else if guess > random_number {
                    try_medium_guess(s, Some(random_number), GuessHeat::Higher);
                } else if guess < random_number {
                    try_medium_guess(s, Some(random_number), GuessHeat::Lower);
                }
            })
            .button("Quit", |s| {
                s.quit();
            }),
    );
}

fn try_hard_guess(s: &mut Cursive, number: Option<u8>, retry: GuessHeat, tries_left: u8) {
    let random_number: u8 = match number {
        Some(num) => num,
        None => generate_number(100),
    };

    let text = match retry {
        GuessHeat::NoGuess => format!("Guess a number between 1 and 100\n\nTries left: {}", tries_left),
        GuessHeat::Higher => format!("Incorrect guess is to high.\n\nTry again, guess a lower number between 1 and 100\n\nTries left: {}", tries_left),
        GuessHeat::Lower => format!("Incorrect guess is to low.\n\nTry again, guess a higher number between 1 and 100\n\nMake sure you're guessing a number\n\nTries left: {}", tries_left),
    };

    let layout = LinearLayout::vertical()
        .child(TextView::new(text))
        .child(
                EditView::new()
                    .on_submit(move |s, guess| {
                        let guess: u8 = guess.trim().parse().unwrap_or(0);
                        if guess != random_number && tries_left - 1 == 0 {
                            you_lose(s);
                        } else if guess == random_number {
                            you_win(s);
                        } else if guess > random_number {
                            try_hard_guess(s, Some(random_number), GuessHeat::Higher, tries_left - 1);
                        } else if guess < random_number {
                            try_hard_guess(s, Some(random_number), GuessHeat::Lower, tries_left - 1);
                        }
                    })
                    .with_name("Guess")
                    .fixed_width(10));

    s.pop_layer();
    s.add_layer(
        Dialog::around(layout)
            .title("Playing Hard Mode")
            .button("Guess", move |s| {
                let guess = s.call_on_name("Guess", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
                let guess: u8 = guess.trim().parse().unwrap_or(0);
                if guess == random_number {
                    you_win(s);
                } else if guess > random_number {
                    try_hard_guess(s, Some(random_number), GuessHeat::Higher, tries_left - 1);
                } else if guess < random_number {
                    try_hard_guess(s, Some(random_number), GuessHeat::Lower, tries_left - 1);
                }
            })
            .button("Quit", |s| {
                s.quit();
            }),
    );
}
