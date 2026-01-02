use bevy::prelude::*;

fn close_handler(
    mut events: MessageReader<KeyboardInput>,
    mut exit_events: MessageWriter<AppExit>,
) {
    for ev in events.read() {
        if ev.logical_key == Key::Escape {
            exit_events.write(AppExit::Success);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, close_handler)
        .run();
}
