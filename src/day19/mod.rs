use crate::utils::{self, parse_int};
use nom::{
    bytes::complete::tag,
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Blueprint {
    index: i32,
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_cost: (i32, i32),
    geode_robot_cost: (i32, i32),
    max_ore_cost: i32,
}

impl Blueprint {
    fn new(
        index: i32,
        ore_robot_cost: i32,
        clay_robot_cost: i32,
        obsidian_robot_cost: (i32, i32),
        geode_robot_cost: (i32, i32),
    ) -> Blueprint {
        let max_ore_cost = clay_robot_cost
            .max(obsidian_robot_cost.0)
            .max(geode_robot_cost.0);
        Blueprint {
            index,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
            max_ore_cost,
        }
    }
    fn get_nb_geodes_aux(
        &self,
        time: i32,
        nb_ore: i32,
        nb_ore_robots: i32,
        nb_clay: i32,
        nb_clay_robots: i32,
        nb_obsidian: i32,
        nb_obsidian_robots: i32,
        nb_geode: i32,
        nb_geode_robots: i32,
    ) -> i32 {
        if time < 0 {
            return 0;
        }

        let mut result = nb_geode + time * nb_geode_robots;

        // make an ore robot
        if nb_ore_robots < self.max_ore_cost {
            let time_for_robot =
                utils::div_up(self.ore_robot_cost - nb_ore, nb_ore_robots).max(0) + 1;
            if time_for_robot <= time {
                result = result.max(self.get_nb_geodes_aux(
                    time - time_for_robot,
                    nb_ore + time_for_robot * nb_ore_robots - self.ore_robot_cost,
                    nb_ore_robots + 1,
                    nb_clay + time_for_robot * nb_clay_robots,
                    nb_clay_robots,
                    nb_obsidian + time_for_robot * nb_obsidian_robots,
                    nb_obsidian_robots,
                    nb_geode + time_for_robot * nb_geode_robots,
                    nb_geode_robots,
                ));
            }
        }

        // make a clay robot
        if nb_clay_robots < self.obsidian_robot_cost.1 {
            let time_for_robot =
                utils::div_up(self.clay_robot_cost - nb_ore, nb_ore_robots).max(0) + 1;
            if time_for_robot <= time {
                result = result.max(self.get_nb_geodes_aux(
                    time - time_for_robot,
                    nb_ore + time_for_robot * nb_ore_robots - self.clay_robot_cost,
                    nb_ore_robots,
                    nb_clay + time_for_robot * nb_clay_robots,
                    nb_clay_robots + 1,
                    nb_obsidian + time_for_robot * nb_obsidian_robots,
                    nb_obsidian_robots,
                    nb_geode + time_for_robot * nb_geode_robots,
                    nb_geode_robots,
                ));
            }
        }

        // make an obsidian robot
        if nb_clay_robots > 0 && nb_obsidian_robots < self.geode_robot_cost.1 {
            let time_for_robot = utils::div_up(self.obsidian_robot_cost.0 - nb_ore, nb_ore_robots)
                .max(utils::div_up(
                    self.obsidian_robot_cost.1 - nb_clay,
                    nb_clay_robots,
                ))
                .max(0)
                + 1;
            if time_for_robot <= time {
                result = result.max(self.get_nb_geodes_aux(
                    time - time_for_robot,
                    nb_ore + time_for_robot * nb_ore_robots - self.obsidian_robot_cost.0,
                    nb_ore_robots,
                    nb_clay + time_for_robot * nb_clay_robots - self.obsidian_robot_cost.1,
                    nb_clay_robots,
                    nb_obsidian + time_for_robot * nb_obsidian_robots,
                    nb_obsidian_robots + 1,
                    nb_geode + time_for_robot * nb_geode_robots,
                    nb_geode_robots,
                ));
            }
        }

        // make a geode robot
        if nb_obsidian_robots > 0 {
            let time_for_robot = utils::div_up(self.geode_robot_cost.0 - nb_ore, nb_ore_robots)
                .max(utils::div_up(
                    self.geode_robot_cost.1 - nb_obsidian,
                    nb_obsidian_robots,
                ))
                .max(0)
                + 1;
            if time_for_robot <= time {
                result = result.max(self.get_nb_geodes_aux(
                    time - time_for_robot,
                    nb_ore + time_for_robot * nb_ore_robots - self.geode_robot_cost.0,
                    nb_ore_robots,
                    nb_clay + time_for_robot * nb_clay_robots,
                    nb_clay_robots,
                    nb_obsidian + time_for_robot * nb_obsidian_robots - self.geode_robot_cost.1,
                    nb_obsidian_robots,
                    nb_geode + time_for_robot * nb_geode_robots,
                    nb_geode_robots + 1,
                ));
            }
        }

        result
    }

    fn get_nb_geodes(&self, time: i32) -> i32 {
        self.get_nb_geodes_aux(time, 0, 1, 0, 0, 0, 0, 0, 0)
    }
}

fn parse_header(input: &str) -> IResult<&str, i32> {
    delimited(tag("Blueprint "), parse_int, tag(": "))(input)
}

fn parse_ore_robot_cost(input: &str) -> IResult<&str, i32> {
    delimited(tag("Each ore robot costs "), parse_int, tag(" ore. "))(input)
}

fn parse_clay_robot_cost(input: &str) -> IResult<&str, i32> {
    delimited(tag("Each clay robot costs "), parse_int, tag(" ore. "))(input)
}

fn parse_obsidian_robot_cost(input: &str) -> IResult<&str, (i32, i32)> {
    pair(
        delimited(
            tag("Each obsidian robot costs "),
            parse_int,
            tag(" ore and "),
        ),
        terminated(parse_int, tag(" clay. ")),
    )(input)
}

fn parse_geode_robot_cost(input: &str) -> IResult<&str, (i32, i32)> {
    pair(
        delimited(tag("Each geode robot costs "), parse_int, tag(" ore and ")),
        terminated(parse_int, tag(" obsidian.")),
    )(input)
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (remaining, (index, c1, c2, (c3, c4), (c5, c6))) = tuple((
        parse_header,
        parse_ore_robot_cost,
        parse_clay_robot_cost,
        parse_obsidian_robot_cost,
        parse_geode_robot_cost,
    ))(input)?;

    Ok((remaining, Blueprint::new(index, c1, c2, (c3, c4), (c5, c6))))
}

fn make_blueprints(filename: &str) -> Vec<Blueprint> {
    let input = utils::read_input(filename).unwrap();
    let mut blueprints = Vec::new();
    for line in input.lines() {
        let (_, blueprint) = parse_blueprint(line).unwrap();
        blueprints.push(blueprint);
    }
    blueprints
}

pub fn solve1() -> i32 {
    let blueprints = make_blueprints("src/day19/input.txt");
    let mut total = 0;
    for blueprint in blueprints.iter() {
        println!("blueprint: {:?}", blueprint);
        let nb_geodes = blueprint.get_nb_geodes(24);
        println!("nb geodes: {}", nb_geodes);
        total += blueprint.index * nb_geodes;
    }

    total
}

pub fn solve2() -> i32 {
    let blueprints = make_blueprints("src/day19/input.txt");
    let mut total = 1;

    for blueprint in &blueprints[0..3] {
        println!("blueprint: {:?}", blueprint);
        let nb_geodes = blueprint.get_nb_geodes(32);
        println!("nb geodes: {}", nb_geodes);
        total *= nb_geodes;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 2160);
    }

    #[test]
    #[ignore = "long test"]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 13340);
    }
}
