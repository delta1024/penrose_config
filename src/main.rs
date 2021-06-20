#[macro_use]
extern crate penrose;

use penrose::{
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,Config, 
};
use penrose_conf::{
    set_keybindings,
};

fn main() -> penrose::Result<()> {
    let key_bindings = set_keybindings();

    let mut wm = new_xcb_backed_window_manager(
        Config::default(),
        vec![],
        logging_error_handler(),
    )?;

    wm.grab_keys_and_run(key_bindings, map!{})
}
