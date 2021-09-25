use std::fmt;
use std::option::Option;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}


// Color enum
#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black
}


// Piece enum
#[derive(Copy, Clone)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

impl Piece {
    fn take_turn(&self, /*...*/)  {
        
    }

    // ...
}

pub struct Game {
    /* save board, active colour, ... */
    board: [[Option<(Piece, Color)>; 8]; 8],
    state: GameState
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */

            state: GameState::InProgress,
            board: {
                let mut board: [[Option<(Piece, Color)>; 8]; 8] = [[None; 8]; 8];
                for (row, row_pieces) in board.iter_mut().enumerate() {
                    for (column, piece) in row_pieces.iter_mut().enumerate() {
                        let color = {if row == 0 {Color::Black} else {Color::White}};
                        match row {
                            0 | 7 => {
                                *piece = {
                                    match column {
                                        0 | 7 => {Some((Piece::Rook, color))},
                                        1 | 6 => {Some((Piece::Knight, color))},
                                        2 | 5 => {Some((Piece::Bishop, color))},
                                        3 => {Some((Piece::Queen, color))},
                                        4 => {Some((Piece::King, color))},
                                        _ => {None}
                                    }
                                }
                            }
                            1 | 6 => { *piece = Some((Piece::Pawn, color)); }
                            _ => {continue;}
                        }
                    }
                }
                board
            }
        }
    }

    

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        None
    }

    /// Set the piece type that a peasant becomes following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
        None
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        
        write!(f, "s");
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}