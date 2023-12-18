use piece::*;
use piece::chess_core::*;

#[path="piece.rs"] pub mod piece;

pub struct Chess_Board
{
    board: [[Piece; 8]; 8],
    current_player: Player,
}

impl Chess_Board
{
    pub fn new(player: Player) -> Self
    {
        let mut new_board = Chess_Board { 
            board: [[Piece::new(Piece_Type::Empty, Player::None); 8]; 8],
            current_player: player,
        };
        new_board.initial_setup();
        return new_board;
    }
    
    fn initial_setup(&mut self)
    {
        self.board[0][0] = Piece::new(Piece_Type::Rook,   Player::Black);
        self.board[0][1] = Piece::new(Piece_Type::Knight, Player::Black);
        self.board[0][2] = Piece::new(Piece_Type::Bishop, Player::Black);
        self.board[0][3] = Piece::new(Piece_Type::Queen,  Player::Black);
        self.board[0][4] = Piece::new(Piece_Type::King,   Player::Black);
        self.board[0][5] = Piece::new(Piece_Type::Bishop, Player::Black);
        self.board[0][6] = Piece::new(Piece_Type::Knight, Player::Black);
        self.board[0][7] = Piece::new(Piece_Type::Rook,   Player::Black);
        
        for s in 0..8
        {
            self.board[1][s] = Piece::new(Piece_Type::Pawn, Player::Black);
            self.board[6][s] = Piece::new(Piece_Type::Pawn, Player::White);
        }
        
        self.board[0][0] = Piece::new(Piece_Type::Rook,   Player::White);
        self.board[0][1] = Piece::new(Piece_Type::Knight, Player::White);
        self.board[0][2] = Piece::new(Piece_Type::Bishop, Player::White);
        self.board[0][3] = Piece::new(Piece_Type::Queen,  Player::White);
        self.board[0][4] = Piece::new(Piece_Type::King,   Player::White);
        self.board[0][5] = Piece::new(Piece_Type::Bishop, Player::White);
        self.board[0][6] = Piece::new(Piece_Type::Knight, Player::White);
    }
}

impl std::ops::Index<(usize, usize)> for Chess_Board
{
    type Output = Piece;
    
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return &self.board[index.0][index.1];
    }
}

impl std::ops::Index<chess_core::Square> for Chess_Board
{
    type Output = Piece;
    
    fn index(&self, index: chess_core::Square) -> &Self::Output {
        return &self.board[index.row][index.col];
    }
}

