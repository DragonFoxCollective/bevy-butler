use bevy_butler::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use super::common::*;

#[derive(Resource, Default)]
struct Counter(pub u8);

#[butler_plugin(build = init_resource::<Counter>)]
struct MyPlugin;

config_systems! {
    (plugin = MyPlugin, schedule = Startup)

    system_set! {
        (chain)

        #[system]
        fn system_one(mut counter: ResMut<Counter>) {
            info!("System one!");
            assert_eq!(counter.0, 0);
            counter.0 = 1;
        }

        #[system]
        fn system_two(mut counter: ResMut<Counter>) {
            info!("System two");
            assert_eq!(counter.0, 1);
            counter.0 = 2;
        }

        #[system]
        fn system_three(mut counter: ResMut<Counter>) {
            info!("System three");
            assert_eq!(counter.0, 2);
            counter.0 = 3;
        }
    }
}

#[test]
fn test() {
    App::new()
        .add_plugins(log_plugin())
        .add_plugins(MyPlugin)
        .add_systems(PostStartup, |counter: Res<Counter>| assert_eq!(counter.0, 3))
        .run();
}