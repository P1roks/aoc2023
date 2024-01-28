use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    hash::{Hash, Hasher},
};

use itertools::Itertools;
use ModuleType::*;

#[derive(PartialEq, Clone, Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction,
    Broadcaster,
    Final,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}

impl From<Pulse> for bool {
    fn from(val: Pulse) -> Self {
        match val {
            Pulse::Low => false,
            Pulse::High => true,
        }
    }
}

impl From<bool> for Pulse {
    fn from(val: bool) -> Self {
        match val {
            true => Pulse::High,
            false => Pulse::Low,
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    r#type: ModuleType,
    child_idxs: Vec<usize>,
    parent_states: Option<HashMap<usize, Pulse>>,
}

impl Module {
    fn new(r#type: ModuleType) -> Self {
        let parent_states = if r#type == ModuleType::Conjunction {
            Some(HashMap::new())
        } else {
            None
        };
        Self {
            r#type,
            parent_states,
            child_idxs: Vec::new(),
        }
    }

    fn with_idxs(r#type: ModuleType, child_idxs: Vec<usize>) -> Self {
        let parent_states = if r#type == ModuleType::Conjunction {
            Some(HashMap::new())
        } else {
            None
        };

        Self {
            r#type,
            parent_states,
            child_idxs,
        }
    }
}

type ReceiveArgs = (usize, usize, Pulse);

#[derive(Debug, Clone)]
struct Machine {
    modules: Vec<Module>,
    send_queue: VecDeque<ReceiveArgs>,
    low: usize,
    high: usize,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Machine {
    fn receive_pulse(&mut self, receiver: usize, sender: usize, pulse: Pulse) {
        let Self {
            modules,
            send_queue,
            low,
            high,
        } = self;

        match pulse {
            Pulse::Low => *low += 1,
            Pulse::High => *high += 1,
        }

        let new_pulse = {
            let module = &mut modules[receiver];
            match module.r#type {
                Broadcaster => Pulse::Low,
                FlipFlop(state) if pulse == Pulse::Low => {
                    let new_state = !state;
                    module.r#type = ModuleType::FlipFlop(new_state);
                    new_state.into()
                }
                Conjunction => {
                    module
                        .parent_states
                        .as_mut()
                        .unwrap()
                        .entry(sender)
                        .and_modify(|e| *e = pulse);
                    if module
                        .parent_states
                        .as_ref()
                        .is_some_and(|states| states.values().all(|val| *val == Pulse::High))
                    {
                        Pulse::Low
                    } else {
                        Pulse::High
                    }
                }
                _ => {
                    return;
                }
            }
        };

        for idx in modules[receiver].child_idxs.iter() {
            send_queue.push_back((*idx, receiver, new_pulse));
        }
    }

    fn solve_part1(mut self) -> usize {
        for _ in 0..1000 {
            self.receive_pulse(0, 0, Pulse::Low);
            while let Some((next_idx, sender, next_pulse)) = self.send_queue.pop_front() {
                self.receive_pulse(next_idx, sender, next_pulse);
            }
        }

        self.low * self.high
    }

    fn find_cycle(mut self, module_idx: usize) -> usize {
        let mut cnt = 1;
        loop {
            self.receive_pulse(0, 0, Pulse::Low);
            while let Some((next_idx, sender, next_pulse)) = self.send_queue.pop_front() {
                self.receive_pulse(next_idx, sender, next_pulse);
                if sender == module_idx && next_pulse == Pulse::High {
                    return cnt;
                }
            }
            cnt += 1;
        }
    }

    fn solve_part2(self) -> usize {
        // conjunction module which points to rx
        let (second_last_idx, _) = self
            .modules
            .iter()
            .find_position(|module| module.child_idxs.contains(&(self.modules.len() - 1)))
            .unwrap();

        let mut mod_idxs = self
            .modules
            .iter()
            .enumerate()
            .filter(|(_, module)| module.child_idxs.contains(&second_last_idx))
            .map(|(idx, _)| idx)
            .collect_vec();

        mod_idxs
            .drain(..)
            .map(|idx| self.clone().find_cycle(idx))
            .fold(1, num::integer::lcm)
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut next_idx = 0..usize::MAX;
        let mut name_idx_map = HashMap::<u64, usize>::new();
        let mut conj_parent_idxs = HashMap::<usize, Vec<usize>>::new();

        // line number + rx module
        let module_no = bytes.iter().filter(|byte| **byte == b'\n').count() + 1;
        let mut modules = Vec::<Module>::with_capacity(module_no);
        #[allow(clippy::uninit_vec)]
        unsafe {
            modules.set_len(module_no);
        }
        name_idx_map.insert(calculate_hash(b"rx"), module_no - 1);
        modules[module_no - 1] = Module::new(ModuleType::Final);

        let mut get_or_insert_idx = |name: &[u8]| -> usize {
            let hash = calculate_hash(&name);
            if let Some(entry) = name_idx_map.get(&hash) {
                *entry
            } else {
                let new_idx = next_idx.next().unwrap();
                name_idx_map.insert(hash, new_idx);
                new_idx
            }
        };

        let mut in_out = bytes
            .split(|byte| *byte == b'\n')
            .map(|line| {
                let mut it = line.split(|byte| *byte == b' ');
                let module = it.next().unwrap();
                // throw out ->
                it.next();
                let out = it
                    .map(|mod_name| {
                        // filter out colons
                        if mod_name.contains(&b',') {
                            mod_name.split_last().unwrap().1
                        } else {
                            mod_name
                        }
                    })
                    .collect_vec();
                (module, out)
            })
            .collect_vec();

        if let Some(broad_idx) = in_out
            .iter()
            .position(|(module, _)| module == b"broadcaster")
        {
            let broad = in_out.swap_remove(broad_idx);
            let idx = get_or_insert_idx(broad.0);
            let child_idxs = broad
                .1
                .iter()
                .map(|name| get_or_insert_idx(name))
                .collect_vec();
            modules[idx] = Module::with_idxs(ModuleType::Broadcaster, child_idxs);
        }
        for (module, out) in in_out.iter().filter(|(module, _)| !module.is_empty()) {
            let r#type = match module[0] {
                b'&' => ModuleType::Conjunction,
                b'%' => ModuleType::FlipFlop(false),
                _ => panic!(""),
            };
            let idx = get_or_insert_idx(&module[1..]);
            // println!("{:?} -> {idx}", String::from_utf8_lossy(&module[1..]));
            let child_idxs = out.iter().map(|name| get_or_insert_idx(name)).collect_vec();
            modules[idx] = Module::with_idxs(r#type, child_idxs);
        }

        // O(n^2) but w/e since this is one-time parse
        for (idx, _) in modules
            .iter()
            .enumerate()
            .filter(|(_, module)| module.r#type == ModuleType::Conjunction)
        {
            let parent_idxs = modules
                .iter()
                .enumerate()
                .filter(|(_, module)| module.child_idxs.contains(&idx))
                .map(|(parent_idx, _)| parent_idx)
                .collect_vec();
            conj_parent_idxs.insert(idx, parent_idxs);
        }

        for (idx, parent_idxs) in conj_parent_idxs.drain() {
            modules[idx].parent_states =
                Some(parent_idxs.iter().map(|idx| (*idx, Pulse::Low)).collect());
        }

        Self {
            modules,
            low: 0,
            high: 0,
            send_queue: VecDeque::new(),
        }
    }
}

pub fn main() {
    let machine = Machine::from_bytes(include_bytes!("../../input/day20"));
    let p1 = machine.clone().solve_part1();
    println!("part 1: {p1}");

    let p2 = machine.clone().solve_part2();
    println!("part 2: {p2}");
}
