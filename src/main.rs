#[macro_use]
extern crate penrose;

use penrose::{
    logging_error_handler,
    xcb::{new_xcb_backed_window_manager, },
};
use penrose_conf::{set_keybindings, gen_status_bar, gen_config, spawn_rule};

fn main() -> penrose::Result<()> {
    let key_bindings = set_keybindings();
    let config = gen_config();
    let status_bar = gen_status_bar(&config)?;
    let spawn_rules = spawn_rule();
    let mut wm =
        new_xcb_backed_window_manager(config, vec![Box::new(status_bar),spawn_rules], logging_error_handler())?;

    wm.grab_keys_and_run(key_bindings, map! {})
}
