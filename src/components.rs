use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "bullet.png")]
    bullet: Handle<Image>,
}
