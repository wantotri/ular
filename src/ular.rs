// #![allow(dead_code)]
use std::{fmt::Display, collections::VecDeque};
use rand::{Rng, prelude::SliceRandom};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

type Position = (usize, usize);

pub struct Snake {
    body: VecDeque<Position>,
    direction: Direction,
    head_icon: &'static str,
    body_icon: &'static str
}

impl Snake {
    /// Create a new Snake
    pub fn new(
        x: usize,
        y: usize,
        head_icon: &'static str,
        body_icon: &'static str
    ) -> Snake {
        let x0 = (x).max(3);
        let y0 = (y).max(0);
        Snake {
            body: VecDeque::from([(x0, y0), (x0-1, y0), (x0-2, y0)]),
            direction: Direction::Right,
            head_icon,
            body_icon
        }
    }

    /// Get snake body length
    pub fn len(&self) -> usize {
        self.body.len()
    }

    /// Get possible turn directions, given self.direction
    pub fn possible_directions(&self) -> Vec<Direction> {
        match self.direction {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }

    /// Check if the snake can turn to new direction
    pub fn can_change_direction_to(&self, new_direction: &Direction) -> bool {
        self.possible_directions()
            .iter()
            .any(|direction| { new_direction == direction })
    }

    /// Moves or grow the snake according to the direction
    pub fn moves(&mut self, direction: Direction, grow: bool) -> &mut Self {
        if self.direction == direction
        || self.can_change_direction_to(&direction) {
            let (x, y) = self.body[0];
            match direction {
                Direction::Left => self.body.push_front((x-1, y)),
                Direction::Right => self.body.push_front((x+1, y)),
                Direction::Up => self.body.push_front((x, y+1)),
                Direction::Down => self.body.push_front((x, y-1))
            }
            if !grow { self.body.pop_back(); }
            if self.direction != direction { self.direction = direction; }
        }
        self
    }

    pub fn include(&self, position: Position) -> bool {
        self.body.iter().any(|pos| { pos == &position })
    }
}

pub struct Food {
    pos: Position,
    icon: &'static str
}

impl Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.icon)
    }
}

impl Food {
    pub fn new(pos: Position, icon: &'static str) -> Food {
        Food { pos, icon }
    }
}

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Position>>,
    cell_icon: &'static str
}

impl Board {
    pub fn new(
        width: usize,
        height: usize,
        cell_icon: &'static str
    ) -> Board {
        let cells: Vec<Vec<Position>> =
            (0..height.max(5)).map(|y| {
                (0..width.max(5)).map(|x| {
                    (x, y)
                }).collect()
            }).collect();

        Board { width, height, cells, cell_icon }
    }

    pub fn cells_count(&self) -> usize {
        self.cells.len() * self.cells[0].len()
    }
}

pub struct Game {
    board: Board,
    snake: Snake,
    food: Food,
    over: bool,
    score: usize
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..(self.board.height)).rev() {
            for x in 0..(self.board.width) {
                if (x, y) == self.food.pos {
                    write!(f, "{}", self.food.icon)?;
                } else if self.snake.body[0] == (x, y) {
                    write!(f, "{}", self.snake.head_icon)?;
                } else if self.snake.include((x, y)) {
                    write!(f, "{}", self.snake.body_icon)?;
                } else {
                    write!(f, "{}", self.board.cell_icon)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Game {
    pub fn new(
        width: usize,
        height: usize,
        cell_icon: &'static str,
        food_icon: &'static str,
        head_icon: &'static str,
        body_icon: &'static str,
    ) -> Game {
        let board = Board::new(width, height, cell_icon);
        let snake = Snake::new(width/2, height/2, head_icon, body_icon);

        let mut rng = rand::thread_rng();
        let mut food_pos: Position = (rng.gen_range(0..width), rng.gen_range(0..height));
        while snake.include(food_pos) {
            food_pos = (rng.gen_range(0..width), rng.gen_range(0..height));
        };
        let food = Food::new(food_pos, food_icon);

        Game { board, snake, food, over: false, score: 0 }
    }

    /// Get the game score
    pub fn get_score(&self) -> usize {
        self.score
    }

    /// Get the game "over" value
    pub fn is_over(&self) -> bool {
        self.over
    }

    /// Get all available cells in the board
    pub fn get_available_cells(&self) -> Vec<Position> {
        let mut cells: Vec<Position> = self.board.cells
            .clone()
            .into_iter()
            .flatten()
            .collect();

        for body in self.snake.body.iter() {
            let index = cells
                .iter()
                .position(|pos| { body == pos })
                .unwrap();
            cells.remove(index);
        }

        cells
    }

    /// Randomly spawn a food in available cells
    pub fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        let available_cells = self.get_available_cells();

        if available_cells.len() == 0 {
            self.over = true;
            return;
        }

        self.food.pos = *available_cells.choose(&mut rng).unwrap();
    }

    /// Get the snake direction
    pub fn get_snake_direction(&self) -> Direction {
        self.snake.direction
    }

    /// Change the snake direction
    pub fn set_snake_direction(&mut self, direction: Direction) {
        if self.snake.can_change_direction_to(&direction) {
            self.snake.direction = direction
        }
    }

    /// Game tick
    pub fn tick(&mut self) {
        let mut grow = false;
        let head = self.snake.body[0];
        let next_head: Position;

        // Check if snake head out of board
        if (head.0 == 0 && self.snake.direction == Direction::Left)
        || (head.1 == 0 && self.snake.direction == Direction::Down)
        || (head.0 == self.board.width - 1 && self.snake.direction == Direction::Right)
        || (head.1 == self.board.height - 1 && self.snake.direction == Direction::Up) {
            self.over = true;
            return
        };

        match self.snake.direction {
            Direction::Right => next_head = (head.0 + 1, head.1),
            Direction::Left => next_head = (head.0 - 1, head.1),
            Direction::Up => next_head = (head.0, head.1 + 1),
            Direction::Down => next_head = (head.0, head.1 - 1)
        };

        // Check if the snake will collide with its own body
        if self.snake.include(next_head) {
            self.over = true;
            return
        };

        if next_head == self.food.pos { grow = true; };
        self.snake.moves(self.snake.direction, grow);
        if grow {
            self.spawn_food();
            self.score += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_new() {
        let snake = Snake::new(4, 0, "🟪", "🟦");
        assert_eq!(snake.body, vec![(4, 0), (3, 0), (2, 0)])
    }

    #[test]
    fn snake_len() {
        let snake = Snake::new(4, 0, "🟪", "🟦");
        assert_eq!(snake.len(), 3)
    }

    #[test]
    fn snake_possible_directions() {
        let mut snake = Snake::new(0, 0, "🟪", "🟦");
        assert_eq!(snake.possible_directions(), vec![Direction::Up, Direction::Down]);

        snake.direction = Direction::Left;
        assert_eq!(snake.possible_directions(), vec![Direction::Up, Direction::Down]);

        snake.direction = Direction::Down;
        assert_eq!(snake.possible_directions(), vec![Direction::Left, Direction::Right]);

        snake.direction = Direction::Up;
        assert_eq!(snake.possible_directions(), vec![Direction::Left, Direction::Right]);
    }

    #[test]
    fn snake_can_change_direction_to() {
        let mut snake = Snake::new(0, 0, "🟪", "🟦");
        assert!(snake.can_change_direction_to(&Direction::Up));
        assert!(snake.can_change_direction_to(&Direction::Down));
        assert!(!snake.can_change_direction_to(&Direction::Left));
        assert!(!snake.can_change_direction_to(&Direction::Right));

        snake.direction = Direction::Down;
        assert!(snake.can_change_direction_to(&Direction::Left));
        assert!(snake.can_change_direction_to(&Direction::Right));
        assert!(!snake.can_change_direction_to(&Direction::Down));
        assert!(!snake.can_change_direction_to(&Direction::Up));
    }

    #[test]
    fn snake_moves() {
        let mut snake = Snake::new(4, 0, "🟪", "🟦");
        snake.moves(Direction::Right, false);
        assert_eq!(snake.body, vec![(5, 0), (4, 0), (3, 0)]);

        snake.moves(Direction::Up, false);
        assert_eq!(snake.body, vec![(5, 1), (5, 0), (4, 0)]);
        assert_eq!(snake.direction, Direction::Up);

        snake.moves(Direction::Left, false);
        assert_eq!(snake.body, vec![(4, 1), (5, 1), (5, 0)]);
        assert_eq!(snake.direction, Direction::Left);

        snake.moves(Direction::Down, false);
        assert_eq!(snake.body, vec![(4, 0), (4, 1), (5, 1)]);
        assert_eq!(snake.direction, Direction::Down);
    }

    #[test]
    fn snake_grow() {
        let mut snake = Snake::new(4, 0, "🟪", "🟦");
        snake.moves(Direction::Right, true);
        assert_eq!(snake.body, vec![(5, 0), (4, 0), (3, 0), (2, 0)]);

        snake.moves(Direction::Up, true);
        assert_eq!(snake.body, vec![(5, 1), (5, 0), (4, 0), (3, 0), (2, 0)]);

        snake.moves(Direction::Left, true);
        assert_eq!(snake.body, vec![(4, 1), (5, 1), (5, 0), (4, 0), (3, 0), (2, 0)]);

        let mut snake = Snake::new(4, 4, "🟪", "🟦");
        snake.moves(Direction::Down, true);
        assert_eq!(snake.body, vec![(4, 3), (4, 4), (3, 4), (2, 4)]);
    }

    #[test]
    fn board_new() {
        let board = Board::new(9, 7, "⬛");
        assert_eq!(board.cells[0].len(), 9);
        assert_eq!(board.cells.len(), 7);
    }

    #[test]
    fn board_cells_count() {
        let board = Board::new(7, 8, "⬛");
        assert_eq!(board.cells_count(), 7 * 8)
    }
}