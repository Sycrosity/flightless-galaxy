/// Represents a key that the game recognises after input mapping.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    AsRefStr,
    EnumIter,
    EnumString,
)]
pub enum GameInput {
    #[strum(serialize = "gameinput-primary")]
    Primary,
    #[strum(serialize = "gameinput-secondary")]
    Secondary,
    #[strum(serialize = "gameinput-ability")]
    Ability,
    #[strum(serialize = "gameinput-dodge")]
    Dodge,
    #[strum(serialize = "gameinput-shownames")]
    ShowNames,
    #[strum(serialize = "gameinput-togglecursor")]
    ToggleCursor,
    #[strum(serialize = "gameinput-moveleft")]
    MoveLeft,
    #[strum(serialize = "gameinput-moveright")]
    MoveRight,
    #[strum(serialize = "gameinput-fly")]
    Fly,
    #[strum(serialize = "gameinput-jump")]
    Jump,
    #[strum(serialize = "gameinput-chat")]
    Chat,
    #[strum(serialize = "gameinput-escape")]
    Escape,
    #[strum(serialize = "gameinput-settings")]
    Settings,
    #[strum(serialize = "gameinput-toggledebug")]
    ToggleDebug,
    #[strum(serialize = "gameinput-togglechat")]
    ToggleChat,
    #[strum(serialize = "gameinput-fullscreen")]
    Fullscreen,
    #[strum(serialize = "gameinput-respawn")]
    Respawn,
    #[strum(serialize = "gameinput-mutemaster")]
    MuteMaster,
    #[strum(serialize = "gameinput-mutemusic")]
    MuteMusic,
    #[strum(serialize = "gameinput-mutesfx")]
    MuteSfx,
    #[strum(serialize = "gameinput-muteambience")]
    MuteAmbience,
}