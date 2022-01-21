extern crate ncurses;
extern crate rand;
use ncurses::*;

#[derive(PartialEq)]
enum Status {
    Success,
    Failure,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

struct Board {
    xmax: u32,
    ymax: u32,
    snake: Vec<Point>,
    foods: Vec<Point>,
}

impl Board {
    fn eat_food(&mut self, point: Point) {
        self.snake.insert(0, point);
    }

    fn move_to(&mut self, point: Point) {
        self.snake.insert(0, point);
        self.snake.pop();
    }

    fn add_new_food(&mut self) {
        let mut point = self.create_random_cell();
        while self.snake.contains(&point) || self.foods.contains(&point) {
            point = self.create_random_cell();
        }
        self.foods.push(point);
    }

    fn create_random_cell(&self) -> Point {
        Point {
            x: rand::random::<u32>() % self.xmax,
            y: rand::random::<u32>() % self.ymax,
        }
    }

    fn initialize(&mut self) {
        self.snake.push(Point { x: 2, y: 3 });
        self.snake.push(Point { x: 2, y: 2 });

        let num_food = self.xmax * self.ymax / 10;
        for _ in 1..num_food {
            self.add_new_food();
        }
    }

    fn move_snake(&mut self, dir: Direction) -> Status {
        let beginning = self.next_move(dir);
        if beginning.is_err() {
            return Status::Failure;
        }
        let point: Point = beginning.unwrap();

        if self.snake.contains(&point) {
            return Status::Failure;
        }
        if self.foods.contains(&point) {
            self.eat_food(point);
            self.foods.retain(|&x| x != point);
            self.add_new_food();
            return Status::Success;
        }
        self.move_to(point);
        Status::Success
    }

    fn next_move(&self, dir: Direction) -> Result<Point, ()> {
        let head = &self.snake[0];
        let mut new_x = head.x as i32;
        let mut new_y = head.y as i32;
        match dir {
            Direction::Up => {
                new_y -= 1;
            }
            Direction::Down => {
                new_y += 1;
            }
            Direction::RIGHT => {
                new_x += 1;
            }
            Direction::Left => {
                new_x -= 1;
            }
        }
        if new_x < 0 || new_y < 0 || new_x >= self.xmax as i32 || new_y >= self.ymax as i32 {
            Err(())
        } else {
            Ok(Point {
                x: new_x as u32,
                y: new_y as u32,
            })
        }
    }
}
fn display_points(snake: &[Point], symbol: chtype) {
    for point in snake {
        mvaddch(point.y as i32, point.x as i32, symbol);
    }
}

fn get_next_move(previous: Direction) -> Direction {
    let ch = getch();
    match (ch, previous) {
        (KEY_LEFT, Direction::Right) => previous,
        (KEY_RIGHT, Direction::Left) => previous,
        (KEY_UP, Direction::Down) => previous,
        (KEY_DOWN, Direction::Up) => previous,
        (KEY_RIGHT, _) => Direction::Right,
        (KEY_LEFT, _) => Direction::Left,
        (KEY_DOWN, _) => Direction::Down,
        (KEY_UP, _) => Direction::Up,
        _ => previous,
    }
}

fn main() {
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(100);

    let mut xmax: i32 = 0;
    let mut ymax: i32 = 0;
    getmaxyx(stdscr(), &mut ymax, &mut xmax);
    let mut dir = Direction::Right;

    let mut board = Board {
        xmax: xmax as u32,
        ymax: ymax as u32,
        foods: vec![],
        snake: vec![],
    };

    board.initialize();
    let mut status = Status::Success;
    while status == Status::Success {
        clear();
        display_points(&board.snake, ACS_DEGREE());
        display_points(&board.foods, ACS_DIAMOND());
        refresh();
        dir = get_next_move(dir);
        status = board.move_snake(dir);
    }
    endwin();
}
