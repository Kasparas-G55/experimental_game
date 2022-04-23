use bevy::prelude::*;

pub mod button;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    button::create(commands, asset_server);
}
