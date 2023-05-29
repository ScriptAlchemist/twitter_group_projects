use cursive::views::{Button, Dialog, DummyView, LinearLayout};
use cursive::traits::*;
use cursive::theme::{BorderStyle, Palette};

mod help;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.set_theme(cursive::theme::Theme{
        shadow: false,
        borders: BorderStyle::None,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;
            {
                use cursive::theme::Color::TerminalDefault;
                use cursive::theme::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }

            {
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Green.light()).combine(Bold);
            }

        }),
    });

    // Quit from anywhere using q
    siv.add_global_callback('q', |s| s.quit());

    let welcome_screen = LinearLayout::vertical()
        .child(Dialog::text("This is a guessing game!\nPress <Next> when you're ready.\n\nCheck <Open Directions> to learn how to play the game\n\n"));

    let start_buttons = LinearLayout::vertical()
        .child(DummyView)
        .child(DummyView)
        .child(Button::new("Next", help::show_next))
        .child(DummyView)
        .child(Button::new("Open Directions", help::show_rules))
        .child(DummyView)
        .child(Button::new("Quit", |s| s.quit()));

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(welcome_screen)
            .child(DummyView)
            .child(start_buttons))
        .title("Guessing Game"));

    siv.run();
}
