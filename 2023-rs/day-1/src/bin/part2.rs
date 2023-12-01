fn main() {
    let input = include_str!("input2.txt");
    let result = input.split("\n").map(|line| {
        if line.len() == 0 {
            return 0;
        }
        let mut first_idx = line.len();
        let mut last_idx = 0;
        let mut first_num = 0;
        let mut last_num = 0;
        for num in 1..=9 {
            let word = match num {
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                5 => "five",
                6 => "six",
                7 => "seven",
                8 => "eight",
                _ => "nine",
            };
            let mut word_idx = line.find(word);
            if word_idx.is_some() && word_idx.unwrap() <= first_idx {
                first_idx = word_idx.unwrap();
                first_num = num;
            }
            let mut num_idx = line.find(&num.to_string());
            if num_idx.is_some() && num_idx.unwrap() <= first_idx {
                first_idx = num_idx.unwrap();
                first_num = num;
            }
            word_idx = line.rfind(word);
            if word_idx.is_some() && word_idx.unwrap() >= last_idx {
                last_idx = word_idx.unwrap();
                last_num = num;
            }
            num_idx = line.rfind(&num.to_string());
            if num_idx.is_some() && num_idx.unwrap() >= last_idx {
                last_idx = num_idx.unwrap();
                last_num = num;
            }
        }
        println!("{} {} {} {}", first_idx, first_num, last_idx, last_num);
        first_num * 10 + last_num
    }).sum::<i32>();
    println!("Result: {result}");
}
