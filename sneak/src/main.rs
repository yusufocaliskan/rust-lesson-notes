use bevy::prelude::*;

mod components;

fn main() {
    //✅ 1. Draw a window
    //✅ 2. Set a title to the window
    //✅ 3. Set height and width to the window

    //4. Draw a sneak head.
    //5. Move the sneak by controllers
    //6. The Senak should not go throug the walls.
    //7. Create a food
    //8. Let the sneak to eat the food
    //9. Append the sneak tail after each food
    //10. Increase a the score of the user for each food
    //11. Think about conditions of fail

    //TODO: FEATURES
    //1. Add a Close button to end the game.

    //--- Start ----
    //1. Draw a window
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from(" 🐍 Snake Game / Maar"),
                decorations: false,

                ..default()
            }),

            ..default()
        }))
        .add_plugins(components::windows_controller::WindowsControllerButtonPlugin)
        .add_plugins(components::the_snake::Snake)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut cmd: Commands) {
    //add a new camera
    cmd.spawn(Camera2dBundle::default());
    // window_constoller_buttons(&mut cmd);
}

fn update(mut cmd: Commands) {
    // handle_on_press_wind_controller_buttons(&mut cmd);
}
