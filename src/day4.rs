use std::collections::BTreeMap;

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

pub struct Bingo {
    boards: Vec<(BTreeMap<u8, usize>, u32)>,
    nums: Vec<u8> 
}

pub struct Card {
    board: BTreeMap<u8, usize>,
    mark: u32,
    num: u8
}

pub fn generator(input: &str) -> Bingo { 
    let (nums, boards) = input.split_once("\n\n").unwrap();

    let nums = nums
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<(BTreeMap<u8, usize>, u32)> = boards
        .split("\n\n")
        .map(|b| {
            (
                b.split_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    Bingo { boards, nums }
}

pub fn part_1(input: &Bingo) -> u32 {
    let nums = &input.nums;
    let mut boards = input.boards.clone();

    let (board, mark, num) = nums
         .into_iter()
         .find_map(|n| {
             boards.iter_mut().find_map(|(b, m)| {
                b.get(&n)
                    .map(|i| *m |= 1 << *i)
                    .filter(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
                    .map(|_| (b.clone(), *m, n))
            })
        })
        .unwrap();

    let result = board
        .into_iter()
        .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * *num as u32)
        .sum::<u32>();

    result 
}


pub fn part_2(input: &Bingo) -> u32 {
    let nums = &input.nums;
    let mut boards = input.boards.clone();

    let (board, mark, num) = nums
        .into_iter()
        .filter_map(|n| {
            boards
                .drain_filter(|(b, m)| {
                    b.get(&n)
                        .map(|i| *m |= 1 << *i)
                        .map(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
                        .unwrap_or(false)
                })
                .map(|(b, m)| (b, m, n))
                .next()
        })
        .last()
        .unwrap();

    let result = board
        .into_iter()
        .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * *num as u32)
        .sum::<u32>();

    result 
}

