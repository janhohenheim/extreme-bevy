use bevy::{log, prelude::*};

pub struct NativePlugin;
impl Plugin for NativePlugin {
    fn build(&self, _app: &mut AppBuilder) {
        log::info!("Using native networking plugin");
    }
}