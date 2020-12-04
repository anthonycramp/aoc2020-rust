fn main() {
    println!("Hello, world!");
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

    fn at(&self, row: usize, col: usize) -> Location {
        self.area[row][col]
    }
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
        assert_eq!(area.at(0, 0), Location::Open);
        assert_eq!(area.at(0, 2), Location::Tree);
    }
}
