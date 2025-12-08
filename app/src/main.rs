use clap::Parser;
use yatl::{duration_to_human_string, Timer};

#[derive(Parser)]
struct Arguments {
    #[clap(short = 't')]
    time: bool,
    #[clap(short, long, value_delimiter = ',')]
    days: Option<Vec<usize>>,
}

type ChallengeFn = fn() -> usize;

fn main() {
    let args = Arguments::parse();
    let funs: [(ChallengeFn, ChallengeFn); 7] = [
        (day_01::part_1, day_01::part_2),
        (day_02::part_1, day_02::part_2),
        (day_03::part_1, day_03::part_2),
        (day_04::part_1, day_04::part_2),
        (day_05::part_1, day_05::part_2),
        (day_06::part_1, day_06::part_2),
        (day_07::part_1, day_07::part_2),
    ];
    let timer = if args.time {
        let mut timer = Timer::new();
        timer.start().unwrap();
        Some(timer)
    } else { None };
    funs.iter().enumerate().for_each(|(mut day, (p1, p2))| {
        day += 1;
        if let Some(days) = &args.days && !days.contains(&day) {
            return;
        }
        let day_timer = if args.time {
            let mut timer = Timer::new();
            timer.start().unwrap();
            Some(timer)
        } else { None };
        println!("Day {}", day);
        println!("- Part 1: {}", p1());
        println!("- Part 2: {}", p2());
        if let Some(mut timer) = day_timer {
            let time = timer.lap().unwrap();
            println!(" Day time: {}\n", duration_to_human_string(&time));
        }
    });
    if let Some(mut timer) = timer {
        let time = timer.lap().unwrap();
        println!("Total time: {}", duration_to_human_string(&time));
    }
}
