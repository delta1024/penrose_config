#[macro_use]
extern crate penrose;

use penrose::{
    core::helpers::index_selectors,
    Backward, Forward, Less, More, Result, WindowManager, XcbConnection,
    __example_helpers::KeyCode,
    draw::{Color, StatusBar, TextStyle},
    xcb::{new_xcb_backed_status_bar, XcbDraw, XcbDrawContext},
    Config,
};
use std::collections::HashMap;
use std::convert::TryFrom;

pub const FONT: &str = "FiraCode Nerd Font";

pub const HEIGHT: usize = 18;

pub const BLACK: &str = "#282828";

pub const WHITE: &str = "#ebdbb2";

pub const GREY: &str = "#3c3836";

pub const GREEN: &str = "#20821d";
pub fn gen_status_bar(
    config: &Config,
) -> penrose::draw::Result<StatusBar<XcbDrawContext, XcbDraw, XcbConnection>> {
    new_xcb_backed_status_bar(
        HEIGHT,
        &TextStyle {
            font: FONT.to_string(),
            point_size: 10,
            fg: Color::try_from(WHITE)?,
            bg: Some(Color::try_from(BLACK)?),
            padding: (2.0, 2.0),
        },
        Color::try_from(GREY)?, // highlight
        Color::try_from(GREEN)?,  // empty_ws
        config.workspaces().clone(),
    )
}

pub fn set_keybindings() -> HashMap<
    KeyCode,
    Box<(dyn for<'r> FnMut(&'r mut WindowManager<XcbConnection>) -> Result<()> + 'static)>,
> {
    let keys = gen_keybindings! {
        "M-h" => run_internal!(cycle_client, Forward);
        "M-j" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-c" => run_internal!(kill_client);
        "M-S-grave" => run_internal!(toggle_workspace);
        "M-Tab" => run_internal!(cycle_layout, Forward);
        "M-S-Tab" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);
        "M-A-Return" => run_external!("dmenu_run -c -l 10");
        "M-S-q" => run_internal!(exit);

        refmap [ 1..10 ] in {
            "M-{}" => focus_workspace [ index_selectors(9) ];
            "M-S-{}" => client_to_workspace [ index_selectors(9) ];
        };
    };
    keys
}

pub fn gen_config() -> Config {
    Config::default()
        .builder()
        .workspaces(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"])
	.border_px(3)
	.focused_border(0x20821d)
        .build()
        .unwrap()
}
