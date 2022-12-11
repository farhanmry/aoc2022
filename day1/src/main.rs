use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut elf_stack: [u32; 3] = [0, 0, 0];

    let mut current_elf_belly: u32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let line_value = line.expect("idk why this one not converted into string");

            if let Ok(line_value) = line_value.parse::<u32>() {
                current_elf_belly = current_elf_belly + line_value;
            } else {
                arrange_elf(&mut elf_stack, current_elf_belly);
                current_elf_belly = 0;
            }
        }
    }

    let mut total:u32 =  0;
    for fat in elf_stack {
        total = total + fat
    }

    println!("answer: {}", total)

}

fn arrange_elf(elf_stack: &mut [u32; 3], current_elf_belly: u32) {
    if current_elf_belly > elf_stack[0] {
        elf_stack[2] = elf_stack[1];
        elf_stack[1] = elf_stack[0];
        elf_stack[0] = current_elf_belly;

        return;
    }

    if current_elf_belly > elf_stack[1] {
        elf_stack[2] = elf_stack[1];
        elf_stack[1] = current_elf_belly;

        return;
    }

    if current_elf_belly > elf_stack[2] {
        elf_stack[2] = current_elf_belly;
        return;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
