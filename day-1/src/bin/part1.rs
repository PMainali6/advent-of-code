fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let contents:Vec<_> = input.split("\n").collect();
    let mut output: Vec<i32> = vec![];

    for line in contents {
        let mut line_number = vec![];
        line.chars().for_each(|c| {
            if c.is_numeric() {
                line_number.push(c.to_owned());
            }
        });

        let first_digit = line_number.first().unwrap();
        let last_digit = line_number.last().unwrap();
        let line_number = first_digit.to_string() 
        + &format!("{}", &last_digit);
        
        let num = line_number.parse::<i32>().unwrap();
        output.push(num);
    }
    let final_out = output.iter().sum();
    final_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./input1.txt");
        let received = part1(input);
        let expected = 142;
        assert_eq!(received, expected);
    }
}