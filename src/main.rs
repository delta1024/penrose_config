#[macro_use]
extern crate penrose;

use penrose::{
    core::helpers::index_selectors, logging_error_handler, xcb::new_xcb_backed_window_manager,
    Backward, Config, Forward, Less, More, WindowManager,
};

fn main() -> penrose::Result<()> {
    let key_bindings = gen_keybindings! {
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-q" => run_internal!(kill_client);
        "M-Tab" => run_internal!(toggle_workspace);
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);
        "M-A-Return" => run_external!("dmenu_run -c -l 10");
        "M-Return" => run_external!("alacritty");
        "M-A-Escape" => run_internal!(exit);

        refmap [ 1..10 ] in {
            "M-{}" => focus_workspace [ index_selectors(9) ];
            "M-S-{}" => client_to_workspace [ index_selectors(9) ];
        };
    };

    let mut wm = new_xcb_backed_window_manager(Config::default(), vec![], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map! {})
}
