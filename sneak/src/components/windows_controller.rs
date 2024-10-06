use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
struct CloseButton;

// #[derive(Component)]
// struct MinimizeButton;

pub struct WindowsControllerButtonPlugin;
impl Plugin for WindowsControllerButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, window_constoller_buttons);
        app.add_systems(Update, handle_on_press_buttons);
    }
}

fn window_constoller_buttons(mut cmd: Commands) {
    let mut parent = cmd.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        ..default()
    });
    // parent.with_children(|parent| {
    //     parent
    //         .spawn(ButtonBundle {
    //             style: Style {
    //                 width: Val::Px(18.0),
    //                 height: Val::Px(18.0),
    //                 border: UiRect::all(Val::Px(2.0)),
    //                 align_items: AlignItems::Center,
    //                 margin: UiRect::right(Val::Px(5.0)),
    //                 ..default()
    //             },

    //             background_color: BackgroundColor(Color::srgb_u8(250, 180, 55)),
    //             border_radius: BorderRadius::MAX,
    //             ..default()
    //         })
    //         .insert(Name::new("MinimizeButton"))
    //         .insert(MinimizeButton);
    // });
    parent.with_children(|parent| {
        parent
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(18.0),
                    height: Val::Px(18.0),
                    border: UiRect::all(Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb_u8(199, 37, 62)),
                border_radius: BorderRadius::MAX,

                ..default()
            })
            .insert(Name::new("CloseButton"))
            .insert(CloseButton);
    });
}

fn handle_on_press_buttons(
    mut close_query: Query<(&Interaction, &Name), (Changed<Interaction>, With<CloseButton>)>,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok((interaction, _name)) = close_query.get_single_mut() {
        if let Interaction::Pressed = *interaction {
            exit.send(AppExit::Success);
        }
    }
}
