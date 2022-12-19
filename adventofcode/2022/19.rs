use fxhash::FxHashMap;
use rayon::prelude::*;
use std::env;
use std::io::{self, BufRead};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct State {
    remain_min: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}
impl State {
    fn new(remain_min: u32) -> Self {
        Self {
            remain_min,
            ore_robots: 1,
            ..Default::default()
        }
    }
    fn next_minute(&self) -> Self {
        let mut s = self.clone();
        s.remain_min -= 1;
        s.ore += s.ore_robots;
        s.clay += s.clay_robots;
        s.obsidian += s.obsidian_robots;
        s.geode += s.geode_robots;
        s
    }
}

struct OreRobot {
    ore: u32,
}
impl OreRobot {
    fn build(&self, s: &State) -> Option<State> {
        if s.ore < self.ore {
            return None;
        }
        let mut s = s.next_minute();
        s.ore -= self.ore;
        s.ore_robots += 1;
        Some(s)
    }
}
struct ClayRobot {
    ore: u32,
}
impl ClayRobot {
    fn build(&self, s: &State) -> Option<State> {
        if s.ore < self.ore {
            return None;
        }
        let mut s = s.next_minute();
        s.ore -= self.ore;
        s.clay_robots += 1;
        Some(s)
    }
}
struct ObsidianRobot {
    ore: u32,
    clay: u32,
}
impl ObsidianRobot {
    fn build(&self, s: &State) -> Option<State> {
        if s.ore < self.ore || s.clay < self.clay {
            return None;
        }
        let mut s = s.next_minute();
        s.ore -= self.ore;
        s.clay -= self.clay;
        s.obsidian_robots += 1;
        Some(s)
    }
}
struct GeodeRobot {
    ore: u32,
    obsidian: u32,
}
impl GeodeRobot {
    fn build(&self, s: &State) -> Option<State> {
        if s.ore < self.ore || s.obsidian < self.obsidian {
            return None;
        }
        let mut s = s.next_minute();
        s.ore -= self.ore;
        s.obsidian -= self.obsidian;
        s.geode_robots += 1;
        Some(s)
    }
}

struct Blueprint {
    id: u32,
    ore: OreRobot,
    clay: ClayRobot,
    obsidian: ObsidianRobot,
    geode: GeodeRobot,
    max_ore: u32,
}
impl Blueprint {
    fn parse(mut s: String) -> Self {
        s.retain(|c| c.is_ascii_digit() || c.is_ascii_whitespace());
        let mut s = s.split_ascii_whitespace().map(|s| s.parse().unwrap());
        let id = s.next().unwrap();
        let ore = OreRobot {
            ore: s.next().unwrap(),
        };
        let clay = ClayRobot {
            ore: s.next().unwrap(),
        };
        let obsidian = ObsidianRobot {
            ore: s.next().unwrap(),
            clay: s.next().unwrap(),
        };
        let geode = GeodeRobot {
            ore: s.next().unwrap(),
            obsidian: s.next().unwrap(),
        };
        let max_ore = [ore.ore, clay.ore, geode.ore].into_iter().max().unwrap();
        Self {
            id,
            ore,
            clay,
            obsidian,
            geode,
            max_ore,
        }
    }
}

#[derive(Default)]
struct Memo {
    results: FxHashMap<State, u32>,
    max: u32,
}

fn max_geodes(state: State, b: &Blueprint, memo: &mut Memo) -> u32 {
    if state.remain_min == 0 {
        memo.max = memo.max.max(state.geode);
        return state.geode;
    }

    if let Some(&ret) = memo.results.get(&state) {
        return ret;
    }
    let possible_max_geode =
        state.geode + (state.geode_robots * 2 + state.remain_min - 1) * state.remain_min / 2; // n + n+1 + ... + n+m = (n + n + m - 1) * m / 2
    if possible_max_geode <= memo.max && memo.max > 0 {
        memo.results.insert(state, 0);
        return 0; // This branch cannot update max value
    }

    let max = [
        b.geode.build(&state),
        if state.ore_robots < b.max_ore {
            b.ore.build(&state)
        } else {
            None
        },
        if state.obsidian_robots < b.geode.obsidian {
            b.obsidian.build(&state)
        } else {
            None
        },
        if state.clay_robots < b.obsidian.clay {
            b.clay.build(&state)
        } else {
            None
        },
        Some(state.next_minute()), // Build nothing
    ]
    .into_iter()
    .flatten()
    .map(|s| max_geodes(s, b, memo))
    .max()
    .unwrap();

    memo.results.insert(state, max);
    max
}

fn part1(lines: impl Iterator<Item = String>) {
    let answer = lines
        .map(|line| {
            let b = Blueprint::parse(line);
            let max = max_geodes(State::new(24), &b, &mut Memo::default());
            b.id * max
        })
        .sum::<u32>();
    println!("{}", answer);
}

fn part2(lines: impl Iterator<Item = String>) {
    let lines = lines.take(3).collect::<Vec<_>>();
    let answer = lines
        .into_par_iter()
        .map(|line| {
            max_geodes(
                State::new(32),
                &Blueprint::parse(line),
                &mut Memo::default(),
            )
        })
        .product::<u32>();
    println!("{}", answer);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
