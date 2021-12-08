pub fn generator(input: &str) -> Vec<&str> {
    input
        .lines()
        .collect()
}

pub fn part_1(input: &[&str]) -> u32 {
    let len: u32 = input[0].len().try_into().unwrap();
    let mut frequencies: Vec<(u64, u64)> = vec!((0, 0); len.try_into().unwrap()); 

    for line in input.iter() {
        let bytes = line.as_bytes();

        for (i, &bit) in bytes.iter().enumerate() {
            if bit == b'0' {
                frequencies[i].0 += 1;
            } else {
                frequencies[i].1 += 1;
            };
        };
    };

    let mut gama = 0;
    let mut epsilon = 0;

    for (i, &freq) in frequencies.iter().enumerate() {
        let pos = len - 1 - (i as u32);
        if freq.0 > freq.1 {
            epsilon = epsilon | u32::pow(2, pos);
        } else {
            gama = gama | u32::pow(2, pos);
        }
    }
    
    gama * epsilon
}

pub fn part_2(input: &[&str]) -> u32 {
    let mut data = input.to_vec();
    let mut i = 0;

    let oxygen: u32 = loop {
        data = filter(data, true, i);
        if data.len() == 1 {
            break u32::from_str_radix(data[0], 2).unwrap();
        };
        i += 1;
    };

    data = input.to_vec();
    i = 0;

    let co2: u32 = loop {
        data = filter(data, false, i);
        if data.len() == 1 {
            break u32::from_str_radix(data[0], 2).unwrap();
        };
        i += 1;
    };

    oxygen * co2
}

pub fn filter<'a>(input: Vec<&'a str>, method: bool, i: usize) -> Vec<&'a str> {
    let mut result: (Vec<&'a str>, Vec<&'a str>) = (Vec::new(), Vec::new());
    let mut freq: (u32, u32) = (0,0);

    for value in input.iter() {
        if value.bytes().nth(i).unwrap() == b'0' {
            freq.0 += 1;
            result.0.push(value);
        } else {
            freq.1 += 1;
            result.1.push(value);
        };
    };

    if method {
        if freq.0 > freq.1 { 
            return result.0;
        } else { 
            return result.1;
        }
    } else {
        if freq.0 > freq.1 {
            return result.1;
        } else {
            return result.0;
        }
    }
}
