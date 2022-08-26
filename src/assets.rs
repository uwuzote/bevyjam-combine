use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AssetCollection,
)]
pub struct Assets {
    #[asset(
        paths(
            "demons/00d-koci4.png",
            "demons/00s-koci4.png",
            "demons/01d-exban1.png",
            "demons/01s-exban1.png",
        ),
        collection(typed)
    )]
    pub demons: Vec<Handle<Image>>,

    #[asset(
        paths("items/00-hellfire.png", "items/01-infernoflames.png",),
        collection(typed)
    )]
    pub items: Vec<Handle<Image>>,

    #[asset(
        paths("tiles/00-infernoflames.png", "tiles/01-chest.png",),
        collection(typed)
    )]
    pub tiles: Vec<Handle<Image>>,

    #[asset(path = "font-game-commands.ttf")]
    pub font: Handle<Font>,
}
