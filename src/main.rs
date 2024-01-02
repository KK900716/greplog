use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, stdin};
use std::process::exit;
use regex::Regex;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    check_args(&args);
    let content = read(&args[1]);
    let record = try_match(&args[2], &content);
    monitor(&record, &content);
}

fn monitor(record: &HashMap<u64, u64>, content: &Vec<String>) {
    let mut input = String::new();
    loop {
        match stdin().lock().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "q" => {
                        exit(0);
                    }
                    other => {
                        handle_input(other, record, content);
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
}

fn handle_input(other: &str, record: &HashMap<u64, u64>, content: &Vec<String>) {
    match other.parse::<u64>() {
        Ok(key) => {
            match record.get(&key) {
                Some(value) => {
                    println!();
                    for row in value - 15..value + 16 {
                        if row < content.len() as u64 {
                            println!("{}", content.get(row as usize).unwrap());
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn check_args(args: &Vec<String>) {
    if args.len() != 3 {
        panic!("请重新输入，输入的内容过长：len: {}。", args.len())
    }
}

fn try_match(regex_string: &str, content: &Vec<String>) -> HashMap<u64, u64> {
    let red = "\x1b[31m";
    let reset = "\x1b[0m";
    let regex = Regex::new(regex_string).unwrap();
    let mut count: u64 = 0;
    let mut row: u64 = 0;
    let mut res: HashMap<u64, u64> = HashMap::new();
    for line in content {
        if regex.is_match(line) {
            count += 1;
            row += 1;
            print!("匹配: {}, 行号: {}, 次数:{}: ", row, regex_string, count);
            let format = line.clone().replace(regex_string, &*format!("{}{}{}", red, regex_string, reset));
            println!("{}", format);
            res.insert(count, row);
        }
    }
    return res;
}

fn read(path: &str) -> Vec<String> {
    let file = File::open(path).expect(&*format!("文件代开失败：path：{}", path));
    let reader = io::BufReader::new(file);
    let mut content = Vec::new();
    for line in reader.lines() {
        content.push(line.unwrap());
    }
    return content;
}

#[cfg(test)]
mod tests {
    use crate::{monitor, try_match};

    #[test]
    fn read() {
        let v: Vec<String> = vec!["run".to_string(), "test/resource/test1.log".to_string(), "ab".to_string()];
        let content = crate::read(&v[1]);
        let x = try_match(&v[2], &content);
        monitor(&x, &content);
    }
}