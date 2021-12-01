mod day1 {
    pub fn generator(input: &str) -> Vec<u64> {
        input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect()
    }

    pub fn part_1(input: &[u64]) -> u64 {
        let (count, _) = input.iter().cloned().fold((0, 99999), |acc, x| {
            let (mut count, last) = acc;
            if x > last { count = count + 1};
            return (count, x);
        });

        count
    }

    pub fn part_2(input: &[u64]) -> u64 {
        let mut new_input:Vec<u64> = Vec::new(); 

        input[2..].iter().enumerate().for_each(|(i, x)| {
            // i is index in reference of "chopped" vector
            new_input.push(x+input[i]+input[i+1])    
        });

        part_1(&new_input)
    }
}

aoc_main::main! {
    year 2021;
    day1 : generator => part_1, part_2;
}
