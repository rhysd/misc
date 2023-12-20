use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

enum Mod<'a> {
    Broad,
    Flip(bool),
    Conj(HashMap<&'a str, Pulse>),
}

type Modules<'a> = HashMap<&'a str, Mod<'a>>;
type Conns<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(lines: &[String]) -> (Modules<'_>, Conns<'_>) {
    let mut mods = HashMap::new();
    let mut conns = HashMap::<_, Vec<_>>::new();

    for line in lines.iter() {
        let (l, r) = line.split_once(" -> ").unwrap();
        let dests = r.split(", ").collect::<Vec<_>>();
        let (name, module) = match l {
            n @ "broadcaster" => (n, Mod::Broad),
            n if n.starts_with('%') => (&n[1..], Mod::Flip(false)),
            n if n.starts_with('&') => (&n[1..], Mod::Conj(HashMap::new())),
            _ => unreachable!(),
        };
        mods.insert(name, module);
        if let Some(v) = conns.get_mut(name) {
            v.extend_from_slice(&dests);
        } else {
            conns.insert(name, dests);
        }
    }

    for (n, m) in mods.iter_mut() {
        if let Mod::Conj(src) = m {
            src.extend(
                conns
                    .iter()
                    .filter(|(_, dest)| dest.iter().any(|d| d == n))
                    .map(|(&src, _)| (src, Pulse::Low)),
            )
        }
    }

    (mods, conns)
}

fn simulate_button_press<'a, F: FnMut(&'a str, &'a str, Pulse)>(
    mods: &mut Modules<'a>,
    conns: &Conns<'a>,
    mut on_tick: F,
) {
    let mut queue = VecDeque::from_iter(
        conns["broadcaster"]
            .iter()
            .map(|&n| ("broadcaster", n, Pulse::Low)),
    );
    while let Some((tx, rx, pulse)) = queue.pop_front() {
        on_tick(tx, rx, pulse);

        let Some(module) = mods.get_mut(rx) else {
            continue;
        };

        let dests = conns.get(rx).unwrap();
        let next_pulse = match module {
            Mod::Broad => pulse,
            Mod::Flip(_) if pulse == Pulse::High => continue,
            Mod::Flip(is_on) => {
                let p = if *is_on { Pulse::Low } else { Pulse::High };
                *is_on = !*is_on;
                p
            }
            Mod::Conj(src) => {
                if let Some(p) = src.get_mut(tx) {
                    *p = pulse;
                }
                if src.values().all(|&p| p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
        };
        queue.extend(dests.iter().map(|&d| (rx, d, next_pulse)));
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let lines = lines.collect::<Vec<_>>();
    let (mut mods, conns) = parse(&lines);

    let mut lows = 0; // button sends low pulse
    let mut highs = 0;

    for _ in 0..1000 {
        lows += 1; // `button` module sends low pulse
        simulate_button_press(&mut mods, &conns, |_, _, p| match p {
            Pulse::High => highs += 1,
            Pulse::Low => lows += 1,
        });
    }

    println!("{}", highs * lows);
}

fn lcm(a: usize, b: usize) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    let d = gcd(a, b);
    a / d * b
}

fn part2(lines: impl Iterator<Item = String>) {
    let lines = lines.collect::<Vec<_>>();
    let (mut mods, conns) = parse(&lines);

    // rx is connected from the single conjunction df. When rx receives LOW, ln and xp and gp and xl
    // send HIGH. After some investigations, I understood each of them have cycles on sending HIGH.
    //
    //   ...-->HIGH-->LOW
    //    .      .     .
    //    .      .     .
    //    .      .     .
    // ...-+     .     .
    // ...-+-&ln-+     .
    // ...-+     |     .
    //           |     .
    // ...-+     |     .
    // ...-+-&xp-+     .
    // ...-+     |     .
    //           +-&df-+-&rx
    // ...-+     |
    // ...-+-&gp-+
    // ...-+     |
    //           |
    // ...-+     |
    // ...-+-&xl-+
    // ...-+

    let (&to_rx, _) = conns.iter().find(|(_, d)| d.contains(&"rx")).unwrap();
    // Check assumed pre-conditions
    assert_eq!(conns.iter().filter(|(_, d)| d.contains(&"rx")).count(), 1);
    let to_rx_src_len = if let Mod::Conj(src) = &mods[to_rx] {
        src.len()
    } else {
        unreachable!()
    };

    let mut iters_on_high = HashMap::new();
    for iters in 1usize.. {
        simulate_button_press(&mut mods, &conns, |tx, rx, p| {
            if p == Pulse::High && rx == to_rx {
                iters_on_high.entry(tx).or_insert(iters);
            }
        });
        if iters_on_high.len() == to_rx_src_len {
            break;
        }
    }

    println!("{}", iters_on_high.into_values().reduce(lcm).unwrap());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
