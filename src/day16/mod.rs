use crate::utils;
use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub values: Vec<Vec<i32>>,
    pub nb_rows: usize,
    pub nb_cols: usize,
}

impl Matrix {
    pub fn new(nb_rows: usize, nb_cols: usize) -> Matrix {
        Matrix {
            values: vec![vec![0; nb_cols]; nb_rows],
            nb_rows,
            nb_cols,
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::new(self.nb_rows, other.nb_cols);
        for i in 0..self.nb_rows {
            for j in 0..other.nb_cols {
                for k in 0..self.nb_cols {
                    result.values[i][j] += self.values[i][k] * other.values[k][j];
                }
            }
        }
        result
    }

    pub fn fill(&mut self, value: i32) {
        for i in 0..self.nb_rows {
            for j in 0..self.nb_cols {
                self.values[i][j] = value;
            }
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.values {
            for value in row {
                write!(f, "{:2} ", value)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_line(input: &str) -> IResult<&str, (&str, i32, Vec<&str>)> {
    tuple((
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), utils::parse_int),
        preceded(
            tuple((
                tag("; tunnel"),
                opt(tag("s")),
                tag(" lead"),
                opt(tag("s")),
                tag(" to valve"),
                opt(tag("s")),
                tag(" "),
            )),
            separated_list1(tag(", "), alpha1),
        ),
    ))(input)
}

fn parse_input(filename: &str) -> (Matrix, Vec<i32>) {
    // parse input and make node indexes map, adjacency lists and list of flow rates
    let mut node_indexes = HashMap::new();
    let mut adjacency = Vec::new();
    let mut flow_rates = Vec::new();

    let input = utils::read_input(filename).unwrap();
    // sort input lines in ascending order (so that AA is the first node)
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.sort();

    for line in lines {
        let (_, (label, flow_rate, neighbor_labels)) = parse_line(line).unwrap();
        node_indexes.insert(label, adjacency.len());
        flow_rates.push(flow_rate);
        let mut node_neighbors = Vec::new();
        for neighbor_label in neighbor_labels {
            if let Some(neighbor_index) = node_indexes.get(neighbor_label) {
                node_neighbors.push(*neighbor_index);
            }
        }
        adjacency.push(node_neighbors);
    }

    // Initialize empty distance matrix
    let nb_nodes = adjacency.len();
    let mut matrix = Matrix::new(nb_nodes, nb_nodes);

    // Fill distance matrix
    for (node_index, node_neighbors) in adjacency.iter().enumerate() {
        for neighbor_index in node_neighbors {
            matrix.values[node_index][*neighbor_index] = 1;
            matrix.values[*neighbor_index][node_index] = 1;
        }
    }

    // return adjacency matrix and list of flow rates
    (matrix, flow_rates)
}

fn floyd_warshall(adj_matrix: Matrix) -> Matrix {
    let n = adj_matrix.nb_rows;
    let mut distances = Matrix::new(n, n);
    distances.fill(std::i32::MAX);

    for i in 0..n {
        distances.values[i][i] = 0;
        for j in 0..n {
            if adj_matrix.values[i][j] == 1 {
                distances.values[i][j] = 1;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distances.values[i][k] != std::i32::MAX
                    && distances.values[k][j] != std::i32::MAX
                {
                    distances.values[i][j] =
                        distances.values[i][j].min(distances.values[i][k] + distances.values[k][j]);
                }
            }
        }
    }

    distances
}

fn solve1_aux(
    distances: &Matrix,
    flow_rates: &Vec<i32>,
    current_node: usize,
    remaining_time: i32,
    remaining_valves: HashSet<usize>,
) -> i32 {
    let mut best_score = 0;
    for j in &remaining_valves {
        if distances.values[current_node][*j] >= remaining_time - 1 {
            // there isn't enough time to open the valve
            continue;
        }
        let mut new_remaining_valves = remaining_valves.clone();
        new_remaining_valves.remove(&j);
        let score = solve1_aux(
            distances,
            flow_rates,
            *j,
            remaining_time - distances.values[current_node][*j] - 1,
            new_remaining_valves,
        ) + flow_rates[*j] * (remaining_time - distances.values[current_node][*j] - 1);
        best_score = best_score.max(score);
    }
    best_score
}

pub fn solve1() -> i32 {
    let (adjacency_matrix, flow_rates) = parse_input("src/day16/input.txt");
    let distances = floyd_warshall(adjacency_matrix);

    let mut remaining_valves = HashSet::new();
    for i in 0..flow_rates.len() {
        if flow_rates[i] > 0 {
            remaining_valves.insert(i);
        }
    }
    solve1_aux(&distances, &flow_rates, 0, 30, remaining_valves)
}

pub fn solve2() -> i32 {
    let (adjacency_matrix, flow_rates) = parse_input("src/day16/input.txt");
    let distances = floyd_warshall(adjacency_matrix);

    let mut best_score = 0;
    let mut active_valves = Vec::new();
    for i in 0..flow_rates.len() {
        if flow_rates[i] > 0 {
            active_valves.push(i);
        }
    }

    for i in 0..(1 << active_valves.len()) {
        let mut valves1 = HashSet::new();
        let mut valves2 = HashSet::new();
        for j in 0..active_valves.len() {
            if i & (1 << j) != 0 {
                valves1.insert(active_valves[j]);
            } else {
                valves2.insert(active_valves[j]);
            }
        }

        let score1 = solve1_aux(&distances, &flow_rates, 0, 26, valves1);
        let score2 = solve1_aux(&distances, &flow_rates, 0, 26, valves2);
        best_score = best_score.max(score1 + score2);
    }
    best_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 2080);
    }

    #[test]
    #[ignore = "long test (125s)"]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 2752);
    }
}
