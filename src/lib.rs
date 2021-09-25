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
#[derive(Copy, Clone, Debug)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

// Cred: https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Piece {
    fn take_turn(&self, /*...*/)  {
        
    }

    fn represent(&self) -> &str {
        match self {
            Piece::King => {"K"},
            Piece::Queen => {"Q"},
            Piece::Bishop => {"B"},
            Piece::Knight => {"Kn"},
            Piece::Rook => {"R"},
            Piece::Pawn => {"P"},
        }
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
        let mut board_representation = String::from("");
        board_representation.push_str("\n|:------------------------:|\n");
        // row_pieces - the row that contains the current iterated pieces 
        for row_pieces in &self.board {
            let mut row_representation = String::from("");
            // starting wall |
            row_representation.push_str("|  ");
            
            // ppiece - potential piece, it could also be None
            for ppiece in row_pieces {
                match ppiece {
                    // now that I know it's a piece I can call it piece
                    Some((piece, _color)) => {
                        // this ugly match statement is needed because Knight is represented by two characters 'Kn'
                        // and without this compromise the board indentations get weird
                        match piece {
                            Piece::Knight => {
                                row_representation.push_str(piece.represent());
                                row_representation.push_str(" ");
                            },
                            _ => {
                                row_representation.push_str(piece.represent());
                                row_representation.push_str("  ");
                            }
                        }
                    },
                    None => {
                        row_representation.push_str("*");
                        row_representation.push_str("  ");
                    }
                }
                
            }
            row_representation.push_str("|\n");
            // borrow the row_representation so that it is not owned before pushing
            board_representation.push_str(&row_representation);
        }
        board_representation.push_str("|:------------------------:|");
        write!(f, "{}", board_representation)
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