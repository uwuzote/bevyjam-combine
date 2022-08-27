use bevy::prelude::*;

/// Playable chars
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Demon;

/// Active char (single)
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct ActiveDemon;

/// Core root elem for animating demons (single)
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Animator;

/// Elem for highlighting active demon (single)
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Highlighter;

/// Handle of shade of demon
#[derive(
    Debug, Clone, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct HighlighterTexture(pub Handle<Image>);

/// Root of menu node (single)
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct UiMenuNode;

/// Storage section in UI (single)
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct UiStorageSection;

/// Demon's item holder node in UI (single)
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct UiDemonInvNode;

/// Storage's item holder node in UI (single)
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct UiStorageInvNode;

/// Root of hotbar node (single)
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct UiHotbarNode;

/// Item holder in hotbar node (single)
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct UiHotbarInvNode;

/// Active demon's slot
#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum ActiveSlot {
    #[default]
    First,
    Second,
    Third,
}
