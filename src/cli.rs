use std::io;
use std::io::Write;
use serde_json::Value;
use crate::{calc, data};

#[cfg(windows)]
fn pause() {
    // Use the pause of cmd on Windows

    use std::process::Command;
    Command::new("cmd.exe").args(["/c", "pause"]).status().unwrap();
}

#[cfg(unix)]
fn pause() {
    // From https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter

    use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
    use std::io::Read;

    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte
    print!("请按任意键继续...");
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap();  // reset the stdin to original termios data
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    flush_stdout();
}

fn flush_stdout() {
    io::stdout().flush().unwrap();
}

fn get_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}

fn get_menu_choice(min: i32, max: i32) -> i32 {
    loop {
        print!("请输入你的选择：");
        flush_stdout();

        let input_text = get_line();
        let input = match input_text.trim().parse::<i32>() {
            Ok(i) => i,
            Err(..) => {
                println!("请输入一个合法的选项！");
                continue;
            }
        };

        if input >= min && input <= max {
            return input;
        } else {
            println!("请输入一个合法的选项！");
        }
    }
}

fn get_float(hint_text: &str) -> f32 {
    loop {
        print!("{}", hint_text);
        flush_stdout();

        let input_text = get_line();
        match input_text.trim().parse::<f32>() {
            Ok(i) => return i,
            Err(..) => {
                println!("请输入一个合法的数字！");
            }
        };
    }
}

fn print_main_menu() {
    println!("0. 退出Acc单曲计算软件");
    println!("1. 使用内置ma段数据");
}

fn calc_single(data: &Value, group: &str) {
    let examples = (*data)["examples"][group].as_array().unwrap().iter().map(|it| it.as_str().unwrap().to_string()).collect::<Vec<_>>().join(", ");
    let mut input_text = String::new();
    let dan_data: Vec<i32>;
    loop {
        print!("请输入你想计算的段位（例：{}）：", examples);
        flush_stdout();

        io::stdin().read_line(&mut input_text).unwrap();
        if let Some(arr) = (*data)[group][input_text.trim()].as_array() {
            dan_data = arr.iter().map(|it| it.as_i64().unwrap() as i32).collect();
            break;
        } else {
            println!("请检查输入的段位是否存在！");
        }
    }

    let mut acc_list = Vec::new();

    for index in 0..dan_data.len() {
        let acc = get_float(&format!("请输入打完第 {} 首后的准度：", index + 1));
        acc_list.push(calc::calc_acc(acc / 100.0, &acc_list, &dan_data));
    }

    let acc_str = acc_list.iter().map(|it| format!("{:2}%", it * 100.0)).collect::<Vec<String>>().join(" - ");
    println!("您的单曲准度依次为：{}", acc_str);

    pause();
}

pub fn run() {
    let data = data::parse_data();

    loop {
        clear_screen();
        print_main_menu();
        let choice = get_menu_choice(0, 1);

        match choice {
            0 => return,
            1 => calc_single(&data, "malody"),
            _ => panic!("Illegal state!")
        }
    }
}
