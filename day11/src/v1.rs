use std::{cmp::Reverse, fs, mem, ops::{Add, Mul}};

struct Monkey {
    items: Vec<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    test_num: i64,
    true_throw: usize,
    false_throw: usize,
    inspections: i32,
}

impl Monkey {
    //Resolves operator string to function
    fn resolve_op(str_op: &str) -> Box<dyn Fn(i64) -> i64> {
        //Matches tokens to values
        enum Operand {
            Old,
            Value(i64),
        }
        //Parses operand
        fn parse_operand(token: &str) -> Operand {
            match token {
                "old" => Operand::Old,
                val => Operand::Value(val.parse().unwrap()),
            }
        }

        //Parse each token
        let mut tokens = str_op.split(" ");

        let a = parse_operand(tokens.next().unwrap());

        let func: fn(i64, i64) -> i64 = match tokens.next().unwrap() {
            "*" => i64::mul,
            "+" => i64::add,
            _ => panic!("Invalid operation"),
        };

        let b = parse_operand(tokens.next().unwrap());

        //Create function based on string parsing
        Box::new(move |x| {
            let left = match a {
                Operand::Old => x,
                Operand::Value(v) => v,
            };

            let right = match b {
                Operand::Old => x,
                Operand::Value(v) => v,
            };

            func(left, right)
        })
    }

    //Performs inspection and toss
    fn inspect_and_toss(index: usize, monkies: &mut Vec<Monkey>) {
        //Takes the memory such that it is owned locally by items, replacing it with an empty vec
        let items = mem::take(&mut monkies[index].items);

        for item in items {
            //Call operator on each item then div by 3
            let worry = (monkies[index].op)(item) / 3;
            monkies[index].inspections += 1;

            //If divisible by test num, give to true_throw
            let target = if worry % monkies[index].test_num == 0 {
                monkies[index].true_throw
            }
            //Otherwise, give to false_throw
            else {
                monkies[index].false_throw
            };

            //Update items for target
            monkies[target].items.push(worry);
        }
    }
}

pub fn v1() {
    //Reads text to string
    let text = fs::read_to_string("src/input.txt").unwrap();
    //Holds all monkies
    let mut monkies: Vec<Monkey> = vec![];

    //Do a whole lot of input processing
    for chunk in text.split("\n\n") {
        let mut lines = chunk.lines();

        //Toss Monkey: # lines
        lines.next();

        //Get worry level for items it holds
        let items: Vec<i64> = lines
            .next().unwrap()
            .split(": ").nth(1).unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        //String representation of operation on inspection
        let str_op = lines
            .next().unwrap()
            .split("= ").nth(1).unwrap();

        //Number to test division with
        let test_num: i64 = lines
            .next().unwrap()
            .split("by ").nth(1).unwrap()
            .parse().unwrap();

        //Which to throw to if true
        let true_throw: usize = lines
            .next().unwrap()
            .split("monkey ").nth(1).unwrap()
            .parse().unwrap();

        //Which to throw to if false
        let false_throw: usize = lines
            .next().unwrap()
            .split("monkey ").nth(1).unwrap()
            .parse().unwrap();

        //Add to vec
        monkies.push(
            Monkey {
                items,
                op: Monkey::resolve_op(str_op),
                test_num,
                true_throw,
                false_throw,
                inspections: 0,
            }
        );
    }

    //Process all the monkies 20 times
    for _ in 0..20 {
        for index in 0..monkies.len() {
            Monkey::inspect_and_toss(index, &mut monkies);
        }
    }

    //Sort descending
    monkies.sort_by_key(|monkey| Reverse(monkey.inspections));
    //Print mult of top 2
    println!("{}", monkies[0].inspections * monkies[1].inspections);
}