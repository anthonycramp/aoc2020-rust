fn main() {
    let input = aoc2020::get_input_string_from_file().unwrap();

    let tree_count_part1 = run_part1(&input);
    println!("Day 03 Part 1: {}", tree_count_part1);
    let tree_count_part2 = run_part2(&input);
    println!("Day 03 Part 2: {}", tree_count_part2);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Location {
    Open,
    Tree,
}

impl From<char> for Location {
    fn from(item: char) -> Self {
        match item {
            '.' => Location::Open,
            '#' => Location::Tree,
            _ => panic!("Unknown location char"),
        }
    }
}

struct Area {
    area: Vec<Vec<Location>>,
}

impl From<&str> for Area {
    fn from(item: &str) -> Self {
        let mut ret = vec![];
        for line in item.lines() {
            ret.push(line.trim().chars().map(Location::from).collect());
        }

        Area { area: ret }
    }
}

impl Area {
    fn height(&self) -> usize {
        self.area.len()
    }

    fn width(&self) -> usize {
        self.area[0].len()
    }

    fn at(&self, row: usize, col: usize) -> Location {
        self.area[row][col]
    }
}

fn trees_hit_while_traversing_area(area: &Area, slope: (usize, usize)) -> usize {
    let mut location = (0, 0);
    let area_height = area.height();
    let area_width = area.width();

    let mut tree_count = 0;
    while location.0 < area_height {
        if area.at(location.0, location.1) == Location::Tree {
            tree_count += 1;
        }

        location.0 += slope.0;
        location.1 = (location.1 + slope.1) % area_width;
    }

    tree_count
}

fn run_part1(map: &str) -> usize {
    let area = Area::from(map);

    trees_hit_while_traversing_area(&area, (1, 3))
}

fn run_part2(map: &str) -> usize {
    let area = Area::from(map);
    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let trees_hit: Vec<usize> = slopes
        .iter()
        .map(|&s| trees_hit_while_traversing_area(&area, s))
        .collect();
    trees_hit.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        let area = Area::from(input);
        assert_eq!(area.height(), 11);
        assert_eq!(area.width(), 11);
        assert_eq!(area.at(0, 0), Location::Open);
        assert_eq!(area.at(0, 2), Location::Tree);
    }

    #[test]
    fn test_run_part1() {
        let input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        let num_trees = run_part1(&input);
        assert_eq!(num_trees, 7);
    }

    #[test]
    fn test_run_part2() {
        let input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        let prod = run_part2(&input);
        assert_eq!(prod, 336);
    }
}
