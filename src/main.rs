extern crate piston_window;

use crate::components::application::MiniApp;
use menu::{config::Config, ui::MainMenu};
use piston_window::*;

mod components;
mod menu;
mod snake;
mod tictactoe;
mod wordle;

fn main() {
    let mut window = WindowSettings::new(
        "UnoLife",
        [
            components::application::DEFAULT_WIDTH,
            components::application::DEFAULT_HEIGHT,
        ],
    )
    .resizable(false)
    .build::<PistonWindow>()
    .unwrap()
    .max_fps(60);

    let mut main_menu = MainMenu::new();
    let mut config = Config::fetch_config();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("Roboto-Regular.ttf")).unwrap();

    while let Some(event) = window.next() {
        main_menu.render(&mut window, &event, &mut glyphs, &mut config);
    }
}
