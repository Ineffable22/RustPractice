mod c_lib;
use c_lib::sum;
mod cmd;
use cmd::{
    built_in::{exit, cd},
    commands::execve,
};

use std::io::{stdin, stdout, Write};

fn get_line(status: &mut i32) -> String {
    let mut input = String::new();
    let size: i32 = stdin()
        .read_line(&mut input)
        .expect("No se pudo leer la línea") as i32;
    if size == 0 || size == 1 {
        *status = size;
    }
    input.trim().to_string()
}

fn main() {
    let a: i32;
    unsafe { a = sum(1, 2) };
    println!("sum(1, 2) = {}", a);

    let mut status: i32 = 1;
    while status != 0 {
        // Write the character '>' followed by a space to stdout
        let stdout: std::io::Stdout = stdout();
        // Block stdout and store the lock in handle
        let mut handle: std::io::StdoutLock<'_> = stdout.lock();
        handle.write(b"> ").expect("Error al escribir en stdout");

        // Secure that the text is shown immediately
        handle.flush().expect("Error al vaciar el búfer");

        let input: String = get_line(&mut status);
        // println!("Input: {} | Status: {}", input, status);
        if status != 0 {
            by_pass(input);
        }
    }
}

fn by_pass(input: String) {
    let token: Vec<&str> = input.split(" ").collect::<Vec<&str>>();

    match token[0] {
        "exit" => exit(token),
        "cd" => cd(token),
        _ => execve(token),
    }
}
