use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    println!("Day 1:");
    let path = Path::new("./inputs/day1.txt");

    let mut running_total = 0;
    let re = Regex::new(r"(\d)").unwrap();
    
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(code) = line {

                let mut line_total = 0;
                let mut mat = re.find_iter(&code);

                line_total += match &mat.next() {
                    Some(res) => res.as_str().chars().last().unwrap().to_digit(10).unwrap() * 10,
                    None => panic!("No digits in this line")
                };

                line_total += match &mat.last() {
                    Some(res) => res.as_str().chars().last().unwrap().to_digit(10).unwrap(),
                    None => line_total/10 // using this as a shortcut to get the first value again
                };

                running_total += line_total;

                println!("Line Total: {line_total}")
            }
        }
    }

    println!("TOTAL: {running_total}")

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
