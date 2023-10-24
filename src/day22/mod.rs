use nom::{
    branch::alt,
    character::complete::{digit1, one_of},
    combinator::map,
    multi::many1,
    IResult,
};

use crate::utils;

static DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug)]
enum Instruction {
    Rotate(i32),
    Move(i32),
}

struct CubeFace {
    offset_i: i32,
    offset_j: i32,
    top: (usize, i32),
    bottom: (usize, i32),
    left: (usize, i32),
    right: (usize, i32),
}

struct Cube {
    size: i32,
    map: Vec<Vec<u8>>,
    faces: Vec<CubeFace>,
}

#[derive(Debug)]
struct CubePosition {
    face_index: usize,
    i: i32,
    j: i32,
    dir: i32,
}

impl Cube {
    fn new(filename: &str, size: i32) -> Cube {
        let map_input = utils::read_input(filename).unwrap();
        let map = map_input
            .lines()
            .map(|l| l.chars().map(|c| c as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let faces = vec![
            CubeFace {
                offset_i: 0,
                offset_j: size,
                top: (5, 1),
                bottom: (2, 0),
                left: (4, 2),
                right: (1, 0),
            },
            CubeFace {
                offset_i: 0,
                offset_j: 2 * size,
                top: (5, 0),
                bottom: (2, 1),
                left: (0, 0),
                right: (3, 2),
            },
            CubeFace {
                offset_i: size,
                offset_j: size,
                top: (0, 0),
                bottom: (3, 0),
                left: (4, -1),
                right: (1, -1),
            },
            CubeFace {
                offset_i: 2 * size,
                offset_j: size,
                top: (2, 0),
                bottom: (5, 1),
                left: (4, 0),
                right: (1, 2),
            },
            CubeFace {
                offset_i: 2 * size,
                offset_j: 0,
                top: (2, 1),
                bottom: (5, 0),
                left: (0, 2),
                right: (3, 0),
            },
            CubeFace {
                offset_i: 3 * size,
                offset_j: 0,
                top: (4, 0),
                bottom: (1, 0),
                left: (0, -1),
                right: (3, -1),
            },
        ];

        Cube {
            size,
            map,
            faces,
        }
    }

    fn step(&self, p: &CubePosition) -> CubePosition {
        let (di, dj) = DIRECTIONS[p.dir as usize];
        let mut i = p.i + di;
        let mut j = p.j + dj;
        let mut dir = p.dir;
        let mut face_index = p.face_index;
        let new_face;
        let k;

        // check if moving to another face
        if i < 0 {
            // moving up to next face
            k = j;
            new_face = self.faces[face_index].top;
        } else if i >= self.size  {
            // moving down to next face
            k = self.size - j - 1;
            new_face = self.faces[face_index].bottom;
        } else if j < 0 {
            // moving left to next face
            k = self.size - i - 1;
            new_face = self.faces[face_index].left;
        } else if j >= self.size {
            // moving right to next face
            k = i;
            new_face = self.faces[face_index].right;
        } else {
            // move within current face
            return CubePosition {
                face_index,
                i,
                j,
                dir,
            };
        }

        face_index = new_face.0;
        dir = (dir + new_face.1).rem_euclid(4);
        match dir {
            0 => {
                // entering from left side
                i = k;
                j = 0;
            }
            1 => {
                // entering from top side
                i = 0;
                j = self.size - k - 1;
            }
            2 => {
                // entering from right side
                i = self.size - k - 1;
                j = self.size - 1;
            }
            3 => {
                // entering from bottom side
                i = self.size - 1;
                j = k;
            }
            _ => panic!("Invalid direction"),
        }
        CubePosition {
            face_index: face_index,
            i,
            j,
            dir,
        }
    }

    fn get_cell(&self, p: &CubePosition) -> u8 {
        self.map[(p.i + self.faces[p.face_index].offset_i) as usize]
            [(p.j + self.faces[p.face_index].offset_j) as usize]
    }
}

fn parse_move(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(one_of("RL"), |c| match c {
            'R' => Instruction::Rotate(1),
            'L' => Instruction::Rotate(-1),
            _ => panic!("Invalid rotation"),
        }),
        map(digit1, |s: &str| {
            Instruction::Move(s.parse::<i32>().unwrap())
        }),
    ))(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_move)(input)
}

fn parse_map(filename: &str) -> Vec<Vec<u8>> {
    let map_input = utils::read_input(filename).unwrap();
    let mut map = map_input
        .lines()
        .map(|l| l.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_width = map.iter().map(|l| l.len()).max().unwrap();
    for line in map.iter_mut() {
        while line.len() < max_width {
            line.push(' ' as u8);
        }
    }
    map
}

pub fn solve1() -> i32 {
    // parse instructions
    let (_, instructions) =
        parse_moves(&utils::read_input("src/day22/instructions.txt").unwrap()).unwrap();

    // parse map
    let map = parse_map("src/day22/map.txt");
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    // find starting position
    let (mut i, mut j) = (0, 0);
    while map[i as usize][j as usize] != b'.' {
        j += 1;
    }

    let mut dir = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Rotate(r) => {
                dir = (dir + r).rem_euclid(4);
            }
            Instruction::Move(m) => {
                let (di, dj) = DIRECTIONS[dir as usize];
                for _ in 0..m {
                    if di != 0 {
                        // move vertically
                        let mut i2 = (i + di).rem_euclid(height as i32);
                        while map[i2 as usize][j as usize] == b' ' {
                            i2 = (i2 + di).rem_euclid(height as i32);
                        }
                        if map[i2 as usize][j as usize] == b'#' {
                            // hit a wall
                            break;
                        } else {
                            i = i2;
                        }
                    } else {
                        // move horizontally
                        let mut j2 = (j + dj).rem_euclid(width as i32);
                        while map[i as usize][j2 as usize] == b' ' {
                            j2 = (j2 + dj).rem_euclid(width as i32);
                        }
                        if map[i as usize][j2 as usize] == b'#' {
                            // hit a wall
                            break;
                        } else {
                            j = j2;
                        }
                    }
                }
            }
        }
    }
    1000 * (i + 1) + 4 * (j + 1) + dir
}

pub fn solve2() -> i32 {
    // parse instructions
    let (_, instructions) =
        parse_moves(&utils::read_input("src/day22/instructions.txt").unwrap()).unwrap();

    // make cube from map
    let cube = Cube::new("src/day22/map.txt", 50);

    let mut p = CubePosition {
        face_index: 0,
        i: 0,
        j: 0,
        dir: 0,
    };

    for instruction in instructions {
        match instruction {
            Instruction::Rotate(r) => {
                p.dir = (p.dir + r).rem_euclid(4);
            }
            Instruction::Move(m) => {
                for _ in 0..m {
                    let p2 = cube.step(&p);
                    if cube.get_cell(&p2) == b'#' {
                        // hit a wall
                        break;
                    } else {
                        p = p2;
                    }
                }
            }   
        }
    }

    let f = &cube.faces[p.face_index];
    1000 * (p.i + f.offset_i + 1) + 4 * (p.j + f.offset_j + 1) + p.dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 126350);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 129339);
    }
}
