#![feature(drain_filter)]

mod day1;
mod day2;
mod day3;
mod day4;

aoc_main::main! {
    year 2021;
    day1 : generator => part_1, part_2;
    day2 : generator => part_1, part_2;
    day3 : generator => part_1, part_2;
    day4 : generator => part_1, part_2;
}

