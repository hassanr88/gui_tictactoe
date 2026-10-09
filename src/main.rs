/*
By: Hassan Rana
Date: 2026-10-07
Program Details: a fully functional single-player Tic-Tac-Toe game where the user competes against the computer. The program meets the following requirements:

Clearly display who wins each game.

Prevent the player from changing an already selected spot (X or O) on the board.

Implement computer logic that attempts to win or block the player from winning.

Use a single, consistent method to handle moves for all spots on the board (no separate methods per position).

Keep track of and display the running score of wins for both the player and the computer.
*/

mod ui;
mod utils;
//use crate::ui::grid::draw_grid;
use crate::ui::image_button::ImageButton;
use crate::ui::text_button::TextButton;
use crate::ui::label::Label;

use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_string(),
        window_width: 1050,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut btn_reset = TextButton::new(250.0, 700.0, 150.0, 50.0, "Reset", GRAY, RED, 30);
    let mut btn_exit = TextButton::new(650.0, 700.0, 150.0, 50.0, "Exit", GRAY, RED, 30);
    let lbl_score = Label::new("Score: 0 - 0", 445.0, 700.0, 30);
    btn_reset.with_hover_text_color(BLACK);
    btn_exit.with_hover_text_color(BLACK);
    let mut btn_img_1 = ImageButton::new(
        250.0,                     // x position
        50.0,                     // y position
        150.0,                     // width
        150.0,                      // height
        "assets/white.png",       // normal state image
        "assets/white.png", // hover state image
    )
    .await;
    let mut btn_img_2 = ImageButton::new(
        450.0,                     // x position
        50.0,                     // y position
        150.0,                     // width
        150.0,                      // height
        "assets/white.png",       // normal state image
        "assets/white.png", // hover state image
    )
    .await;
    let mut btn_img_3 = ImageButton::new(
        650.0,                     // x position
        50.0,                     // y position
        150.0,                     // width
        150.0,                      // height
        "assets/white.png",       // normal state image
        "assets/white.png", // hover state image
    )
    .await;
    let mut btn_img_4 = ImageButton::new(
        250.0,                     // x position
        250.0,                     // y position
        150.0,                     // width
        150.0,                      // height
        "assets/white.png",       // normal state image
        "assets/white.png", // hover state image
    )
    .await;
    let mut btn_img_5 = ImageButton::new(
        450.0,                     // x position
        250.0,                     // y position
        150.0,                     // width
        150.0,                      // height
        "assets/white.png",       // normal state image
        "assets/white.png", // hover state image
    )
    .await;
    let mut btn_img_6 = ImageButton::new(
        650.0,                     // x position
        250.0,                     // y position
        150.0,                     // width
        150.0,                      // height
        "assets/white.png",       // normal state image
        "assets/white.png", // hover state image
    )
    .await;
    let mut btn_img_7 = ImageButton::new(
        250.0,                     // x position
        450.0,                     // y position
        150.0,                     // width
        150.0,                      // height
        "assets/white.png",       // normal state image
        "assets/white.png", // hover state image
    )
    .await;
    let mut btn_img_8 = ImageButton::new(
        450.0,                     // x position
        450.0,                     // y position
        150.0,                     // width
        150.0,                      // height
        "assets/white.png",       // normal state image
        "assets/white.png", // hover state image
    )
    .await;
    let mut btn_img_9 = ImageButton::new(
        650.0,                     // x position
        450.0,                     // y position
        150.0,                     // width
        150.0,                      // height
        "assets/white.png",       // normal state image
        "assets/white.png", // hover state image
    )
    .await;
    loop {
        clear_background(WHITE);
        //draw_grid(50.0, BROWN);
        if btn_img_1.click() {

        }
        if btn_img_2.click() {
            
        }
        if btn_img_3.click() {
            
        }
        if btn_img_4.click() {
            
        }
        if btn_img_5.click() {
            
        }
        if btn_img_6.click() {
            
        }
        if btn_img_7.click() {
            
        }
        if btn_img_8.click() {
            
        }
        if btn_img_9.click() {
            
        }
        if btn_reset.click() {

        }
        if btn_exit.click() {
            break;
        }
        lbl_score.draw();
        next_frame().await;
    }
}
