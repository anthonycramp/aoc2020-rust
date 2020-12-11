use std::collections::HashMap;

const INPUT: &str = include_str!("../../resources/day07_input.txt");
const SHINY_GOLD: &str = "shiny gold";

fn main() {
    let luggage = Luggage::from(INPUT);

    println!(
        "Day 07 Part 1: {}",
        get_how_many_can_contain(&luggage, SHINY_GOLD)
    );
    println!("Day 07 Part 2: {}", luggage.count_bags(SHINY_GOLD));
}

fn get_how_many_can_contain(luggage: &Luggage, colour: &str) -> usize {
    // flip the script, luggage.luggage is a map from bag (colour)
    // to the list of bags (colour and count) it can contain. We want a map
    // from bag (colour) to bags that it can be contained by.
    let mut reverse: HashMap<String, Vec<String>> = HashMap::new();

    let my_bag = Bag::from(colour);

    for (outer_bag, inner_bags) in &luggage.luggage {
        for inner_bag in inner_bags {
            if let Some(inner) = reverse.get_mut(&inner_bag.bag.colour) {
                (*inner).push(outer_bag.colour.clone());
            } else {
                reverse.insert(inner_bag.bag.colour.clone(), vec![outer_bag.colour.clone()]);
            }
        }
    }

    let mut unvisited: Vec<String> = reverse
        .get(&String::from(colour))
        .expect("not contained in other bags")
        .to_vec();
    let mut visited: HashMap<String, String> = HashMap::new();

    loop {
        if unvisited.is_empty() {
            break;
        }

        let bag_colour = unvisited.pop().expect("nothing to pop!");
        if visited.contains_key(&bag_colour) {
            continue;
        }

        if let Some(next_bags) = reverse.get(&bag_colour) {
            unvisited.extend(next_bags.iter().cloned());
        }

        visited.insert(bag_colour.clone(), String::default());
    }
    visited.len()
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
struct Bag {
    colour: String,
}

impl Bag {
    fn default() -> Self {
        Bag {
            colour: String::default(),
        }
    }
}

impl From<&str> for Bag {
    fn from(item: &str) -> Self {
        if item.ends_with(" bag,")
            || item.ends_with(" bags,")
            || item.ends_with(" bag.")
            || item.ends_with(" bags.")
            || item.ends_with(" bag")
            || item.ends_with(" bags")
        {
            Bag {
                colour: String::from(item.rsplitn(2, " ").collect::<Vec<_>>()[1]),
            }
        } else {
            Bag {
                colour: String::from(item),
            }
        }
    }
}

#[derive(Debug)]
struct BagCount {
    bag: Bag,
    count: usize,
}

impl BagCount {
    fn default() -> Self {
        BagCount {
            bag: Bag::default(),
            count: 0,
        }
    }
}

impl From<&str> for BagCount {
    fn from(item: &str) -> Self {
        if item == "no other bags." {
            BagCount::default()
        } else {
            let fields = item.splitn(2, " ").collect::<Vec<_>>();

            BagCount {
                bag: Bag::from(fields[1]),
                count: fields[0].parse().expect("Couldn't parse bag count"),
            }
        }
    }
}

#[derive(Debug)]
struct Luggage {
    luggage: HashMap<Bag, Vec<BagCount>>,
}

impl Luggage {
    fn default() -> Self {
        Luggage {
            luggage: HashMap::new(),
        }
    }

    fn count_bags(&self, bag_colour: &str) -> usize {
        if let Some(bags) = self.luggage.get(&Bag::from(bag_colour)) {
            let mut sum = 0;
            for bag in bags {
                sum += bag.count + bag.count * self.count_bags(bag.bag.colour.as_str());
            }
            sum
        } else {
            0
        }
    }
}

impl From<&str> for Luggage {
    fn from(item: &str) -> Self {
        let lines = item.lines().map(|l| l.trim()).collect::<Vec<_>>();

        let mut luggage = Luggage::default();

        for line in lines {
            let outer_fields = line.split(" contain ").collect::<Vec<_>>();
            let lhs = Bag::from(outer_fields[0]);
            let rhs = outer_fields[1]
                .split(", ")
                .map(BagCount::from)
                .collect::<Vec<_>>();
            luggage.luggage.insert(lhs, rhs);
        }

        luggage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        let luggage = Luggage::from(input);
        dbg!(&luggage);
        let number_can_contain = get_how_many_can_contain(&luggage, SHINY_GOLD);
        assert_eq!(number_can_contain, 4);
    }

    #[test]
    fn test_parse() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        let luggage = Luggage::from(input);
        assert_eq!(luggage.luggage.len(), 9);
        assert_eq!(luggage.luggage.contains_key(&Bag::from("light red")), true);
        assert_eq!(
            luggage.luggage.contains_key(&Bag::from("dark orange")),
            true
        );
        assert_eq!(
            luggage.luggage.contains_key(&Bag::from("bright white")),
            true
        );
        assert_eq!(
            luggage.luggage.contains_key(&Bag::from("muted yellow")),
            true
        );
        assert_eq!(luggage.luggage.contains_key(&Bag::from("shiny gold")), true);
        assert_eq!(luggage.luggage.contains_key(&Bag::from("dark olive")), true);
        assert_eq!(
            luggage.luggage.contains_key(&Bag::from("vibrant plum")),
            true
        );
        assert_eq!(luggage.luggage.contains_key(&Bag::from("faded blue")), true);
        assert_eq!(
            luggage.luggage.contains_key(&Bag::from("dotted black")),
            true
        );

        assert_eq!(
            luggage.luggage.get(&Bag::from(SHINY_GOLD)).unwrap().len(),
            2
        );
    }

    #[test]
    fn test2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        let luggage = Luggage::from(input);
        let bag_count = luggage.count_bags(SHINY_GOLD);
        assert_eq!(bag_count, 32);
    }

    #[test]
    fn test3() {
        let input = "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.";

        let luggage = Luggage::from(input);
        let bag_count = luggage.count_bags(SHINY_GOLD);
        assert_eq!(bag_count, 126);
    }
}
