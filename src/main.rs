use crossterm::*;
use std::{collections::HashMap, fmt::Formatter, io::Write};

enum Types {
    Int(i32),
    Float(f32),
    Char(char),
    Str(String),
}

impl Types {
    fn new<S: AsRef<str>>(s: S) -> Self {
        let s = s.as_ref();
        let is_num = { s.chars().all(|c| c.is_digit(10)) };

        if is_num {
            if s.contains('.') {
                return Types::Float(s.parse::<f32>().unwrap());
            } else {
                return Types::Int(s.parse::<i32>().unwrap());
            }
        } else if s.len() == 1 {
            return Types::Char(s.chars().nth(0).unwrap());
        } else {
            return Types::Str(s.into());
        }
    }

    fn id(&self) -> &str {
        match self {
            Types::Int(_) => "Int",
            Types::Float(_) => "Float",
            Types::Char(_) => "Char",
            Types::Str(_) => "String",
        }
    }
}

impl std::fmt::Display for Types {
    fn fmt(&self, fm: &mut Formatter<'_>) -> std::fmt::Result {
        use Types::*;
        match self {
            Int(i) => write!(fm, "{}", i),
            Float(f) => write!(fm, "{}", f),
            Char(c) => write!(fm, "{}", c),
            Str(s) => write!(fm, "{}", s),
        }
    }
}

fn input() -> String {
    let mut s = String::new();
    match std::io::stdin().read_line(&mut s) {
        Ok(_) => s.trim().to_owned(),
        Err(_) => String::new(),
    }
}

fn intro() {
    println!("QQ Interpretor v0.1");
}

fn main() {
    let mut cmds = HashMap::<String, Types>::new();
    let mut exit = false;
    let mut sout = std::io::stdout();
    intro();

    while !exit {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let expression = input();
        if !expression.is_empty() {
            if expression == "exit" {
                exit = true;
            } else if expression.starts_with("echo") {
                let expression = expression.split_at(4);
                let name = expression.1.trim();
                if cmds.contains_key(name) {
                    println!("{}", cmds[name]);
                } else {
                    println!("Error: {} does not exist", name);
                }
            } else if expression.starts_with("type") {
                let expression = expression.split_at(4).1.trim();
                if cmds.contains_key(expression) {
                    println!("{}", cmds[expression].id());
                }
            } else if expression.starts_with("clear") {
                sout.execute(terminal::Clear(terminal::ClearType::All))
                    .unwrap();
                intro();
            } else if expression.contains(|p| p == '=') {
                let mut expression = expression.split('=');
                let count = expression.clone().count();
                if count == 2 {
                    let name = expression.next();
                    let value = expression.next();
                    if name.is_some()
                        && !name.unwrap().chars().next().unwrap().is_digit(10)
                        && value.is_some()
                    {
                        cmds.insert(
                            name.unwrap().trim().into(),
                            Types::new(value.unwrap().trim()),
                        );
                    } else {
                        println!("Error on making a variable.");
                    }
                }
            } else {
                println!("Invalid Command.");
            }
        }
    }
}
