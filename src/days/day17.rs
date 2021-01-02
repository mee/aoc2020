use std::collections::HashMap;
use std::str::FromStr;

pub fn day17() {
    let mut space = include_str!("17.input").parse::<Space>().unwrap();
    match space.step_times(6) {
        Err(_) => eprintln!("Error."),
        Ok(_) => {
            println!(
                "After 6 steps, there are {} active cubes",
                space.active_count(),
            );
        }
    };

    let mut space = include_str!("17.input").parse::<Space4D>().unwrap();
    match space.step_times(6) {
        Err(_) => eprintln!("Error."),
        Ok(_) => {
            println!(
                "After 6 steps, there are {} active cubes in ~~4D Space~~ wooo",
                space.active_count(),
            );
        }
    };
}

// I _could_ make space parametric over its dimension, but that sounds like work
type Point4D = (isize, isize, isize, isize);

#[derive(PartialEq)]
struct Space4D {
    size: isize,
    inner: HashMap<Point4D, bool>,
}

#[derive(Debug)]
struct Space4DStepError;

impl Space4D {
    pub fn step_times(self: &mut Self, count: usize) -> Result<(), SpaceStepError> {
        for _ in 0..count {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(self: &mut Self) -> Result<(), SpaceStepError> {
        let mut new_inner: HashMap<Point4D, bool> = HashMap::new();
        let lb = -self.size - 1;
        let ub = self.size + 1;
        for x in lb..=ub {
            for y in lb..=ub {
                for z in lb..=ub {
                    for w in lb..=ub {
                        let p: Point4D = (x, y, z, w);

                        // look at neighbors
                        let mut active_nbors = 0;
                        for xd in (-1 as isize)..=1 {
                            for yd in (-1 as isize)..=1 {
                                for zd in (-1 as isize)..=1 {
                                    for wd in (-1 as isize)..=1 {
                                        if xd == 0 && yd == 0 && zd == 0 && wd == 0 {
                                            continue;
                                        }
                                        let np: Point4D = (p.0 + xd, p.1 + yd, p.2 + zd, p.3 + wd);
                                        if *self.inner.get(&np).unwrap_or(&false) {
                                            active_nbors += 1;
                                        }
                                    }
                                }
                            }
                        }

                        //set point
                        let mut increase_size = false;
                        if *self.inner.get(&p).unwrap_or(&false) {
                            let new_state = active_nbors == 2 || active_nbors == 3;
                            new_inner.insert(p, new_state);
                        } else {
                            let new_state = active_nbors == 3;
                            new_inner.insert(p, new_state);
                            if new_state
                                && (x.abs() > self.size
                                    || y.abs() > self.size
                                    || z.abs() > self.size)
                                || w.abs() > self.size
                            {
                                increase_size = true;
                            }
                        }
                        self.size += if increase_size { 1 } else { 0 };
                    }
                }
            }
        }
        self.inner = new_inner;
        Ok(())
    }

    pub fn active_count(self: &Self) -> usize {
        self.inner.values().filter(|&v| *v).count()
    }
}

#[derive(Debug)]
struct Space4DParseError;

impl FromStr for Space4D {
    type Err = Space4DParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // assumes size is 1 or 3x3
        let mut inner: HashMap<Point4D, bool> = HashMap::new();
        let mut size: isize = 1;
        for (li, line) in s.lines().enumerate() {
            size = (line.len() as isize) / 2;
            for (ci, c) in line.chars().enumerate() {
                inner.insert((li as isize - size, ci as isize - size, 0, 0), c == '#');
            }
        }
        Ok(Self { size, inner })
    }
}

impl std::fmt::Debug for Space4D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl std::fmt::Display for Space4D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in -self.size..=self.size {
            for w in -self.size..=self.size {
                writeln!(f, "z={}, w={}", z, w)?;
                for x in -self.size..=self.size {
                    for y in -self.size..=self.size {
                        write!(
                            f,
                            "{}",
                            self.inner.get(&(x, y, z, w)).map_or('.', |&b| if b {
                                '#'
                            } else {
                                '.'
                            })
                        )?;
                    }
                    writeln!(f)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

//-------

type Point = (isize, isize, isize);

#[derive(PartialEq)]
struct Space {
    size: isize,
    inner: HashMap<Point, bool>,
}

#[derive(Debug)]
struct SpaceStepError;

impl Space {
    pub fn step_times(self: &mut Self, count: usize) -> Result<(), SpaceStepError> {
        for _ in 0..count {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(self: &mut Self) -> Result<(), SpaceStepError> {
        let mut new_inner: HashMap<Point, bool> = HashMap::new();
        let lb = -self.size - 1;
        let ub = self.size + 1;
        for x in lb..=ub {
            for y in lb..=ub {
                for z in lb..=ub {
                    let p: Point = (x, y, z);

                    // look at neighbors
                    let mut active_nbors = 0;
                    for xd in (-1 as isize)..=1 {
                        for yd in (-1 as isize)..=1 {
                            for zd in (-1 as isize)..=1 {
                                if xd == 0 && yd == 0 && zd == 0 {
                                    continue;
                                }
                                let np: Point = (p.0 + xd, p.1 + yd, p.2 + zd);
                                if *self.inner.get(&np).unwrap_or(&false) {
                                    active_nbors += 1;
                                }
                            }
                        }
                    }

                    //set point
                    let mut increase_size = false;
                    if *self.inner.get(&p).unwrap_or(&false) {
                        let new_state = active_nbors == 2 || active_nbors == 3;
                        new_inner.insert(p, new_state);
                    } else {
                        let new_state = active_nbors == 3;
                        new_inner.insert(p, new_state);
                        if new_state
                            && (x.abs() > self.size || y.abs() > self.size || z.abs() > self.size)
                        {
                            increase_size = true;
                        }
                    }
                    self.size += if increase_size { 1 } else { 0 };
                }
            }
        }
        self.inner = new_inner;
        Ok(())
    }

    pub fn active_count(self: &Self) -> usize {
        self.inner.values().filter(|&v| *v).count()
    }
}

#[derive(Debug)]
struct SpaceParseError;

impl FromStr for Space {
    type Err = SpaceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // assumes size is 1 or 3x3
        let mut inner: HashMap<Point, bool> = HashMap::new();
        let mut size: isize = 1;
        for (li, line) in s.lines().enumerate() {
            size = (line.len() as isize) / 2;
            for (ci, c) in line.chars().enumerate() {
                inner.insert((li as isize - size, ci as isize - size, 0), c == '#');
            }
        }
        Ok(Self { size, inner })
    }
}

impl std::fmt::Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in -self.size..=self.size {
            writeln!(f, "z={}", z)?;
            for x in -self.size..=self.size {
                for y in -self.size..=self.size {
                    write!(
                        f,
                        "{}",
                        self.inner
                            .get(&(x, y, z))
                            .map_or('.', |&b| if b { '#' } else { '.' })
                    )?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[warn(unused_must_use)]
mod test {
    use super::*;
    const INPUT1: &str = ".#.
..#
###";

    #[test]
    fn test() {
        let mut space = INPUT1.parse::<Space>().unwrap();
        assert_eq!(space.active_count(), 5);
        space.step_times(6).unwrap();
        assert_eq!(space.active_count(), 112);
        println!("{}", space);
    }

    #[test]
    fn test4_d() {
        let mut space = INPUT1.parse::<Space4D>().unwrap();
        assert_eq!(space.active_count(), 5);
        space.step_times(6).unwrap();
        assert_eq!(space.active_count(), 848);
        println!("{}", space);
    }
}
