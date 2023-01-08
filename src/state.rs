#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    MainMenu,
    Playing,
    PauseMenu,
}
