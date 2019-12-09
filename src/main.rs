use std::fs;
#[macro_use] extern crate text_io;

fn main() {
    part1();
    part2();
}

fn run_instruction(int_codes: &mut Vec<i32>, pos: &mut usize) -> bool {
    let instruction = int_codes[*pos];
    let digits: Vec<_> = instruction.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

    let op_code = digits[digits.len() - 1];
    let mut first_param_mode = 0;
    let mut second_param_mode = 0;

    if op_code == 9 {
        return true;
    }

    if digits.len() > 2 {
        first_param_mode = digits[digits.len() - 3];
    }

    if digits.len() > 3 {
        second_param_mode = digits[digits.len() - 4];
    }

    *pos += 1;

    if op_code == 1 || op_code == 2 { 
        let first_value: i32;
        let second_value: i32;

        if first_param_mode == 0 {
            let first_loc: usize = int_codes[*pos] as usize;
            first_value = int_codes[first_loc];
        }
        else{
            first_value = int_codes[*pos];
        }

        *pos += 1;

        if second_param_mode == 0 {
            let second_loc: usize = int_codes[*pos] as usize;
            second_value = int_codes[second_loc];
        }
        else{
            second_value = int_codes[*pos];
        }

        *pos += 1;

        let destination: usize = int_codes[*pos] as usize;

        if op_code == 1 { // addition
            int_codes[destination] = first_value + second_value;
        }

        if op_code == 2 { // multiplication
            int_codes[destination] = first_value * second_value;
        }
    }

    if op_code == 3 {
        println!("Input: ");
        let input: i32 = read!();

        let destination: usize = int_codes[*pos] as usize;
        int_codes[destination] = input;
    }

    if op_code == 4 {
        if first_param_mode == 0 {
            let destination: usize = int_codes[*pos] as usize;
            println!("output: {}", int_codes[destination]);
        }
        else {
            println!("output: {}", int_codes[*pos]);
        }
    }

    if op_code == 5 {
        let first_value: i32;
        let second_value: i32;

        if first_param_mode == 0 {
            let first_loc: usize = int_codes[*pos] as usize;
            first_value = int_codes[first_loc];
        }
        else{
            first_value = int_codes[*pos];
        }

        *pos += 1;

        if second_param_mode == 0 {
            let second_loc: usize = int_codes[*pos] as usize;
            second_value = int_codes[second_loc];
        }
        else{
            second_value = int_codes[*pos];
        }

        if first_value != 0 {
            *pos = second_value as usize;
            return false;
        }
    }

    if op_code == 6 {
        let first_value: i32;
        let second_value: i32;

        if first_param_mode == 0 {
            let first_loc: usize = int_codes[*pos] as usize;
            first_value = int_codes[first_loc];
        }
        else{
            first_value = int_codes[*pos];
        }

        *pos += 1;

        if second_param_mode == 0 {
            let second_loc: usize = int_codes[*pos] as usize;
            second_value = int_codes[second_loc];
        }
        else{
            second_value = int_codes[*pos];
        }

        if first_value == 0 {
            *pos = second_value as usize;
            return false;
        }
    }

    if op_code == 7 {
        let first_value: i32;
        let second_value: i32;

        if first_param_mode == 0 {
            let first_loc: usize = int_codes[*pos] as usize;
            first_value = int_codes[first_loc];
        }
        else{
            first_value = int_codes[*pos];
        }

        *pos += 1;

        if second_param_mode == 0 {
            let second_loc: usize = int_codes[*pos] as usize;
            second_value = int_codes[second_loc];
        }
        else{
            second_value = int_codes[*pos];
        }

        *pos += 1;

        let destination: usize = int_codes[*pos] as usize;

        if first_value < second_value {
            int_codes[destination] = 1;
        }
        else {
            int_codes[destination] = 0;
        }
    }

    if op_code == 8 {
        let first_value: i32;
        let second_value: i32;

        if first_param_mode == 0 {
            let first_loc: usize = int_codes[*pos] as usize;
            first_value = int_codes[first_loc];
        }
        else{
            first_value = int_codes[*pos];
        }

        *pos += 1;

        if second_param_mode == 0 {
            let second_loc: usize = int_codes[*pos] as usize;
            second_value = int_codes[second_loc];
        }
        else{
            second_value = int_codes[*pos];
        }

        *pos += 1;

        let destination: usize = int_codes[*pos] as usize;

        if first_value == second_value {
            int_codes[destination] = 1;
        }
        else {
            int_codes[destination] = 0;
        }
    }

    *pos += 1;
    
    return false;
}

fn get_int_codes() -> Vec<i32>{
    let codes = fs::read_to_string("input.txt").unwrap();
    
    let int_codes_strs = codes.split(",").collect::<Vec<&str>>();
    let mut int_codes: Vec<i32> = Vec::new();

    for int_code_str in int_codes_strs{
        int_codes.push(int_code_str.parse().unwrap());
    }

    return int_codes;
}

fn part1(){
    let mut int_codes = get_int_codes();

    let mut pos: usize = 0;
    let mut complete: bool = false;

    while !complete {
        complete = run_instruction(&mut int_codes, &mut pos);
    }
}

fn part2(){
    let mut int_codes = get_int_codes();

    let mut pos: usize = 0;
    let mut complete: bool = false;

    while !complete {
        complete = run_instruction(&mut int_codes, &mut pos);
    }
}