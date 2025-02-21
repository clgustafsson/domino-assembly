use std::{env, fs, io};
use Instuction::*;

enum Instuction {
    ADDI,
    ADD,
    JUMP,
    BEQ,
    LI,
    INPUT,
    OUTPUT,
    EXIT,
}

fn main() -> Result<(), io::Error> {
    let mut registers: [i32; 4] = [0; 4];
    let mut curr_line = 0;

    let file_path;

    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(file) => {
            if file.ends_with(".domino") {
                file_path = file;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "file must end with .domino",
                ));
            }
        }
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "use format ./domino <filepath>",
            ))
        }
    }

    println!("Running {}", file_path);
    let contents = fs::read_to_string(file_path)?;

    loop {
        let line = contents.lines().nth(curr_line);

        match line {
            Some(l) => {
                let instuction = l.chars().next().unwrap();
                match parse_instruction(instuction)? {
                    ADD => {
                        add(l, &mut registers)?;
                    }
                    ADDI => {
                        addi(l, &mut registers)?;
                    }
                    JUMP => {
                        jump(l, &mut curr_line)?;
                        continue;
                    }
                    LI => {
                        load_immediate(l, &mut registers)?;
                    }
                    BEQ => {
                        branch_if_equal(l, &mut registers, &mut curr_line)?;
                        continue;
                    }
                    INPUT => {
                        input(&mut registers)?;
                    }
                    OUTPUT => {
                        output(&mut registers);
                    }
                    EXIT => break,
                }
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Program exited unintentionally",
                ))
            }
        }
        curr_line += 1;
    }
    Ok(())
}

fn parse_instruction(c: char) -> Result<Instuction, io::Error> {
    match c {
        '🁪' => return Ok(ADD),
        '🁱' => return Ok(JUMP),
        '🁸' => return Ok(BEQ),
        '🁿' => return Ok(LI),
        '🂆' => return Ok(INPUT),
        '🂍' => return Ok(OUTPUT),
        '🁣' => return Ok(ADDI),
        '🁢' => return Ok(EXIT),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid operation")),
    }
}

fn parse_register(c: char) -> Result<usize, io::Error> {
    match c {
        '🁤' => Ok(0),
        '🁥' => Ok(1),
        '🁦' => Ok(2),
        '🁧' => Ok(3),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid register")),
    }
}

fn add(instruction: &str, registers: &mut [i32; 4]) -> Result<(), io::Error> {
    let r1_c = instruction.chars().nth(1).unwrap();
    let r2_c = instruction.chars().nth(2).unwrap();
    let r3_c = instruction.chars().nth(3).unwrap();
    let r1 = parse_register(r1_c)?;
    let r2 = parse_register(r2_c)?;
    let r3 = parse_register(r3_c)?;
    registers[r1] = registers[r2] + registers[r3];
    Ok(())
}

fn addi(instruction: &str, registers: &mut [i32; 4]) -> Result<(), io::Error> {
    let r1_c = instruction.chars().nth(1).unwrap();
    let r2_c = instruction.chars().nth(2).unwrap();
    let r1 = parse_register(r1_c)?;
    let r2 = parse_register(r2_c)?;
    let imm = parse_imm(instruction, 9)?;
    registers[r1] = registers[r2] + imm;
    Ok(())
}

fn jump(instruction: &str, ptr: &mut usize) -> Result<(), io::Error> {
    let jump = parse_imm(instruction, 13)?;
    let mut temp_ptr = *ptr as i32;
    temp_ptr += jump;
    *ptr = temp_ptr.try_into().unwrap();
    Ok(())
}

fn load_immediate(instruction: &str, registers: &mut [i32; 4]) -> Result<(), io::Error> {
    let r1_c = instruction.chars().nth(1).unwrap();
    let r1 = parse_register(r1_c)?;
    let imm = parse_imm(instruction, 11)?;
    registers[r1] = imm;
    Ok(())
}

fn branch_if_equal(
    instruction: &str,
    registers: &mut [i32; 4],
    ptr: &mut usize,
) -> Result<(), io::Error> {
    let r1_c = instruction.chars().nth(1).unwrap();
    let r2_c = instruction.chars().nth(2).unwrap();
    let r1 = parse_register(r1_c)?;
    let r2 = parse_register(r2_c)?;
    let imm = parse_imm(instruction, 9)?;
    if registers[r1] == registers[r2] {
        let mut temp_ptr = *ptr as i32;
        temp_ptr += imm;
        *ptr = temp_ptr.try_into().unwrap();
    } else {
        *ptr += 1;
    }
    Ok(())
}

fn input(registers: &mut [i32; 4]) -> Result<(), io::Error> {
    let mut input = String::new();
    println!("Enter an int: ");
    io::stdin().read_line(&mut input)?;
    registers[0] = input.trim().parse::<i32>().unwrap();
    Ok(())
}

fn output(registers: &mut [i32; 4]) {
    println!("{}", registers[0]);
}

fn parse_imm(s: &str, size: u8) -> Result<i32, io::Error> {
    let mut bit_string = String::new();
    for c in s.chars() {
        let bits = match c {
            //this match statement was lovely to code
            '🀱' => "",
            '🀲' => "0",
            '🀳' => "00",
            '🀴' => "000",
            '🀵' => "0000",
            '🀶' => "00000",
            '🀷' => "000000",
            '🀸' => "1",
            '🀹' => "10",
            '🀺' => "100",
            '🀻' => "1000",
            '🀼' => "10000",
            '🀽' => "100000",
            '🀾' => "1000000",
            '🀿' => "11",
            '🁀' => "110",
            '🁁' => "1100",
            '🁂' => "11000",
            '🁃' => "110000",
            '🁄' => "1100000",
            '🁅' => "11000000",
            '🁆' => "111",
            '🁇' => "1110",
            '🁈' => "11100",
            '🁉' => "111000",
            '🁊' => "1110000",
            '🁋' => "11100000",
            '🁌' => "111000000",
            '🁍' => "1111",
            '🁎' => "11110",
            '🁏' => "111100",
            '🁐' => "1111000",
            '🁑' => "11110000",
            '🁒' => "111100000",
            '🁓' => "1111000000",
            '🁔' => "11111",
            '🁕' => "111110",
            '🁖' => "1111100",
            '🁗' => "11111000",
            '🁘' => "111110000",
            '🁙' => "1111100000",
            '🁚' => "11111000000",
            '🁛' => "111111",
            '🁜' => "1111110",
            '🁝' => "11111100",
            '🁞' => "111111000",
            '🁟' => "1111110000",
            '🁠' => "11111100000",
            '🁡' => "111111000000",
            _ => "",
        };
        bit_string += bits;
    }
    let first_bit = bit_string[..1].to_string();
    bit_string = "0".repeat(32 - bit_string.len() + 1) + &bit_string[1..];

    if first_bit == "1" {
        //if the int is negative we need to inverse the string
        bit_string = bit_string
            .chars()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect();
    }

    bit_string =
        bit_string[..1].to_string() + "0".repeat(32 - bit_string.len()).as_str() + &bit_string[1..];

    if let Ok(parsed_value) = u32::from_str_radix(&bit_string, 2) {
        let parsed_int = parsed_value as i32;
        let max_val: i32 = (i32::pow(2, size as u32 - 1)) - 1;
        let min_val: i32 = -(i32::pow(2, size as u32 - 1));

        if parsed_int <= max_val && parsed_int >= min_val {
            return Ok(parsed_int);
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "imm not in range"));
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to parse imm"));
    }
}
