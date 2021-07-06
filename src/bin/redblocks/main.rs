#[macro_use]
extern crate redblocks;

use redblocks::{
    plugins::{CpuPlugin, MemPlugin, TimePlugin},
    Widget,
};

fn main() {
    let widgets = vec![
        Widget::new(MemPlugin::new(), 2),
        Widget::new_mili(CpuPlugin::new(), 750),
        Widget::new(Box::new(TimePlugin::default()), 1),
    ];

    start_bar!(widgets);
}
