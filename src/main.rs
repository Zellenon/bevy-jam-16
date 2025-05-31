// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]


use bevy::prelude::*;
use chain_reaction_towers::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
