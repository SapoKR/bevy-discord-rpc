use bevy::prelude::*;
use bevy_async_system::prelude::*;
use bevy_discord_presence::{ActivityState, RPCConfig, RPCPlugin};

fn main() {
    println!("hello world!");
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        AsyncSystemPlugin,
        RPCPlugin {
            config: RPCConfig {
                app_id: 425407036495495169,
                show_time: true,
            },
        }
    ));
    app.add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_async(|schedules| async move {
        schedules.add_system(Update, repeat::forever(update_presence)).await;
    });
}

fn update_presence(mut state: ResMut<ActivityState>) {
    state.instance = Some(true);
    state.details = Some("Hello World".to_string());
    state.state = Some("This is state".to_string());
}
