use crate::util::load_lines;
use std::str::FromStr;

const MAX_CUBES: CubeSet = CubeSet::from_tuple((12, 13, 14));

pub fn puzzle_2_1() -> u32 {
    load_lines("2/input.txt")
        .map(|l| l.unwrap().parse::<Game>().expect("valid puzzle input"))
        .filter(|g| g.possible(&MAX_CUBES))
        .fold(0, |id_sum, g| id_sum + g.get_id())
}

pub fn puzzle_2_2() -> u32 {
    load_lines("2/input.txt")
        .map(|l| l.unwrap().parse::<Game>().expect("valid puzzle input"))
        .map(|g| g.minimal_set().power())
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
enum ColorCube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
enum ColorCubeParseError {
    NotEnoughItems,
    NoNumber,
    NoColor,
}

impl FromStr for ColorCube {
    type Err = ColorCubeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ColorCube::*;
        let mut it = s.split(' ').map(str::trim);
        let count = it
            .next()
            .ok_or(Self::Err::NotEnoughItems)?
            .parse::<u32>()
            .map_err(|_| Self::Err::NoNumber)?;
        let color = it.next().ok_or(Self::Err::NotEnoughItems)?;
        Ok(match color {
            "red" => Red(count),
            "green" => Green(count),
            "blue" => Blue(count),
            _ => return Err(Self::Err::NoColor),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    const fn from_tuple(rgb: (u32, u32, u32)) -> Self {
        CubeSet {
            red: rgb.0,
            green: rgb.1,
            blue: rgb.2,
        }
    }

    fn possible(&self, max: &CubeSet) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct GameRoundParseError {}

impl FromStr for CubeSet {
    type Err = GameRoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ColorCube::*;
        let rgb = s
            .split(',')
            .map(str::trim)
            .map(|s| s.parse::<ColorCube>())
            .fold((0, 0, 0), |mut rgb, cube| {
                match cube {
                    Ok(Red(x)) => rgb.0 += x,
                    Ok(Green(x)) => rgb.1 += x,
                    Ok(Blue(x)) => rgb.2 += x,
                    _ => (), // Invalid cubes are ignored...
                };
                rgb
            });
        Ok(CubeSet::from_tuple(rgb))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    rounds: Vec<CubeSet>,
}

#[derive(Debug)]
enum GameParseError {
    NoGameId,
    InvalidGameId,
    NoRounds,
    InvalidGameRound,
}

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(':').map(str::trim);
        let id = it
            .next()
            .ok_or(Self::Err::NoGameId)?
            .split(" ")
            .last()
            .ok_or(Self::Err::NoGameId)?
            .parse::<u32>()
            .or(Err(Self::Err::InvalidGameId))?;
        let rounds = it
            .next()
            .ok_or(Self::Err::NoRounds)?
            .split(";")
            .map(|s| s.parse::<CubeSet>())
            .collect::<Result<Vec<CubeSet>, _>>()
            .or(Err(Self::Err::InvalidGameRound))?;
        Ok(Game { id, rounds })
    }
}

impl Game {
    fn possible(&self, max: &CubeSet) -> bool {
        self.rounds.iter().fold(true, |p, r| p && r.possible(max))
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn minimal_set(&self) -> CubeSet {
        CubeSet::from_tuple(self.rounds.iter().fold((0, 0, 0), |max, r| {
            (r.red.max(max.0), r.green.max(max.1), r.blue.max(max.2))
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_color_cube() {
        assert_eq!(ColorCube::Red(1), "1 red".parse::<ColorCube>().unwrap());
        assert_eq!(
            ColorCube::Green(34),
            "34 green".parse::<ColorCube>().unwrap()
        );
        assert_eq!(ColorCube::Blue(0), "0 blue".parse::<ColorCube>().unwrap());
    }

    #[test]
    fn test_parse_game_round() {
        assert_eq!(
            CubeSet {
                red: 1,
                green: 2,
                blue: 3
            },
            "1 red, 2 green, 3 blue".parse::<CubeSet>().unwrap()
        );
    }

    #[test]
    fn test_parse_game() {
        let round1 = CubeSet::from_tuple((1, 2, 3));
        let round2 = CubeSet::from_tuple((4, 5, 6));
        let round3 = CubeSet::from_tuple((7, 8, 9));
        assert_eq!(
            Game {
                id: 1,
                rounds: vec![round1, round2, round3]
            },
            "Game 1: 1 red, 3 blue, 2 green; 4 red, 5 green, 6 blue; 9 blue, 8 green, 7 red"
                .parse::<Game>()
                .unwrap()
        );
    }

    #[test]
    fn test_round_possible() {
        let round1 = CubeSet::from_tuple((1, 2, 3));
        let round2 = CubeSet::from_tuple((4, 5, 6));
        let round3 = CubeSet::from_tuple((7, 8, 9));
        let maximum = CubeSet::from_tuple((5, 5, 5));
        assert!(round1.possible(&maximum));
        assert!(!round2.possible(&maximum));
        assert!(!round3.possible(&maximum));
    }

    #[test]
    fn test_game_possible() {
        let round1 = CubeSet::from_tuple((1, 2, 3));
        let round2 = CubeSet::from_tuple((4, 5, 6));
        let maximum = CubeSet::from_tuple((5, 5, 5));
        let game1 = Game {
            id: 1,
            rounds: vec![round1.clone()],
        };
        let game2 = Game {
            id: 2,
            rounds: vec![round1.clone(), round2.clone()],
        };
        assert!(game1.possible(&maximum));
        assert!(!game2.possible(&maximum));
    }

    #[test]
    fn test_minimal_set() {
        let round1 = CubeSet::from_tuple((1, 2, 3));
        let round2 = CubeSet::from_tuple((4, 5, 6));
        let round3 = CubeSet::from_tuple((2, 8, 1));
        let game1 = Game {
            id: 1,
            rounds: vec![round1, round2, round3],
        };
        assert_eq!(CubeSet::from_tuple((4, 8, 6)), game1.minimal_set());
    }
}
