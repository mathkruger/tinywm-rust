use penrose::{
    core::{
        bindings::parse_keybindings_with_xmodmap,
        Config,
        WindowManager,
    },
    x11rb::RustConn,
    extensions::hooks::{
        add_ewmh_hooks,
        SpawnOnStartup
    },
    Result,
};

use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

mod constants;
mod keybindings;
mod layouts;
mod status_bar;

use keybindings::raw_key_bindings;
use layouts::layouts_config;
use status_bar::build_status_bar;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .finish()
        .init();
    
    let startup_hook = SpawnOnStartup::boxed("/home/mathkruger/.tinywm-startup.sh");

    let config = add_ewmh_hooks(Config {
        default_layouts: layouts_config(),
        startup_hook: Some(startup_hook),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let bar = build_status_bar();
    let wm = bar.add_to(WindowManager::new(
        config,
        key_bindings,
        HashMap::new(),
        conn,
    )?);

    wm.run()
}
