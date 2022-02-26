#![allow(clippy::needless_range_loop)]
use eframe::{egui, epi};
use std::cmp::PartialEq;
use std::marker::Copy;
use egui::{
    Color32,RichText, TextureId,
};
use crate::game_screen;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    buttons: [[Square; 3]; 3],
    //the turn number
    turns: u8,
    //the winner
    winner: Square,
    //the spacing between the buttons
    spacing: f32,
}
#[derive(PartialEq, Clone, Copy)]
pub enum Square {
    X,
    Y,
    Blank,
}

pub fn to_square(number: u8) -> Result<Square, String> {
    match number {
        0 => Ok(Square::Blank),
        1 => Ok(Square::X),
        2 => Ok(Square::Y),
        _ => Err("not a valid squre state".to_string()),
    }
}
//?I don't know what this warning means but it's annoying
#[allow(clippy::derivable_impls)]
impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            buttons: [[Square::Blank; 3]; 3],
            turns: 0,
            spacing: 5f32,
            winner: Square::Blank,
        }
    }
}
pub fn win_check(buttons: &[[Square; 3]; 3]) -> Square {
    //*checks horizontal win
    for y in 0..3 {
        if buttons[y] == [Square::X; 3] {
            return Square::X;
        } else if buttons[y] == [Square::Y; 3] {
            return Square::Y;
        }
    }
    //*checks vertical win
    for x in 0..3 {
        let mut x_matches = 0;
        let mut y_matches = 0;
        for y in 0..3 {
            if buttons[y][x] == Square::X {
                x_matches += 1;
            }
            if buttons[y][x] == Square::Y {
                y_matches += 1;
            }
        }
        if x_matches == 3 {
            return Square::X;
        }
        if y_matches == 3 {
            return Square::Y;
        }
    }
    //*checks diagonal win
    if buttons[0][0] == Square::X && buttons[1][1] == Square::X && buttons[2][2] == Square::X {
        return Square::X;
    }
    if buttons[0][0] == Square::Y && buttons[1][1] == Square::Y && buttons[2][2] == Square::Y {
        return Square::Y;
    }
    if buttons[0][2] == Square::X && buttons[1][1] == Square::X && buttons[2][0] == Square::X {
        return Square::X;
    }
    if buttons[0][2] == Square::Y && buttons[1][1] == Square::Y && buttons[2][0] == Square::Y {
        return Square::Y;
    }
    //?the warning here is just wrong and I can't seem to be able to disable it
    return Square::Blank;
}
impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Tic Tac Toe"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        game_screen::frame(ctx, &mut self.turns, &mut self.winner, &self.spacing,&mut self.buttons );
    }
}
