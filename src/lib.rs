use std::fmt;
use std::option::Option;
use std::collections::HashMap;

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
    
    fn get_moves(&self, game: &Game, (file, rank): (usize, usize), color: Color) -> Option<Vec<String>> {
        match self {
            Piece::Rook => {
                let moves: Vec<String> = vec!();
                // There are 4 possible directions that the rook can move
                // Below I handle them in the following order:
                // Right, Left, Up, Down
                for i in file..8 {
                    match game.get_piece((i, rank)) {
                        // t_color for target_color, separated from parameter 
                        // color which is the color of the current piece
                        Some((piece, t_color)) => {
                            if !matches!(color, t_color) {
                                moves.push()
                            }
                        }
                    }
                }
                None
        },
            _ => None
        }
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
    state: GameState,
    turn: Color,
    file_values_i: HashMap<char, usize>,
    file_values_l: [char; 8]
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
            },
            turn: Color::White,
            // Cred for this neat one-liner (user huon): https://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization/58126168
            // Preferably, I would like to have the following value as a constant and not have to create a new 
            // one when a game is initialized, but in Rust you cannot have a constant HashMap
            // I looked at https://crates.io/crates/phf - phf seems to support just this
            // but seems like an overkill in this particular scenario.
            // Furthermore, there are two variables file_values
            // file_values_i is meant to rapidly convert from file (a letter) to an index
            // file_values_l same as above but from index to letter
            file_values_i: "abcdefgh".chars().enumerate().map(|(index, letter)| (letter, index)).collect::<HashMap<_,_>>(),
            file_values_l: {
                let chars = [' '; 8];
                for (index, letter) in "abcdefgh".chars().enumerate() {
                    chars[index] = letter
                }
                chars
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
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        let (file, rank) = self.get_position_as_tuple(&_position);
        match self.get_piece((file, rank)) {
            Some((piece, color)) => {
                // Return None if it's not your turn
                if !matches!(self.turn, color) {
                    return None;
                }
                piece.get_moves(&self, (file, rank), color)
            },
            None => None
        }
    }

    /// Returns the given position as a tuple with format:
    /// (file, rank), where file is the column that the piece is
    /// placed on (0-7). And rank is the row of the piece (also 0-7)
    /// ###Arguments:
    /// position - the position as a string with format "<file><rank>"
    fn get_position_as_tuple(&self, position: &String) -> (usize, usize) {
        let pos_split = position.split_at(1);
        let file: usize = match self.file_values_i.get(&pos_split.0.chars().next().unwrap()) {
            Some(x) => *x,
            None => panic!()
        };
        
        let rank = pos_split.1.parse::<usize>().unwrap()-1;
        (file, rank)
    }

    /// Returns the given position as a string with format:
    /// "<file><rank>"
    /// ###Arguments:
    /// file - the column of the piece as a usize (0-7)
    /// rank - the rank of the piece (0-7)
    fn get_position_as_string(&self, (file, rank): (usize, usize)) -> String {
        let position = String::from("");
        
    }

    /// Returns the piece at the given position
    /// Position (String) should be of format: <file><rank>
    /// Example: A1, B3
    fn get_piece(&self, (file, rank): (usize, usize)) -> Option<(Piece, Color)> {
        return self.board[rank][file];
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
        println!("{:?}", game.get_possible_moves("a1".to_owned()));

        assert_eq!(game.get_game_state(), GameState::InProgress);
        
    }
}