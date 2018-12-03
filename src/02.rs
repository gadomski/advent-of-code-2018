use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/02.txt");
    println!("Part 1: {}", checksum(input));
    println!("Part 2: {}", common_letters_between_correct_ids(input));
}

fn checksum(input: &str) -> u64 {
    let mut has_two_of_any_letter = 0;
    let mut has_three_of_any_letter = 0;
    for line in input.lines() {
        let mut frequencies = HashMap::new();
        for c in line.chars() {
            let mut entry = frequencies.entry(c).or_insert(0);
            *entry += 1;
        }
        if frequencies.values().any(|&n| n == 2) {
            has_two_of_any_letter += 1;
        }
        if frequencies.values().any(|&n| n == 3) {
            has_three_of_any_letter += 1;
        }
    }
    has_two_of_any_letter * has_three_of_any_letter
}

fn common_letters_between_correct_ids(_input: &str) -> String {
    unimplemented!()
}

#[test]
fn part_1() {
    let input = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
    assert_eq!(12, checksum(input));
}

#[test]
fn part_2() {
    let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
    assert_eq!("fgij", common_letters_between_correct_ids(input));
}
