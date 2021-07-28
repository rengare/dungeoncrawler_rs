#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TurnState {
    AwaitInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
}
