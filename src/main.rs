use std::io;

enum Temperature {
    Celsius(Celsius),
    Fahrenheit(Fahrenheit),
    Err(String),
}

#[derive(Debug)]
struct Celsius {
    val: u32,
}

impl Celsius {
    pub fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit {
            val: self.val * 9 / 5 + 32,
        }
    }
}

impl std::fmt::Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Debug)]
struct Fahrenheit {
    val: u32,
}

impl Fahrenheit {
    pub fn to_celsius(&self) -> Celsius {
        Celsius {
            val: (self.val - 32) * 5 / 9,
        }
    }
}

impl std::fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

fn parse_input(input: &str) -> Temperature {
    if input.ends_with('f') {
        Temperature::Fahrenheit(Fahrenheit {
            val: parse_num(input),
        })
    } else if input.ends_with('c') {
        Temperature::Celsius(Celsius {
            val: parse_num(input),
        })
    } else {
        Temperature::Err("Input invalid. Must end with 'c' or 'f'.".to_string())
    }
}

fn parse_num(input: &str) -> u32 {
    let temperature: String = input.chars().take(input.len() - 1).collect();

    match temperature.parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

fn main() {
    println!("Enter temperature as 77f or 18c: ");

    let mut usr_inpt = String::new();

    io::stdin()
        .read_line(&mut usr_inpt)
        .expect("can not read user input");

    let temperature = parse_input(&usr_inpt.trim_end().to_lowercase());

    match temperature {
        Temperature::Celsius(celsius) => println!("Fahrenheit: {}", celsius.to_fahrenheit()),
        Temperature::Fahrenheit(fahrenheit) => println!("Celsius: {}", fahrenheit.to_celsius()),
        Temperature::Err(reason) => eprintln!("{}", reason),
    }
}
