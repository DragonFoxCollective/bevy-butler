pub mod plugin;

pub mod __internal {
    use std::any::TypeId;
    use std::sync::Arc;
    use bevy_utils::HashMap;
    use bevy_ecs::system::Resource;
    use bevy_app::App;
    use bevy_log::debug;

    pub use inventory;

    #[derive(Debug)]
    pub struct ButlerFunc(TypeId, * const ());

    unsafe impl Sync for ButlerFunc {}
    unsafe impl Send for ButlerFunc {}

    impl ButlerFunc {
        pub const fn new<T: 'static + Send + Sync>(func: fn(&T, &mut App) -> ()) -> Self {
            let func_ptr = unsafe { std::mem::transmute(func) };
            Self(TypeId::of::<T>(), func_ptr)
        }

        pub fn type_id(&self) -> TypeId {
            self.0
        }

        pub fn get_func<T: 'static + Send + Sync>(&self) -> fn(&T, &mut App) -> () {
            assert_eq!(TypeId::of::<T>(), self.0);
            unsafe { std::mem::transmute(self.1) }
        }

        pub fn try_get_func<T: 'static + Send + Sync>(&self) -> Option<fn(&T, &mut App) -> ()> {
            if self.0 == TypeId::of::<T>() {
                return Some(self.get_func());
            }
            None
        }
    }

    #[derive(Resource)]
    pub struct ButlerRegistry {
        plugin_map: HashMap<TypeId, Arc<Vec<&'static ButlerFunc>>>,
    }

    impl ButlerRegistry {
        pub fn new(plugin_map: HashMap<TypeId, Vec<&'static ButlerFunc>>) -> Self {
            Self {
                plugin_map: plugin_map.into_iter().map(|(k, v)| (k, Arc::new(v))).collect()
            }
        }

        pub fn get_funcs<T: 'static + Send + Sync>(&self) -> Option<Arc<Vec<&'static ButlerFunc>>> {
            self.plugin_map.get(&TypeId::of::<T>())
                .map(|arc| arc.clone())
        }
    }

    pub fn _butler_debug(msg: &str) {
        debug!("{}", msg);
    }

    inventory::collect!(ButlerFunc);
}