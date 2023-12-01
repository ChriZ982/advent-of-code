fn main() {
    let input = include_str!("input1.txt");
    let result = input.split("\n").map(|line| {
        if line.len() == 0 {
            return 0;
        }
        let first_idx = line.find(char::is_numeric).unwrap();
        let last_idx = line.rfind(char::is_numeric).unwrap();
        let first_char = line.chars().nth(first_idx).unwrap().to_string();
        let last_char = line.chars().nth(last_idx).unwrap().to_string();
        let num_str = first_char + &last_char;
        println!("num_str: {num_str}");
        num_str.parse::<i32>().unwrap() 
    }).sum::<i32>();
    println!("Result: {result}");
}
