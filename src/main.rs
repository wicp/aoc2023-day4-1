fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let split_line: Vec<Vec<&str>> = line.split(": ").map(|slice| slice.split(" | ").collect()).collect(); 
    let [winning_nums, my_nums] = split_line[1][0..2] else {panic!("unexpected format in line {}", line)};
    let [winning_nums, my_nums]: [Vec<u32>; 2] = [winning_nums, my_nums].map(|nums| nums
                                                                                .split_whitespace()
                                                                                .map(|num| num.parse::<u32>().expect("NaN"))
                                                                            .collect());
    (winning_nums,my_nums)
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Could not read input.txt in current directory");
    let mut total = 0u32;
    for line in input.lines() {
        let (winning_nums, my_nums) = parse_line(line);
        total += my_nums.iter().filter(|num| winning_nums.contains(num)).fold(0, |acc, _num| if acc == 0 {1} else{acc*2});
    }
    println!("{}", total);
}
