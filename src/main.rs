use std::io;

enum Temperature {
    Celsius(Celsius),
    Fahrenheit(Fahrenheit),
    Err(String),
}

trait F {
    fn to_fahrenheit(&self) -> Fahrenheit;
}

trait C {
    fn to_celsius(&self) -> Celsius;
}

#[derive(Debug)]
struct Celsius {
    val: u32,
}

impl F for Celsius {
    fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit {
            val: self.val * 9 / 5 + 32,
        }
    }
}

impl std::fmt::Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}°C", self.val)
    }
}

#[derive(Debug)]
struct Fahrenheit {
    val: u32,
}

impl C for Fahrenheit {
    fn to_celsius(&self) -> Celsius {
        Celsius {
            val: (self.val - 32) * 5 / 9,
        }
    }
}

impl std::fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}°F", self.val)
    }
}

fn parse_input(input: &str) -> Temperature {
    if input.ends_with('f') || input.ends_with('F') {
        Temperature::Fahrenheit(Fahrenheit {
            val: parse_num(input),
        })
    } else if input.ends_with('c') || input.ends_with('C') {
        Temperature::Celsius(Celsius {
            val: parse_num(input),
        })
    } else {
        Temperature::Err("Input invalid. Must end with 'cC' or 'fF'.".to_string())
    }
}

fn parse_num(input: &str) -> u32 {
    let temperature: String = input.chars().take(input.len() - 1).collect();

    temperature.parse().unwrap_or(0)
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
