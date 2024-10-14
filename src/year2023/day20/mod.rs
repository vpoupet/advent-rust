use std::fmt;
use std::collections::{ HashMap, VecDeque };

use nom::bytes::complete::tag;
use nom::character::complete::{ alpha1, one_of };
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::{ pair, tuple };
use nom::IResult;

use crate::utils;

#[derive(Debug)]
struct Pulse {
    source: String,
    target: String,
    is_high: bool,
}
impl fmt::Display for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = if self.is_high { "high" } else { "low" };
        write!(f, "{} -{}-> {}", self.source, label, self.target)
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    outputs: Vec<String>,
    flipflop_state: Option<bool>,
    conjunction_state: Option<HashMap<String, bool>>,
}

impl Module {
    fn receive_pulse(&mut self, pulse: &Pulse, pulses: &mut VecDeque<Pulse>) {
        if let Some(flipflop_state) = self.flipflop_state {
            // flip-flop module
            if !pulse.is_high {
                self.flipflop_state = Some(!flipflop_state); // update state
                for dest in &self.outputs {
                    // send new pulse to all destinations
                    pulses.push_back(Pulse {
                        source: self.name.to_string(),
                        target: dest.to_string(),
                        is_high: !flipflop_state,
                    });
                }
            }
        }
        if let Some(conjunction_state) = &mut self.conjunction_state {
            // conjunction module
            conjunction_state.insert(pulse.source.to_string(), pulse.is_high); // update state
            let mut is_all_high = true;
            for state in conjunction_state.values().copied() {
                if !state {
                    is_all_high = false;
                    break;
                }
            }
            // send a low pulse to all destinations
            for dest in &self.outputs {
                pulses.push_back(Pulse {
                    source: self.name.to_string(),
                    target: dest.to_string(),
                    is_high: !is_all_high, // pulse is low if all inputs are high, high otherwise
                });
            }
        }
        if &self.name == "broadcaster" {
            for output in &self.outputs {
                pulses.push_back(Pulse {
                    source: self.name.to_string(),
                    target: output.to_string(),
                    is_high: pulse.is_high,
                });
            }
        }
    }
}

fn parse_module_name(input: &str) -> IResult<&str, (Option<char>, &str)> {
    pair(opt(one_of("&%")), alpha1)(input)
}

fn parse_module_outputs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    for line in input.lines() {
        let (_, ((module_type, name), _, outputs)) = tuple((
            parse_module_name,
            tag(" -> "),
            parse_module_outputs,
        ))(line).unwrap();
        let flipflop_state = match module_type {
            Some('%') => Some(false), // initial state is off
            _ => None,
        };
        let conjunction_state = match module_type {
            Some('&') => Some(HashMap::<String, bool>::new()),
            _ => None,
        };
        modules.insert(name.to_string(), Module {
            name: name.to_string(),
            outputs: outputs
                .iter()
                .map(|s| s.to_string())
                .collect(),
            flipflop_state,
            conjunction_state,
        });
    }

    // make map of inputs for each module
    let mut inputs_map: HashMap<String, Vec<String>> = HashMap::new();
    for (_, module) in &modules {
        for output in &module.outputs {
            inputs_map
                .entry(output.to_string())
                .or_insert(Vec::new())
                .push(module.name.to_string());
        }
    }
    // set initial state for conjunction modules (all inputs are low)
    for (module_name, inputs) in &inputs_map {
        if let Some(module) = modules.get_mut(module_name) {
            if let Some(conjunction_state) = &mut module.conjunction_state {
                for input_name in inputs {
                    conjunction_state.insert(input_name.to_string(), false);
                }
            }
        }
    }
    modules
}

fn press_button(modules: &mut HashMap<String, Module>) -> (u64, u64) {
    let mut pulses = VecDeque::new();
    pulses.push_back(Pulse {
        source: "button".to_string(),
        target: "broadcaster".to_string(),
        is_high: false,
    });

    let mut nb_low = 0;
    let mut nb_high = 0;
    while !pulses.is_empty() {
        let pulse = pulses.pop_front().unwrap();
        if pulse.is_high {
            nb_high += 1;
        } else {
            nb_low += 1;
        }
        if let Some(module) = modules.get_mut(&pulse.target) {
            module.receive_pulse(&pulse, &mut pulses);
        }
    }

    (nb_low, nb_high)
}

pub fn solve1() -> u64 {
    let input = utils::read_input("src/year2023/day20/input.txt").unwrap();
    let mut modules = parse_input(&input);

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let (delta_low, delta_high) = press_button(&mut modules);
        low_count += delta_low;
        high_count += delta_high;
    }

    low_count * high_count
}

pub fn solve2() -> i32 {
    // let input = utils::read_input("src/year2023/day20/input.txt").unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 832957356);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
