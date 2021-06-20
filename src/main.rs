#[macro_use]
extern crate penrose;

use penrose::{
    draw::{dwm_bar, Color, TextStyle},
    logging_error_handler,
    xcb::{new_xcb_backed_window_manager, XcbDraw},
    Config,
};
use penrose_conf::{set_keybindings, BLACK, BLUE, FONT, GREY, HEIGHT, WHITE};
use std::convert::TryFrom;

fn main() -> penrose::Result<()> {
    let key_bindings = set_keybindings();
    let config = Config::default()
        .builder()
        .workspaces(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"])
        .build()
        .unwrap();
    let workspaces = config.workspaces().clone();
    let mut wm = new_xcb_backed_window_manager(
        config,
        vec![Box::new(dwm_bar(
            XcbDraw::new()?,
            HEIGHT,
            &TextStyle {
                font: FONT.to_string(),
                point_size: 10,
                fg: Color::try_from(WHITE)?,
                bg: Some(Color::try_from(BLACK)?),
                padding: (2.0, 2.0),
            },
            Color::try_from(BLUE)?, // highlight
            Color::try_from(GREY)?, // empty_ws
            workspaces,
        )?)],
        logging_error_handler(),
    )?;

    wm.grab_keys_and_run(key_bindings, map! {})
}
