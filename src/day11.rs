use util::{FromLine, FromLines, read};

mod util;

fn main() {
    // Read data
    let data: Data = read("inputs/day11.txt");

    // Part 1
    println!("Part 1 : {}", data.part_1());

    // Part 2
    println!("Part 2 : {}", data.part_2());
}

#[derive(Debug)]
struct Data(Vec<Monkey>);

impl Data {
    fn part_1(&self) -> usize {
        // 20 rounds. Distress is divided by 3 before monkey inspection.
        Self::monkey_business(self.0.clone(), 20, |it| it / 3)
    }

    fn part_2(&self) -> usize {
        // 10000 rounds. Distress is no longer divided by 3 before monkey inspection.
        // Thus, distress will keep increasing, leading to an integer overflow (even with a u128).
        // We have to find a way to prevent that overflow.
        //
        // Modulo to the rescue!
        //
        //    W = W % X
        //
        // Each monkey decides where to throw an item according to a division test. If the "item"
        // is divisible by a certain value, then he throws it to a certain monkey. Otherwise, he
        // throws it to another monkey. We have to find a X that keeps the properties of W when we
        // modulo it by X.
        //
        // Let's take an example with three monkeys :
        //  - Monkey 1 : Divides by 5
        //  - Monkey 2 : Divides by 3
        //  - Monkey 3 : Divides by 2
        //
        // If W is 175 :
        //  - Monkey 1 : 175 is divisible by 5
        //  - Monkey 2 : 175 is not divisible by 3
        //  - Monkey 3 : 175 is not divisible by 2
        //
        // Let's say we try with X = 100. If W is 175, W % X is 75 :
        //  - Monkey 1 : 75 is divisible by 5
        //  - Monkey 2 : 75 is divisible by 3 <- OH no!
        //  - Monkey 3 : 75 is not divisible by 2
        //
        // We made a mistake about monkey 2 answer. That's not good. Let's try with X = 30.
        // Here, W % X is 25 :
        //  - Monkey 1 : 25 is divisible by 5
        //  - Monkey 2 : 25 is not divisible by 3
        //  - Monkey 3 : 25 is not divisible by 2
        //
        // Great! That's the same answers ! But how can we find that X ? Let's call our three
        // monkeys divisors A, B and C. One way to find X is to multiply A, B and C together.
        // That way, we obtain an X that is guaranteed to be divisible by A, B and C.
        //
        // For our previous monkeys, we have X = 5 * 3 * 2 = 30. Note that :
        //  - 30 is divisible by 5. That gives us 6, or 3 * 2.
        //  - 30 is divisible by 3. That gives us 10, or 5 * 2.
        //  - 30 is divisible by 2. That gives us 15, or 5 * 3.
        //
        // Why does this work ? First, remember that the modulo operator gives us the remainder of
        // a division. This remainder is always smaller than the divider, even if it's only by one.
        // Let's try to modulo every integer from 0 to infinity by 5 :
        //
        //  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | ... |
        //  -----------------------------------------------------------------------------
        //  | 0 | 1 | 2 | 3 | 4 | 0 | 1 | 2 | 3 | 4 |  0 |  1 |  2 |  3 |  4 |  0 | ... |
        //
        // It loops from 0 to 4! If we try with 3, it will loop from 0 to 2. If we try with 2, it
        // will loop form 0 to 1.
        //
        //  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | ... |
        //  -----------------------------------------------------------------------------
        //  | 0 | 1 | 2 | 3 | 4 | 0 | 1 | 2 | 3 | 4 |  0 |  1 |  2 |  3 |  4 |  0 | ... | -> % 5
        //  | 0 | 1 | 2 | 0 | 1 | 2 | 0 | 1 | 2 | 0 |  1 |  2 |  0 |  1 |  2 |  0 | ... | -> % 3
        //  | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 |  0 |  1 |  0 |  1 |  0 |  1 | ... | -> % 2
        //
        // Now, where does the three of them starts looping at the same time ? Here, it's at 30,
        // their smallest common multiple. We can obtain it by multiplying them together (what
        // we did previously).
        //
        // (You may have to scroll to see the example).
        //                                                                                                                                                 |-- First time there is three 0.
        //                                                                                                                                                 V
        //  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | ... |
        //  -------------------------------------------------------------------------------------------------------------------------------------------------------------
        //  | 0 | 1 | 2 | 3 | 4 | 0 | 1 | 2 | 3 | 4 |  0 |  1 |  2 |  3 |  4 |  0 |  1 |  2 |  3 |  4 |  0 |  1 |  2 |  3 |  4 |  0 |  1 |  2 |  3 |  4 |  0 |  1 | ... | -> % 5
        //  | 0 | 1 | 2 | 0 | 1 | 2 | 0 | 1 | 2 | 0 |  1 |  2 |  0 |  1 |  2 |  0 |  1 |  2 |  0 |  1 |  2 |  0 |  1 |  2 |  0 |  1 |  2 |  0 |  1 |  2 |  0 |  1 | ... | -> % 3
        //  | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 |  0 |  1 |  0 |  1 |  0 |  1 |  0 |  1 |  0 |  1 |  0 |  1 |  0 |  1 |  0 |  1 |  0 |  1 |  0 |  1 |  0 |  1 | ... | -> % 2
        //
        // By doing W % 30, we map each W to a value between 0 and 29. It's also repeating :
        //
        //  | 0 | 1 | 2 | 3 | ... | 28 | 29 | 30 | 31 | ... | 298 | 299 | 300 | 301 | ... |
        //  -------------------------------------------------------------------------------
        //  | 0 | 1 | 2 | 3 | ... | 28 | 29 |  0 |  1 | ... |  28 |  29 |   0 |   1 | ... |
        //
        // Interestingly, we now have 4 repeating sequence of numbers that starts over every
        // 30 numbers. THIS is where the magic happens!
        //
        //  | 0 | 1 | 2 | 3 | ... | 28 | 29 | 30 | 31 | ... | 298 | 299 | 300 | 301 | ... |
        //  -------------------------------------------------------------------------------
        //  | 0 | 1 | 2 | 3 | ... | 28 | 29 |  0 |  1 | ... |  28 |  29 |   0 |   1 | ... | % 30
        //  | 0 | 1 | 2 | 3 | ... |  3 |  4 |  0 |  1 | ... |   3 |   4 |   0 |   1 | ... | % 5
        //  | 0 | 1 | 2 | 0 | ... |  1 |  2 |  0 |  1 | ... |   1 |   2 |   0 |   1 | ... | % 3
        //  | 0 | 1 | 0 | 1 | ... |  0 |  1 |  0 |  1 | ... |   0 |   1 |   0 |   1 | ... | % 2
        //
        // By doing W % 30, we map every W to a value between 0 and 29, keeping it inside our
        // repeating pattern. Inside this pattern :
        //  - W % 30 % 5 is the same as W % 5.
        //  - W % 30 % 3 is the same as W % 3.
        //  - W % 30 % 2 is the same as W % 2.
        //
        // We have successfully managed our overflows! Now, here's the code!
        let modulus = self.0.iter().fold(1, |acc, monkey| acc * monkey.test.divisible_by);

        Self::monkey_business(self.0.clone(), 10000, |it| it % modulus)
    }

    fn monkey_business<F>(mut monkeys: Vec<Monkey>, iterations: usize, distress_manager: F) -> usize
        where F: Fn(u64) -> u64 {
        for _ in 0..iterations {
            for i in 0..monkeys.len() {
                let monkey = &mut monkeys[i];
                let items = monkey.starting_items.take();
                let operation = monkey.operation;
                let test = monkey.test;
                monkey.inspections += items.len();

                for mut item in items {
                    // Monkey puts out item.
                    item = operation.apply(item);

                    // Monkey gets bored.
                    item = distress_manager(item);

                    // Monkey throws item.
                    if test.apply(item) {
                        monkeys[test.true_throw_to].starting_items.items.push(item)
                    } else {
                        monkeys[test.false_throw_to].starting_items.items.push(item)
                    }
                }
            }
        }
        monkeys.sort_by_key(|it| it.inspections);
        monkeys.iter().rev().take(2).map(|it| it.inspections).product()
    }
}

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let monkeys = lines.split(on_empty_line!()).map(lines_to!(Monkey)).collect();

        Self(monkeys)
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: StartingItems,
    operation: Operation,
    test: Test,
    inspections: usize,
}

impl FromLines for Monkey {
    fn from_lines(lines: &[&str]) -> Self {
        let starting_items = StartingItems::from_line(&lines[1]);
        let operation = Operation::from_line(&lines[2]);
        let test = Test::from_lines(&lines[3..]);

        Self {
            starting_items,
            operation,
            test,
            inspections: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct StartingItems {
    items: Vec<u64>,
}

impl StartingItems {
    fn take(&mut self) -> Vec<u64> {
        self.items.drain(..).collect()
    }
}

impl FromLine for StartingItems {
    fn from_line(line: &str) -> Self {
        let items = line[18..]
            .split(',')
            .map(|it| u64::from_line(it.trim()))
            .collect();

        Self {
            items
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Pow,
}

impl Operation {
    fn apply(&self, rhs: u64) -> u64 {
        match self {
            Operation::Add(lhs) => lhs + rhs,
            Operation::Multiply(lhs) => lhs * rhs,
            Operation::Pow => rhs * rhs
        }
    }
}

impl FromLine for Operation {
    fn from_line(line: &str) -> Self {
        let operator = &line[23..24];
        let value = &line[25..];

        match (operator, value) {
            ("*", "old") => Self::Pow,
            ("*", value) => Self::Multiply(u64::from_line(value)),
            ("+", value) => Self::Add(u64::from_line(value)),
            _ => panic!("{operator} is not a valid operation")
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Test {
    divisible_by: u64,
    true_throw_to: usize,
    false_throw_to: usize,
}

impl Test {
    fn apply(&self, value: u64) -> bool {
        value % self.divisible_by == 0
    }
}

impl FromLines for Test {
    fn from_lines(lines: &[&str]) -> Self {
        let divisible_by = u64::from_line(&lines[0][21..]);
        let true_throw_to = usize::from_line(&lines[1][29..]);
        let false_throw_to = usize::from_line(&lines[2][30..]);

        Self {
            divisible_by,
            true_throw_to,
            false_throw_to,
        }
    }
}