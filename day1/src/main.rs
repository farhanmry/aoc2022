use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut fattest_elf: u32 = 0;
    let mut current_elf_belly: u32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(content) = line {
                let current_value = content.parse::<u32>();
                if current_value.is_ok() {
                    current_elf_belly = current_elf_belly + current_value.unwrap()
                } else {
                    if fattest_elf < current_elf_belly {
                        fattest_elf = current_elf_belly;
                    }
                    current_elf_belly = 0;
                  }

            }

        }
    }

    println!("answer: {}", fattest_elf)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
