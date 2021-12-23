use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Amphi {
    A,
    B,
    C,
    D,
    E,
}

impl Default for Amphi {
    fn default() -> Self {
        Self::E
    }
}

impl Amphi {
    fn new(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            _ => panic!("Invalid char for Amphi: {:?}", c),
        }
    }

    fn cost(self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
            Self::E => unreachable!(),
        }
    }

    fn goal_idx(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
            Self::E => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Pods {
    half: [Amphi; 7],
    rooms: [[Amphi; 4]; 4],
}

impl Pods {
    fn from_stdin() -> Self {
        let mut s = Self::default();
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines().map(Result::unwrap).skip(2);
        for j in [3, 0] {
            for (i, a) in lines
                .next()
                .unwrap()
                .chars()
                .filter(|c| matches!(c, 'A' | 'B' | 'C' | 'D'))
                .map(Amphi::new)
                .enumerate()
            {
                s.rooms[i][j] = a;
            }
        }

        use Amphi::*;
        s.set_row(2, [D, C, B, A]);
        s.set_row(1, [D, B, A, C]);

        s
    }

    fn set_row(&mut self, idx: usize, row: [Amphi; 4]) {
        for (i, a) in row.into_iter().enumerate() {
            self.rooms[i][idx] = a;
        }
    }

    fn is_done(&self) -> bool {
        use Amphi::*;
        self.rooms[0] == [A; 4] && self.rooms[1] == [B; 4] && self.rooms[2] == [C; 4] && self.rooms[3] == [D; 4]
    }

    fn move_cost(&self, hidx: usize, ridx: usize) -> Option<usize> {
        let leftside = hidx <= ridx + 1;
        let range = if leftside {
            hidx + 1..=ridx + 1
        } else {
            ridx + 2..=hidx - 1
        };
        range.into_iter().all(|i| self.half[i] == Amphi::E).then(|| {
            let mut cost = if leftside {
                1 + (ridx + 1 - hidx) * 2
            } else {
                1 + (hidx - ridx - 2) * 2
            };
            if hidx == 0 || hidx == self.half.len() - 1 {
                cost -= 1;
            }
            cost
        })
    }

    fn push(&self, hidx: usize, ridx: usize) -> Option<(Self, usize)> {
        assert_ne!(self.half[hidx], Amphi::E);
        let move_cost = self.move_cost(hidx, ridx)?;
        let r = self.rooms[ridx];
        let i = r.iter().position(|a| *a == Amphi::E)?;
        let a = self.half[hidx];

        assert_eq!(a.goal_idx(), ridx);
        if r.iter().any(|x| *x != a && *x != Amphi::E) {
            return None;
        }

        let mut p = self.clone();
        p.rooms[ridx][i] = a;
        p.half[hidx] = Amphi::E;
        let push_cost = r.len() - i;
        Some((p, (move_cost + push_cost) * a.cost()))
    }

    fn pop(&self, hidx: usize, ridx: usize) -> Option<(Self, usize)> {
        assert_eq!(self.half[hidx], Amphi::E);

        let move_cost = self.move_cost(hidx, ridx)?;
        let r = self.rooms[ridx];
        let (i, a) = r.into_iter().enumerate().rev().find(|(_, a)| *a != Amphi::E)?;

        if a.goal_idx() == ridx && r[..i].iter().all(|x| *x == a) {
            return None;
        }

        let mut p = self.clone();
        p.half[hidx] = a;
        p.rooms[ridx][i] = Amphi::E;
        let pop_cost = r.len() - i;
        Some((p, (move_cost + pop_cost) * a.cost()))
    }
}

struct State(Pods, usize);

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut queue = BinaryHeap::new();
    let init = Pods::from_stdin();
    queue.push(State(init.clone(), 0));

    let mut costs = HashMap::new();
    costs.insert(init, 0usize);

    let cost = loop {
        let State(p, cost) = queue.pop().unwrap();
        if p.is_done() {
            break cost;
        }

        if cost > costs[&p] {
            continue;
        }

        for (i, h) in p.half.into_iter().enumerate() {
            if h == Amphi::E {
                for (p, c) in (0..p.rooms.len()).into_iter().filter_map(|j| p.pop(i, j)) {
                    let cost = cost + c;
                    if let Some(c) = costs.get(&p) {
                        if cost >= *c {
                            continue;
                        }
                    }
                    costs.insert(p.clone(), cost);
                    queue.push(State(p, cost));
                }
            } else if let Some((p, c)) = p.push(i, h.goal_idx()) {
                let cost = cost + c;
                if let Some(c) = costs.get(&p) {
                    if cost >= *c {
                        continue;
                    }
                }
                costs.insert(p.clone(), cost);
                queue.push(State(p, cost));
            }
        }
    };

    println!("{}", cost);
}
