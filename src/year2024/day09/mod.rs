use crate::utils;

struct Block {
    id: i32,
    address: u32,
    size: u32,
}

fn get_checksum(files: Vec<Block>) -> u64 {
    let mut total: u64 = 0;
    for file in files {
        total += (
            (file.id as u32) * 
            (file.size * file.address + (file.size * (file.size - 1)) / 2)
        ) as u64;
    }
    total
}

fn parse_input(input: &str) -> (Vec<Block>, Vec<Block>) {
    let mut files: Vec<Block> = Vec::new();
    let mut free_blocks = Vec::new();

    let mut is_file = true;
    let mut id = 0;
    let mut address = 0;
    for c in input.chars() {
        let size = c.to_digit(10).unwrap();
        if is_file {
            // file
            if size > 0 {
                files.push(Block {
                    id,
                    address,
                    size,
                });
            }
            id += 1;
        } else {
            // free space
            if size > 0 {
                free_blocks.push(Block {
                    id: -1,
                    address,
                    size,
                });
            }
        }
        address += size;
        is_file = !is_file;
    }
    (files, free_blocks)
}

pub fn solve1() -> u64 {
    let input = utils::read_input("src/year2024/day09/input.txt").unwrap();
    let (mut files, mut free_space) = parse_input(&input);
    files.reverse();

    let mut i = 0;  // index for files
    let mut j = 0;  // index for free_space

    while files[i].address > free_space[j].address {
        if files[i].size < free_space[j].size {
            files[i].address = free_space[j].address;
            free_space[j].address += files[i].size;
            free_space[j].size -= files[i].size;
            i += 1;
        } else if files[i].size == free_space[j].size {
            files[i].address = free_space[j].address;
            free_space[j].size = 0;
            i += 1;
            j += 1;
        } else {
            files.push(Block {
                id: files[i].id,
                address: free_space[j].address,
                size: free_space[j].size,
            });
            files[i].size = files[i].size - free_space[j].size;
            free_space[j].size = 0;
            j += 1;
        }
    }
    get_checksum(files)
}

pub fn solve2() -> u64 {
    let input = utils::read_input("src/year2024/day09/input.txt").unwrap();
    let (mut files, mut free_blocks) = parse_input(&input);
    files.reverse();

    for file in &mut files {
        for free_block in &mut free_blocks {
            if free_block.address >= file.address {
                break;
            }
            if free_block.size >= file.size {
                file.address = free_block.address;
                free_block.size -= file.size;
                free_block.address += file.size;
                break;
            }
        }
    }
    get_checksum(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 6399153661894);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 6421724645083);
    }
}
