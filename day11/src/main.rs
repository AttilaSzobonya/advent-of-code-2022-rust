use derivative::Derivative;
use itertools::Itertools;
use std::vec;

#[derive(Derivative)]
#[derivative(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    inspections: u64,

    #[derivative(Debug = "ignore")]
    worry_operation: fn(u64) -> u64,
    #[derivative(Debug = "ignore")]
    divisable_by: u64,

    throw_on_true: usize,
    throw_on_false: usize,
}

fn main() {
    // let mut monkeys: Vec<Monkey> = vec![
    //     Monkey {
    //         items: vec![79, 98],
    //         inspections: 0,
    //         worry_operation: |x| x * 19,
    //         divisable_by: 23,
    //         throw_on_true: 2,
    //         throw_on_false: 3,
    //     },
    //     Monkey {
    //         items: vec![54, 65, 75, 74],
    //         inspections: 0,
    //         worry_operation: |x| x + 6,
    //         divisable_by: 19,
    //         throw_on_true: 2,
    //         throw_on_false: 0,
    //     },
    //     Monkey {
    //         items: vec![79, 60, 97],
    //         inspections: 0,
    //         worry_operation: |x| x * x,
    //         divisable_by: 13,
    //         throw_on_true: 1,
    //         throw_on_false: 3,
    //     },
    //     Monkey {
    //         items: vec![74],
    //         inspections: 0,
    //         worry_operation: |x| x + 3,
    //         divisable_by: 17,
    //         throw_on_true: 0,
    //         throw_on_false: 1,
    //     },
    // ];

    let mut monkeys: Vec<Monkey> = vec![
        Monkey {
            items: vec![54, 82, 90, 88, 86, 54],
            inspections: 0,
            worry_operation: |x| x * 7,
            divisable_by: 11,
            throw_on_true: 2,
            throw_on_false: 6,
        },
        Monkey {
            items: vec![91, 65],
            inspections: 0,
            worry_operation: |x| x * 13,
            divisable_by: 5,
            throw_on_true: 7,
            throw_on_false: 4,
        },
        Monkey {
            items: vec![62, 54, 57, 92, 83, 63, 63],
            inspections: 0,
            worry_operation: |x| x + 1,
            divisable_by: 7,
            throw_on_true: 1,
            throw_on_false: 7,
        },
        Monkey {
            items: vec![67, 72, 68],
            inspections: 0,
            worry_operation: |x| x * x,
            divisable_by: 2,
            throw_on_true: 0,
            throw_on_false: 6,
        },
        Monkey {
            items: vec![68, 89, 90, 86, 84, 57, 72, 84],
            inspections: 0,
            worry_operation: |x| x + 7,
            divisable_by: 17,
            throw_on_true: 3,
            throw_on_false: 5,
        },
        Monkey {
            items: vec![79, 83, 64, 58],
            inspections: 0,
            worry_operation: |x| x + 6,
            divisable_by: 13,
            throw_on_true: 3,
            throw_on_false: 0,
        },
        Monkey {
            items: vec![96, 72, 89, 70, 88],
            inspections: 0,
            worry_operation: |x| x + 4,
            divisable_by: 3,
            throw_on_true: 1,
            throw_on_false: 2,
        },
        Monkey {
            items: vec![79],
            inspections: 0,
            worry_operation: |x| x + 8,
            divisable_by: 19,
            throw_on_true: 4,
            throw_on_false: 5,
        },
    ];

    let modulo = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.divisable_by);

    for _ in 0..10000 {
        for monkey_ix in 0..monkeys.len() {
            for item_ix in 0..monkeys[monkey_ix].items.len() {
                monkeys[monkey_ix].inspections += 1;

                let old_item = monkeys[monkey_ix].items[item_ix];
                let new_item = (&monkeys[monkey_ix].worry_operation)(old_item % modulo);

                if new_item % &monkeys[monkey_ix].divisable_by == 0 {
                    let throw_on_true = monkeys[monkey_ix].throw_on_true;
                    monkeys[throw_on_true].items.push(new_item);
                } else {
                    let throw_on_false = monkeys[monkey_ix].throw_on_false;
                    monkeys[throw_on_false].items.push(new_item);
                }
            }

            monkeys[monkey_ix].items.clear();
        }
    }

    let monkey_business = monkeys
        .iter()
        .sorted_by(|a, b| b.inspections.cmp(&a.inspections)) // sort descending
        .take(2)
        .fold(1, |acc, monkey| acc * monkey.inspections);

    println!();
    println!("Product of two greatest inspections: {}", monkey_business);
}
