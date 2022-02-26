#![allow(clippy::needless_range_loop)]
use eframe::egui;
use crate::app::*;
use egui::{
    Color32,RichText, TextureId,style::Margin, Vec2
};

pub fn frame(ctx: &egui::Context, turns: &mut u8, winner: &mut Square, spacing: &f32, buttons: &mut [[Square; 3]; 3]){
    let _game = egui::CentralPanel::default()
            .frame(egui::Frame {
                margin: Margin{bottom: 15f32, left: 15f32, right: 15f32, top: 15f32},
                fill: Color32::WHITE,
                ..Default::default()
            })
            .show(ctx, |ui| {
                //the message to display to the user
                let message: &str;
                if *turns == 9 {
                    message = "Draw!";
                } else if *winner != Square::Blank {
                    message = "You won";
                } else if *winner == Square::Y {
                    message = "Opponent won";
                } else if *turns % 2 == 0 {
                    message = "Your turn";
                } else {
                    message = "Opponent's turn";
                };
                let mut info = egui::Grid::new("bruh");
                info = info.spacing(egui::Vec2 {
                    x: *spacing,
                    y: *spacing,
                });
                info = info.min_col_width((ui.available_width()) / 2f32);
                info = info.max_col_width((ui.available_height()) / 2f32);
                info = info.min_row_height(20f32);
                info.show(ui, |ui| {
                    ui.heading(message);
                    if *winner != Square::Blank || *turns == 9 {
                        let rematch = egui::Button::image_and_text(
                            TextureId::User(0),
                            Vec2{x: ui.available_width(), y: 20f32},
                            RichText::new("Rematch").heading(),
                        );
                        if ui.add(rematch).clicked() {
                            *buttons = [[Square::Blank; 3]; 3];
                            *winner = Square::Blank;
                            *turns = 0;
                        }
                    }
                });

                let mut grid = egui::Grid::new("some_unique_id");
                //the space between the grid buttons
                grid = grid.spacing(egui::Vec2 {
                    x: *spacing,
                    y: *spacing,
                });
                //makes the grid the size of the available screen
                grid = grid.min_col_width((ui.available_width()) / 3f32);
                //min can only make `grid` bigger so we need max as well
                grid = grid.max_col_width((ui.available_height()) / 3f32);
                //the size of the buttons
                let size = Vec2{
                    x: (ui.available_width()) / 3f32 - spacing * 3f32,
                    y: (ui.available_height()) / 3f32 - spacing,
                };
                //?height has no min function and works for some reason
                grid = grid.min_row_height((ui.available_height()) / 3f32);
                grid.show(ui, |ui| {
                    for y in 0..3 {
                        for x in 0..3 {
                            //the button text
                            let text: RichText;
                            //wether or not the button should be enabled
                            let mut enabled: bool;
                            match buttons[y][x] {
                                Square::Blank => {
                                    text = RichText::new(" ");
                                    enabled = true
                                }
                                Square::X => {
                                    text = RichText::new("X").color(Color32::WHITE);
                                    enabled = false
                                }
                                Square::Y => {
                                    text = RichText::new("Y").color(Color32::WHITE);
                                    enabled = false
                                }
                                _ => panic!("Invalid button state"),
                            };
                            if *winner != Square::Blank || *turns == 9 {
                                enabled = false;
                            }
                            //we use `image_and_text` so that we can specify the button size
                            let mut frame =
                                egui::Button::image_and_text(TextureId::User(0), size, text);
                            match buttons[y][x] {
                                Square::Blank => {}
                                Square::X => frame = frame.fill(Color32::from_rgb(100, 200, 0)),
                                Square::Y => frame = frame.fill(Color32::from_rgb(255, 166, 0)),
                            };
                            if ui.add_enabled(enabled, frame).clicked() {
                                buttons[y][x] = to_square(*turns % 2 + 1).unwrap();
                                *turns += 1;
                                *winner = win_check(&buttons);
                            }
                        }
                        ui.end_row();
                    }
                });
            });
}