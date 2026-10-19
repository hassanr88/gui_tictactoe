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
use crate::ui::grid::draw_grid;
use crate::ui::image_button::ImageButton;
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use core::num;

use macroquad::miniquad::CursorIcon::Text;
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
    let mut btn: Vec<ImageButton> = vec![
        ImageButton::new(
            100.0,              // x position
            50.0,              // y position
            150.0,              // width
            150.0,               // height
            "assets/white.png", // normal state image
            "assets/white.png", // hover state image
        )
        .await,
        ImageButton::new(
            100.0,              // x position
            50.0,              // y position
            150.0,              // width
            150.0,               // height
            "assets/white.png", // normal state image
            "assets/white.png", // hover state image
        )
        .await,
        ImageButton::new(
            100.0,              // x position
            50.0,              // y position
            150.0,              // width
            150.0,               // height
            "assets/white.png", // normal state image
            "assets/white.png", // hover state image
        )
        .await,
        ImageButton::new(
            100.0,              // x position
            200.0,              // y position
            200.0,              // width
            60.0,               // height
            "assets/white.png", // normal state image
            "assets/white.png", // hover state image
        )
        .await,
        ImageButton::new(
            100.0,              // x position
            200.0,              // y position
            200.0,              // width
            60.0,               // height
            "assets/white.png", // normal state image
            "assets/white.png", // hover state image
        )
        .await,
        ImageButton::new(
            100.0,              // x position
            200.0,              // y position
            200.0,              // width
            60.0,               // height
            "assets/white.png", // normal state image
            "assets/white.png", // hover state image
        )
        .await,
        ImageButton::new(
            100.0,              // x position
            200.0,              // y position
            200.0,              // width
            60.0,               // height
            "assets/white.png", // normal state image
            "assets/white.png", // hover state image
        )
        .await,
        ImageButton::new(
            100.0,              // x position
            200.0,              // y position
            200.0,              // width
            60.0,               // height
            "assets/white.png", // normal state image
            "assets/white.png", // hover state image
        )
        .await,
        ImageButton::new(
            100.0,              // x position
            200.0,              // y position
            200.0,              // width
            60.0,               // height
            "assets/white.png", // normal state image
            "assets/white.png", // hover state image
        )
        .await,
    ];

    let mut btn_reset = TextButton::new(250.0, 700.0, 150.0, 50.0, "Reset", GRAY, RED, 30);
    let mut btn_exit = TextButton::new(650.0, 700.0, 150.0, 50.0, "Exit", GRAY, RED, 30);
    let lbl_score = Label::new("Score: 0 - 0", 445.0, 700.0, 30);
    btn_reset.with_hover_text_color(BLACK);
    btn_exit.with_hover_text_color(BLACK);
    loop {
        clear_background(WHITE);
        draw_grid(50.0, BROWN);
        for i in 0..btn.len() {
            if btn[i].click() {

            }
        }
        if btn_reset.click() {}
        if btn_exit.click() {
            break;
        }
        lbl_score.draw();
        next_frame().await;
    }
}
