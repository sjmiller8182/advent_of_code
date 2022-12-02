use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // iter past name
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config { filename })
    }
}

fn calc_elf_calories(elf_calories: &str) -> u64 {
    let elf_calories: Vec<u64> = elf_calories
        .split("\n")
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    elf_calories.iter().sum()
}

fn count_cal(contents: &str) -> HashMap<usize, u64> {
    let mut elf_cal = HashMap::new();
    let contents_iter = contents.split("\n\n").enumerate();
    for (idx, elf_calories) in contents_iter {
        let calories = calc_elf_calories(elf_calories);
        elf_cal.insert(idx + 1, calories);
    }
    elf_cal
}

fn find_max_idx(cal_counts: &HashMap<usize, u64>) -> usize {
    let mut max_idx = 0;
    let mut max_value: u64 = 0;
    for (k, v) in cal_counts {
        if v > &max_value {
            max_idx = *k;
            max_value = *v;
        }
    }
    max_idx
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let cal_counts = count_cal(&contents);
    let max_idx = find_max_idx(&cal_counts);
    
    println!("The elf with the max calories is elf {} with {} calories", max_idx, cal_counts.get(&max_idx).unwrap_or(&0));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calc_elf_calories() {
        let contents = "\
7000
8000
9000";

        assert_eq!(24000, calc_elf_calories(contents))
    }

    #[test]
    fn test_count_cal() {
        let contents  = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let exp: HashMap<usize, u64> = HashMap::from([(1, 6000), (2, 4000), (3, 11000), (4, 24000), (5, 10000)]);

        let result = count_cal(contents);
        assert_eq!(exp, result)
    }

    #[test]
    fn test_find_max_idx() {
        let cal_counts: HashMap<usize, u64> = HashMap::from([(1, 6000), (2, 4000), (3, 11000), (4, 24000), (5, 10000)]);
        assert_eq!(4, find_max_idx(&cal_counts))
    }
}
