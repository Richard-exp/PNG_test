use std::collections::VecDeque;

const START: Pos = Pos { row: 75, column: 1 };
const FINISH: Pos = Pos { row: 1, column: 199 };
const WIDHT: usize = 200; // square-matrix
const HEIGH: usize = 150;

#[derive(Clone, PartialEq)]
pub struct Pos {
    pub row: i32,
    pub column: i32,
}

#[derive(PartialEq)]
struct Visited {
    node: Pos,
    parent: Pos,
}

struct Neighbours {
    queue: VecDeque<Pos>,
    visited: Vec<Visited>,
}

impl Visited {
    fn new(position: Pos, current: Pos) -> Self {
        Self {
            node: position,
            parent: current,
        }
    }
}

impl Neighbours {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            visited: Vec::new(),
        }
    }

    fn check_clockwise(&mut self, matrix: &Vec<Vec<u8>>) -> Result<&str, ()> {
        if let Some(current) = self.queue.pop_front() {
            #[rustfmt::skip]
            let iterator_arr: [Pos; 8] = [Pos {row: current.row, column: current.column + 1},
                                          Pos {row: current.row + 1, column: current.column + 1},
                                          Pos {row: current.row + 1, column: current.column},
                                          Pos {row: current.row + 1, column: current.column - 1},
                                          Pos {row: current.row, column: current.column - 1},
                                          Pos {row: current.row - 1, column: current.column - 1},
                                          Pos {row: current.row - 1, column: current.column},
                                          Pos {row: current.row - 1, column: current.column + 1}];

            for neighbour in iterator_arr.iter() {
                if !(neighbour.row >= 0
                    && neighbour.row < HEIGH as i32
                    && neighbour.column >= 0
                    && neighbour.column < WIDHT as i32)
                {
                    continue;
                }
                let row = neighbour.row as usize;
                let column = neighbour.column as usize;

                if matrix[row][column] != 1 {
                    // Visits only passable neighbors (1s)
                    continue;
                }
                let visited_neighbour = Visited::new(neighbour.clone(), current.clone());
                if let Some(_) = self
                    .visited
                    .iter()
                    .find(|vis| vis.node == visited_neighbour.node)
                {
                    continue;
                } else {
                    self.visited.push(visited_neighbour);
                    self.queue.push_back(neighbour.clone());
                    if neighbour == &FINISH {
                        return Ok("Path is: ");
                    }
                }
            }
            Err(())
        } else {
            Ok("There is no solution")
        }
    }

    fn generate_path(&self) -> Vec<Pos> {
        let mut path: Vec<Pos> = Vec::new();
        let mut parent = FINISH;
        while let Some(visited_node) = self.visited.iter().find(|vis| vis.node == parent) {
            path.push(visited_node.node.clone());
            if visited_node.node == START {
                break;
            }
            parent = visited_node.parent.clone();
        }
        path.iter().rev().map(|pos| pos.clone()).collect()
    }
}

pub fn find_path(matrix: &Vec<Vec<u8>>) -> Vec<Pos> {
    let mut neighbours = Neighbours::new();
    neighbours.queue.push_back(START);
    neighbours.visited.push(Visited::new(START, START));

    loop {
        match neighbours.check_clockwise(&matrix) {
            Err(()) => {}
            Ok(message) => {
                println!("{message}");
                break;
            }
        }
    }
    let path: Vec<Pos> = neighbours.generate_path();

    let mut counter = 0;
    path.iter().for_each(|pos| {
        counter += 1;
        print!(" -> [{},{},{}]", pos.row, pos.column, counter)
    });

    path
}