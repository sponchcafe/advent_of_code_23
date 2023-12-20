#![allow(unused)]

use crate::util::load_file;
use anyhow::Error;

pub fn puzzle_20_1() -> u64 {
    let input = load_file("20/input.txt");
    let mut machines = parse_machines(&input).unwrap();
    total_pulses(&mut machines, 1000) as u64
}

type Key = String;
type Level = bool;
const LOW: Level = true;
const HIGH: Level = false;

#[derive(Debug, PartialEq, Eq)]
struct Pulse {
    level: Level,
    source: Key,
    target: Key,
}

impl Pulse {
    fn low<K>(source: K, target: K) -> Pulse
    where
        K: AsRef<str>,
    {
        Pulse {
            level: LOW,
            source: source.as_ref().into(),
            target: target.as_ref().into(),
        }
    }
    fn high<K>(source: K, target: K) -> Pulse
    where
        K: AsRef<str>,
    {
        Pulse {
            level: HIGH,
            source: source.as_ref().into(),
            target: target.as_ref().into(),
        }
    }
}

trait Machine {
    fn write_pulse(&mut self, pulse: &Pulse);

    fn read_pulse(&mut self) -> Vec<Pulse> {
        if let Some(level) = self.read_level() {
            self.get_outputs()
                .iter()
                .map(|o| Pulse {
                    level,
                    source: self.get_key(),
                    target: o.clone(),
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn get_outputs(&self) -> Vec<Key>;

    fn read_level(&mut self) -> Option<Level>;

    fn get_key(&self) -> Key;
}

struct Machines {
    index: std::collections::HashMap<Key, usize>,
    machines: Vec<Box<dyn Machine>>,
}

impl Machines {
    fn new() -> Machines {
        Machines {
            index: std::collections::HashMap::new(),
            machines: vec![],
        }
    }
    fn insert(&mut self, key: Key, machine: Box<dyn Machine>) {
        self.machines.push(machine);
        self.index.insert(key, self.machines.len() - 1);
    }
    fn len(&self) -> usize {
        self.index.len()
    }
    fn get_mut(&mut self, key: &Key) -> Option<&mut Box<dyn Machine>> {
        let idx = self.index.get(key)?;
        Some(&mut self.machines[*idx])
    }
    fn values_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Machine>> {
        self.machines.iter_mut()
    }
}

// Broadcaster

#[derive(Debug)]
struct Broadcaster {
    key: Key,
    level: Option<Level>,
    outputs: Vec<Key>,
}

impl Broadcaster {
    fn new<S>(key: S, outputs: &[S]) -> Broadcaster
    where
        S: AsRef<str>,
    {
        let outputs = outputs
            .into_iter()
            .map(|o| o.as_ref().to_string())
            .collect();
        Broadcaster {
            key: key.as_ref().to_string(),
            level: None,
            outputs,
        }
    }
}

impl Machine for Broadcaster {
    fn write_pulse(&mut self, pulse: &Pulse) {
        self.level = Some(pulse.level);
    }

    fn get_outputs(&self) -> Vec<Key> {
        self.outputs.iter().cloned().collect()
    }

    fn read_level(&mut self) -> Option<Level> {
        let l = self.level;
        self.level = None;
        l
    }

    fn get_key(&self) -> Key {
        self.key.clone()
    }
}

// FlipFlop
#[derive(Debug)]
struct FlipFlop {
    key: Key,
    state: Level,
    active: bool,
    outputs: Vec<Key>,
}

impl FlipFlop {
    fn new<S>(key: S, outputs: &[S]) -> FlipFlop
    where
        S: AsRef<str>,
    {
        let outputs = outputs
            .into_iter()
            .map(|o| o.as_ref().to_string())
            .collect();
        FlipFlop {
            key: key.as_ref().to_string(),
            state: LOW,
            active: false,
            outputs,
        }
    }
}

impl Machine for FlipFlop {
    fn write_pulse(&mut self, pulse: &Pulse) {
        if pulse.level == LOW {
            self.state = !self.state;
            self.active = true
        }
    }

    fn get_outputs(&self) -> Vec<Key> {
        self.outputs.iter().cloned().collect()
    }

    fn read_level(&mut self) -> Option<Level> {
        if self.active {
            self.active = false;
            Some(self.state)
        } else {
            None
        }
    }

    fn get_key(&self) -> Key {
        self.key.clone()
    }
}

// Conjunction
#[derive(Debug)]
struct Conjunction {
    key: Key,
    state: std::collections::HashMap<Key, Level>,
    active: bool,
    outputs: Vec<Key>,
}

impl Conjunction {
    fn new<S>(key: S, inputs: &[S], outputs: &[S]) -> Conjunction
    where
        S: AsRef<str>,
    {
        let outputs = outputs
            .into_iter()
            .map(|o| o.as_ref().to_string())
            .collect();
        let state = inputs
            .into_iter()
            .map(|o| (o.as_ref().to_string(), LOW))
            .collect();
        Conjunction {
            key: key.as_ref().to_string(),
            state,
            active: false,
            outputs,
        }
    }
}

impl Machine for Conjunction {
    fn write_pulse(&mut self, pulse: &Pulse) {
        *self
            .state
            .get_mut(&pulse.source)
            .expect(&format!("connected input: {}", pulse.target)) = pulse.level;
        self.active = true;
    }

    fn get_outputs(&self) -> Vec<Key> {
        self.outputs.iter().cloned().collect()
    }

    fn read_level(&mut self) -> Option<Level> {
        if self.active {
            self.active = false;
            if self.state.values().all(|level| *level == HIGH) {
                Some(LOW)
            } else {
                Some(HIGH)
            }
        } else {
            None
        }
    }

    fn get_key(&self) -> Key {
        self.key.clone()
    }
}

fn total_pulses(machines: &mut Machines, inputs: usize) -> usize {
    let mut total_low = 0usize;
    let mut total_high = 0usize;
    for _ in 0..inputs {
        let mut pulses = vec![Pulse::low("button", "broadcaster")];
        while !pulses.is_empty() {
            total_low += pulses.iter().filter(|p| p.level == LOW).count();
            total_high += pulses.iter().filter(|p| p.level == HIGH).count();
            let mut next_pulses = vec![];
            for p in pulses.iter() {
                if let Some(m) = machines.get_mut(&p.target) {
                    m.write_pulse(p);
                    next_pulses.extend(m.read_pulse());
                }
            }
            pulses = next_pulses;
        }
    }
    total_low * total_high
}

#[derive(Debug, PartialEq, Eq)]
enum MachineType {
    Broadcaster,
    FlipFlip,
    Conjunction,
}

fn parse_machines(input: &str) -> Result<Machines, Error> {
    let io: Result<Vec<(&str, &str)>, _> = input
        .trim()
        .lines()
        .map(|l| l.trim().split_once("->").ok_or(Error::msg("no io arrow")))
        .collect();

    let io: Vec<(MachineType, Key, Vec<Key>)> = io?
        .into_iter()
        .map(|(i, o)| {
            (
                match i.trim().chars().next().unwrap() {
                    '%' => MachineType::FlipFlip,
                    '&' => MachineType::Conjunction,
                    _ => MachineType::Broadcaster,
                },
                String::from(&i.trim()[..].replace("%", "").replace("&", "")),
                o.trim()
                    .split(",")
                    .map(|k| String::from(k.trim()))
                    .collect(),
            )
        })
        .collect();

    let mut machines = Machines::new();

    for m in io.iter() {
        match m.0 {
            MachineType::Broadcaster => {
                machines.insert(m.1.clone(), Box::new(Broadcaster::new(m.1.clone(), &m.2)));
            }
            MachineType::FlipFlip => {
                machines.insert(m.1.clone(), Box::new(FlipFlop::new(m.1.clone(), &m.2)));
            }
            MachineType::Conjunction => {
                let inputs: Vec<Key> = io
                    .iter()
                    .filter(|item| item.2.contains(&m.1))
                    .map(|item| item.1.clone())
                    .collect();
                machines.insert(
                    m.1.clone(),
                    Box::new(Conjunction::new(m.1.clone(), &inputs, &m.2)),
                );
            }
        }
    }

    Ok(machines)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_broadcaster() {
        let mut bc = Broadcaster::new("broadcaster", &["ab", "cd"]);
        assert_eq!(Vec::<Pulse>::new(), bc.read_pulse());
        bc.write_pulse(&Pulse::low("broadcaster", "button"));
        assert_eq!(
            vec![
                Pulse::low("broadcaster", "ab"),
                Pulse::low("broadcaster", "cd")
            ],
            bc.read_pulse()
        );
        assert_eq!(Vec::<Pulse>::new(), bc.read_pulse());
    }

    #[test]
    fn test_flipflop() {
        let mut ff = FlipFlop::new("ff", &["ab", "cd"]);
        ff.write_pulse(&Pulse::high("button", "ff"));
        assert_eq!(Vec::<Pulse>::new(), ff.read_pulse());

        ff.write_pulse(&Pulse::low("button", "ff"));
        assert_eq!(
            vec![Pulse::high("ff", "ab"), Pulse::high("ff", "cd")],
            ff.read_pulse()
        );

        ff.write_pulse(&Pulse::low("button", "ff"));
        assert_eq!(
            vec![Pulse::low("ff", "ab"), Pulse::low("ff", "cd")],
            ff.read_pulse()
        );
    }

    #[test]
    fn test_conjunction() {
        let mut conj = Conjunction::new("conj", &["ab", "cd"], &["ef", "gh"]);
        assert_eq!(Vec::<Pulse>::new(), conj.read_pulse());

        conj.write_pulse(&Pulse::high("ab", "conj"));
        conj.write_pulse(&Pulse::high("cd", "conj"));
        assert_eq!(
            vec![Pulse::low("conj", "ef"), Pulse::low("conj", "gh")],
            conj.read_pulse()
        );
        assert_eq!(Vec::<Pulse>::new(), conj.read_pulse());
    }

    #[test]
    fn test_simple_example() {
        // broadcaster -> a, b, c
        // %a -> b
        // %b -> c
        // %c -> inv
        // &inv -> a
        let mut machines = Machines::new();
        machines.insert(
            String::from("broadcaster"),
            Box::new(Broadcaster::new("broadcaster", &["a", "b", "c"])),
        );
        machines.insert(String::from("a"), Box::new(FlipFlop::new("a", &["b"])));
        machines.insert(String::from("b"), Box::new(FlipFlop::new("b", &["c"])));
        machines.insert(String::from("c"), Box::new(FlipFlop::new("c", &["inv"])));
        machines.insert(
            String::from("inv"),
            Box::new(Conjunction::new("inv", &["c"], &["a"])),
        );

        assert_eq!(32, total_pulses(&mut machines, 1));
    }

    #[test]
    fn test_parse_example() {
        let input = "
         broadcaster -> a, b, c
         %a -> b
         %b -> c
         %c -> inv
         &inv -> a
        ";

        let mut machines = parse_machines(&input).unwrap();
        assert_eq!(32, total_pulses(&mut machines, 1));
        assert_eq!(16 * 8, total_pulses(&mut machines, 2));
        assert_eq!(32000000, total_pulses(&mut machines, 1000));
    }

    #[test]
    fn test_parse_example2() {
        let input = "
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        ";
        let mut machines = parse_machines(&input).unwrap();
        assert_eq!(11687500, total_pulses(&mut machines, 1000));
    }
}
