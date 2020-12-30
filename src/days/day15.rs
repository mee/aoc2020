use std::collections::HashMap;

pub fn day15() {
    let turn_count1 = 2020;
    let gen1 = GameNumGen::new(vec![20, 9, 11, 0, 1, 2]);
    println!(
        "On turn {} the number {} will be spoken",
        turn_count1,
        &gen1.take(turn_count1).last().unwrap()
    );

    let gen2 = GameNumGen::new(vec![20, 9, 11, 0, 1, 2]);
    let turn_count2 = 30000000;

    println!(
        "On turn {} the number {} will be spoken",
        turn_count1,
        &gen2.take(turn_count2).last().unwrap()
    );
}

#[derive(Debug)]
struct GameNumGen {
    turn: usize, // 1-indexed!
    last_spoken: usize,
    starting_nums: Vec<usize>,
    nums: HashMap<usize, Vec<usize>>, // number -> turns
}

impl GameNumGen {
    fn new(starting_nums: Vec<usize>) -> Self {
        Self {
            turn: 1,
            last_spoken: 0,
            starting_nums,
            nums: HashMap::new(),
        }
    }

    fn speak(&mut self, num: usize) {
        self.last_spoken = num;
        if self.nums.contains_key(&num) {
            self.nums.get_mut(&num).unwrap().push(self.turn);
        } else {
            self.nums.insert(num, vec![self.turn]);
        }
        self.turn += 1;
    }
}

impl Iterator for GameNumGen {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.turn <= self.starting_nums.len() {
            self.speak(self.starting_nums[self.turn - 1]);
        } else {
            self.speak(match self.nums.get(&self.last_spoken) {
                None => 0,
                Some(whens) if whens.len() <= 1 => 0,
                Some(whens) => whens[whens.len() - 1] - whens[whens.len() - 2],
            });
        }
        Some(self.last_spoken)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let starting_nums: Vec<usize> = vec![0, 3, 6];
        let gen = GameNumGen::new(starting_nums);
        let gen_nums = gen.take(10).collect::<Vec<usize>>();
        assert_eq!(gen_nums.len(), 10);
        assert_eq!(gen_nums, vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0]);
    }

    #[test]
    fn test_example2() {
        let starting_nums: Vec<usize> = vec![1, 3, 2];
        let gen = GameNumGen::new(starting_nums);
        let last = gen.take(2020).last().unwrap();
        assert_eq!(last, 1);
    }

    #[test]
    fn test_example3() {
        let starting_nums: Vec<usize> = vec![2, 1, 3];
        let gen = GameNumGen::new(starting_nums);
        let last = gen.take(2020).last().unwrap();
        assert_eq!(last, 10);
    }

    #[test]
    fn test_example4() {
        let starting_nums: Vec<usize> = vec![1, 2, 3];
        let gen = GameNumGen::new(starting_nums);
        let last = gen.take(2020).last().unwrap();
        assert_eq!(last, 27);
    }

    #[test]
    fn test_example5() {
        let starting_nums: Vec<usize> = vec![2, 3, 1];
        let gen = GameNumGen::new(starting_nums);
        let last = gen.take(2020).last().unwrap();
        assert_eq!(last, 78);
    }

    #[test]
    fn test_example6() {
        let starting_nums: Vec<usize> = vec![3, 2, 1];
        let gen = GameNumGen::new(starting_nums);
        let last = gen.take(2020).last().unwrap();
        assert_eq!(last, 438);
    }

    #[test]
    fn test_example7() {
        let starting_nums: Vec<usize> = vec![3, 1, 2];
        let gen = GameNumGen::new(starting_nums);
        let last = gen.take(2020).last().unwrap();
        assert_eq!(last, 1836);
    }

    #[test]
    fn part1() {
        let gen = GameNumGen::new(vec![20, 9, 11, 0, 1, 2]);
        let last = gen.take(2020).last().unwrap();
        assert_eq!(last, 1111);
    }
}
