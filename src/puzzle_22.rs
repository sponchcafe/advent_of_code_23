#![allow(unused)]
use crate::util::load_file;
use anyhow::Error;

pub fn puzzle_22_1() -> u64 {
    let input = load_file("22/input.txt");
    let mut stack = brick_stack_from_str(&input).unwrap();
    stack.collapse();
    stack.removable_bricks().len() as u64
}

type Point2d = (i32, i32);
type Point3d = (i32, i32, i32);

fn brick_stack_from_str(s: &str) -> Result<BrickStack, Error> {
    s.trim()
        .lines()
        .enumerate()
        .map(|(i, l)| Brick::parse(l.trim(), i + 1))
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Brick {
    id: usize,
    start: Point3d,
    end: Point3d,
}

impl Brick {
    fn parse(s: &str, id: usize) -> Result<Self, Error> {
        let (start, end) = s.split_once('~').ok_or(Error::msg("invalid brick"))?;
        let parse_point = |s: &str| {
            s.split(',')
                .map(|p| p.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
                .map(|v| (v[0], v[1], v[2]))
        };
        let start = parse_point(start)?;
        let end = parse_point(end)?;
        Ok(Brick { id, start, end })
    }
    fn project_along_z(&self) -> std::collections::BTreeSet<Point2d> {
        let mut proj = std::collections::BTreeSet::<Point2d>::new();
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                proj.insert((x, y));
            }
        }
        proj
    }
    fn bottom(&self) -> i32 {
        self.start.2.min(self.end.2)
    }
    fn height(&self) -> i32 {
        i32::abs(self.end.2 - self.start.2) + 1
    }
    fn lower_to(&mut self, z: i32) {
        let h = self.height();
        self.start.2 = z + 1;
        self.end.2 = z + h;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BrickStack {
    bricks: Vec<Brick>,
    top_of_stack: std::collections::BTreeMap<Point2d, (usize, usize)>, // ID, height
    supports: std::collections::BTreeMap<usize, std::collections::BTreeSet<usize>>,
    supported: std::collections::BTreeMap<usize, std::collections::BTreeSet<usize>>,
}

impl FromIterator<Brick> for BrickStack {
    fn from_iter<T: IntoIterator<Item = Brick>>(iter: T) -> Self {
        let mut bricks: Vec<Brick> = iter.into_iter().collect();
        bricks.sort_by(|a, b| a.bottom().cmp(&b.bottom()));
        BrickStack {
            bricks,
            top_of_stack: Default::default(),
            supports: Default::default(),
            supported: Default::default(),
        }
    }
}

impl BrickStack {
    fn collapse(&mut self) {
        for brick in &mut self.bricks {
            let projection = brick.project_along_z();
            let contact_height = (&projection)
                .into_iter()
                .map(|point| *self.top_of_stack.get(&point).unwrap_or(&(0, 0)))
                .map(|(_, height)| height)
                .max()
                .unwrap_or(0);

            if contact_height == 0 {
                &projection.iter().for_each(|point| {
                    self.top_of_stack.insert(*point, (0, 0));
                });
            }

            let contact_points: Vec<(Point2d, (usize, usize))> = self
                .top_of_stack
                .iter()
                .filter(|(point, (id, _))| projection.contains(point))
                .filter(|(_, (_, height))| *height == contact_height || *height == 0)
                .map(|(point, (id, height))| (*point, (*id, *height))) // Dereferencing manually - not sure why .cloned does not work here...
                .collect();

            for point in projection.iter() {
                self.top_of_stack.insert(
                    point.clone(),
                    (brick.id, contact_height + brick.height() as usize),
                );
            }

            for (point, (id, _)) in contact_points {
                if self.supports.contains_key(&id) {
                    self.supports.get_mut(&id).unwrap().insert(brick.id);
                } else {
                    let mut set = std::collections::BTreeSet::<usize>::new();
                    set.insert(brick.id);
                    self.supports.insert(id, set);
                }

                if self.supported.contains_key(&brick.id) {
                    self.supported.get_mut(&brick.id).unwrap().insert(id);
                } else {
                    let mut set = std::collections::BTreeSet::<usize>::new();
                    set.insert(id);
                    self.supported.insert(brick.id, set);
                }
            }

            /*
            println!(
                "Lowering brick {} ({:?} ~ {:?}) from {} to {} now resting on {:?} with height {}",
                brick.id,
                brick.start,
                brick.end,
                brick.bottom(),
                contact_height,
                self.supported.get(&brick.id).unwrap(),
                brick.height(),
            );
            */

            brick.lower_to((contact_height) as i32);
        }
    }

    fn removable_bricks(&self) -> Vec<usize> {
        let mut removable = vec![];
        let candidates: std::collections::BTreeSet<usize> = self
            .supported
            .iter()
            .filter(|(_, v)| v.len() > 1)
            .flat_map(|(k, v)| v.iter().cloned())
            .collect();

        let candidates: std::collections::BTreeSet<usize> = candidates
            .into_iter()
            .filter(|id| {
                self.supports
                    .get(id)
                    .unwrap()
                    .iter()
                    .all(|s| self.supported.get(s).unwrap().len() > 1)
            })
            .collect();

        removable.extend(candidates.iter());

        // All bricks on top
        removable.extend(
            self.supported
                .iter()
                .filter(|(k, v)| !self.supports.contains_key(k))
                .map(|(k, v)| k),
        );
        removable
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const BRICK_1: &str = "1,2,3~4,2,3";
    const BRICK_2: &str = "2,1,8~2,3,8";
    const BRICK_3: &str = "2,2,9~2,2,12";

    const EXAMPLE: &str = "
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    ";

    #[test]
    fn test_brick_parse() {
        let brick = Brick::parse(BRICK_1, 1).unwrap();
        assert_eq!((1, 2, 3), brick.start);
        assert_eq!((4, 2, 3), brick.end);
    }

    #[test]
    fn test_height() {
        let brick1 = Brick::parse(BRICK_1, 1).unwrap();
        let brick2 = Brick::parse(BRICK_2, 2).unwrap();
        let brick3 = Brick::parse(BRICK_3, 3).unwrap();
        assert_eq!(1, brick1.height());
        assert_eq!(1, brick2.height());
        assert_eq!(4, brick3.height());
    }
    #[test]
    fn test_projection() {
        let brick = Brick::parse(BRICK_1, 1).unwrap();
        let projection = brick.project_along_z();
        assert_eq!(4, projection.len());
        assert!(projection.contains(&(1, 2)));
        assert!(projection.contains(&(2, 2)));
        assert!(projection.contains(&(3, 2)));
        assert!(projection.contains(&(4, 2)));
    }

    #[test]
    fn test_stack_building() {
        let brick1 = Brick::parse(BRICK_1, 1).unwrap();
        let brick2 = Brick::parse(BRICK_2, 2).unwrap();
        let brick3 = Brick::parse(BRICK_3, 3).unwrap();
        let stack: BrickStack = [brick3, brick2, brick1].into_iter().collect();
        assert_eq!(1, stack.bricks[0].id);
        assert_eq!(2, stack.bricks[1].id);
        assert_eq!(3, stack.bricks[2].id);
    }

    #[test]
    fn test_lower() {
        let mut brick1 = Brick::parse(BRICK_1, 1).unwrap();
        brick1.lower_to(0);
        assert_eq!(brick1.start.2, 1);
        assert_eq!(brick1.end.2, 1);
    }

    #[test]
    fn test_collapse() {
        let brick1 = Brick::parse(BRICK_1, 1).unwrap();
        let brick2 = Brick::parse(BRICK_2, 2).unwrap();
        let brick3 = Brick::parse(BRICK_3, 3).unwrap();
        let mut stack: BrickStack = [brick3, brick2, brick1].into_iter().collect();
        stack.collapse();
    }

    #[test]
    fn test_example() {
        let mut stack = brick_stack_from_str(EXAMPLE).unwrap();
        stack.collapse();
        assert_eq!(5, stack.removable_bricks().len());
    }
}
