use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_turret_placement_hover);
}

pub fn on_turret_placement_hover(trigger: Trigger<Pointer<Click>>) {
    println!("{} was hovered!", trigger.target);
}
