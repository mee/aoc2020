pub fn day15() {
    let turns = 2020;
    let gen = GameNumGen::new(vec![20, 9, 11, 0, 1, 2]);
    println!(
        "On turn {} the number {} will be spoken",
        turns,
        gen.take(2020).last().unwrap()
    );
}

use std::collections::HashMap;

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
}

/*
For example, suppose the starting numbers are 0,3,6:

    Turn 1: The 1st number spoken is a starting number, 0.
    Turn 2: The 2nd number spoken is a starting number, 3.
    Turn 3: The 3rd number spoken is a starting number, 6.

expected state: turn = 4, last_spoke = 6, nums = {0:1, 3:2, 6:3}

    Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had
            been spoken, the 4th number spoken is 0.
    Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the
            next number to speak is the difference between the turn number when it was last spoken
            (the previous turn, 4) and the turn number of the time it was most recently spoken
            before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
    Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2.
            So, the 6th number spoken is 5 - 2, 3. Turn 7: Since 3 was just spoken twice in a row,
            and the last two turns are 1 turn apart, the 7th number spoken is 1.
    Turn 8: Since 1 is new, the 8th number spoken is 0.
    Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between
            them, 4.
    Turn 10: 4 is new, so the 10th number spoken is 0.
*/

impl Iterator for GameNumGen {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        //        println!("turn {}\n{:?}", self.turn, &self);
        if self.turn <= self.starting_nums.len() {
            self.speak(self.starting_nums[self.turn - 1]);
        } else {
            if let Some(whens) = self.nums.get(&self.last_spoken) {
                if whens.len() <= 1 {
                    self.speak(0);
                } else {
                    let li = whens.len() - 1;
                    self.speak(whens[li] - whens[li - 1]);
                }
            } else {
                self.speak(0);
            }
        }
        Some(self.last_spoken)
    }
}

impl GameNumGen {
    fn speak(&mut self, num: usize) -> usize {
        self.last_spoken = num;
        if self.nums.contains_key(&num) {
            self.nums.get_mut(&num).unwrap().push(self.turn);
        } else {
            self.nums.insert(num, vec![self.turn]);
        }
        self.turn += 1;
        self.last_spoken
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
}
