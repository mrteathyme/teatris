fn main() {
    const HEIGHT: usize = 20;
    const WIDTH: usize = 4;
    let mut board: GameBoard<HEIGHT,WIDTH> = GameBoard {lines: [Line::new();HEIGHT]};
    loop {
        let mut piece = Piece {rotation: Rotation::Up, shape: Shape::O, position: Position {column: 0, row: HEIGHT-1}};
        println!("{:#?}", board.drop_piece(&mut piece).unwrap());
    }
}


#[derive(Copy, Clone, Debug)]
enum Tile {
    Filled,
    Empty
}

#[derive(Copy, Clone, Debug)]
struct Line<const WIDTH: usize> {tiles: [Tile;WIDTH]}
impl<const T: usize> Line<T> {
    fn new() -> Self {
        Self{tiles: [Tile::Empty;T]}
    }
}

#[derive(Debug)]
struct GameBoard<const HEIGHT: usize, const WIDTH: usize>{
    lines: [Line<WIDTH>;HEIGHT]
}


#[derive(Copy,Clone, Debug)]
enum Rotation {
    Left,
    Right,
    Up,
    Down,
}

impl Rotation {
    fn counter_clockwise(&self) -> Self {
        match self {
            Self::Left => { Self::Down },
            Self::Right => { Self::Up },
            Self::Up => { Self::Left },
            Self::Down => { Self::Right }
        }
    }
    fn clockwise(&self) -> Self {
        match self {
            Self::Left => { Self::Up },
            Self::Right => { Self::Down },
            Self::Up => { Self::Right},
            Self::Down => { Self::Left }
        }
    }
}

#[derive(Debug, Clone)]
struct Piece {
    rotation: Rotation,
    shape: Shape,
    position: Position
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L
}

#[derive(Copy, Clone, Debug)]
struct Position {
    column: usize,
    row: usize
}

use std::ops::{Add, AddAssign};

impl Add<Rotation> for Position {
    type Output = Position;
    fn add(self, rhs: Rotation) -> Self::Output {
       match rhs {
        Rotation::Up => { Position { column: self.column, row: self.row + 1}},
        Rotation::Down => {Position { column: self.column, row: self.row - 1}},
        Rotation::Left => {Position {column: self.column - 1, row: self.row}},
        Rotation::Right => {Position {column: self.column + 1, row: self.row}}
       }
    }
}

impl Add for Rotation {
    type Output = Rotation;
    fn add(self, rhs: Self) -> Self::Output {
       match rhs {
           Self::Up => {self},
           Self::Down => {self.clockwise().clockwise()},
           Self::Right => {self.clockwise()},
           Self::Left => {self.counter_clockwise()}
            
       }
    }
}

impl AddAssign<Rotation> for Position {
    fn add_assign(&mut self, rhs: Rotation) {
        self.column = (*self + rhs).column;
        self.row = (*self + rhs).row;
    }
}

impl Shape {
    fn get_tiles(&self, rotation: Rotation, origin: Position) -> [Position; 4] {
        let up = Rotation::Up + rotation;
        let right = Rotation::Right + rotation;
        let down = Rotation::Down + rotation;
        let left = Rotation::Left + rotation;

        match self {
            Self::I => [origin, origin+right, origin+right+right, origin+right+right+right],
            Self::O => [origin, origin+right, origin+up, origin+up+right],
            _ => [origin, origin, origin, origin]
        }
    }
}

#[derive(Debug)]
enum GameOver {
    Ceiling
}

impl<const HEIGHT: usize, const WIDTH: usize> GameBoard<HEIGHT, WIDTH> {
    fn drop_piece(&mut self, piece: &mut Piece) -> Result<(), GameOver> {
        'dropping: while piece.position.row > 0{
            for tile in piece.get_tiles() {
                if tile.row <= 0 {
                    break 'dropping
                }
                if let Tile::Filled = self.lines[tile.row-1].tiles[tile.column] {
                    break 'dropping
                }
            }
        piece.position += Rotation::Down;
        }
        for tile in piece.get_tiles() {
            if tile.row >= HEIGHT {
                return Err(GameOver::Ceiling);
            }
            self.lines[tile.row].tiles[tile.column] = Tile::Filled;
        }
        Ok(())
    } 
}

impl Piece {
    fn rotate_clockwise(&mut self) {
        self.rotation = self.rotation.clockwise()
    }

    fn rotate_counter_clockwise(&mut self) {
        self.rotation = self.rotation.counter_clockwise()
    }
    fn get_tiles(&self) -> [Position; 4] {
        self.shape.get_tiles(self.rotation, self.position)
    }
}
