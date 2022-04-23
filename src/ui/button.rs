use bevy::prelude::*;

pub fn create(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor::from(Color::BLUE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Push Me!",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::GRAY,
                    },
                    Default::default(),
                ),
                ..default()
            });
        });
}
