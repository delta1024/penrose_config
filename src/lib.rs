#[macro_use]
extern crate penrose;

use penrose::{
    core::helpers::index_selectors, Backward, Forward, Less, More, Result, WindowManager,
    XcbConnection, __example_helpers::KeyCode,
};
use std::collections::HashMap;

pub const FONT: &str = "FiraCode Nerd Font";

pub const HEIGHT: usize = 18;

pub const BLACK: &str = "#282828";

pub const WHITE: &str = "#ebdbb2";

pub const GREY: &str = "#3c3836";

pub const BLUE: &str = "#458588";

pub fn set_keybindings() -> HashMap<
    KeyCode,
    Box<(dyn for<'r> FnMut(&'r mut WindowManager<XcbConnection>) -> Result<()> + 'static)>,
> {
    let keys = gen_keybindings! {
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-c" => run_internal!(kill_client);
        "M-Tab" => run_internal!(toggle_workspace);
        "M-A-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
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
