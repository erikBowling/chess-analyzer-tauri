use crate::chess::piece::Piece;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone, Debug)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn to_int(self) -> u8 {
        self as u8
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Rank::One),
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            _ => None,
        }
    }
}

#[derive(EnumIter, Copy, Clone, Debug)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn to_int(self) -> u8 {
        self as u8
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'a' => Some(File::A),
            'b' => Some(File::B),
            'c' => Some(File::C),
            'd' => Some(File::D),
            'e' => Some(File::E),
            'f' => Some(File::F),
            'g' => Some(File::G),
            'h' => Some(File::H),
            _ => None,
        }
    }
}

pub struct Square {
    rank: Rank,
    file: File,
    piece: Option<Piece>,
}

impl Square {
    pub fn new(rank: Rank, file: File, piece: Option<Piece>) -> Self {
        Square { rank, file, piece }
    }

    pub fn get_piece(&self) -> Option<&Piece> {
        match &self.piece {
            Some(piece) => Some(&piece),
            None => None,
        }
    }

    pub fn set_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }

    pub fn clear_piece(&mut self) {
        self.piece = None;
    }

    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    /// Parse a square notation like "e2" or "a8" into (File, Rank)
    pub fn parse_notation(notation: &str) -> Option<(Rank, File)> {
        if notation.len() != 2 {
            return None;
        }

        let mut chars = notation.chars();
        let file_char = chars.next()?;
        let rank_char = chars.next()?;

        let file = File::from_char(file_char)?;
        let rank = Rank::from_char(rank_char)?;

        Some((rank, file))
    }
}
