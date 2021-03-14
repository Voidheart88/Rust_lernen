use heapless::consts::*;
use heapless::String;
use heapless::Vec;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
enum State {
    Default,
    SetPWM,
    Reset,
}

#[derive(Debug, PartialEq)]
enum Errors {
    ValueError,
}

fn process(input: String<U64>) -> (State, Result<i32, Errors>) {
    // split input into tokens
    let tokens: Vec<&str, U4> = input.split(' ').collect();

    // Check length
    if tokens.len() < 3 {
        return (State::Default, Err(Errors::ValueError));
    }

    // Parse value
    let value = match tokens[2].parse::<i32>() {
        Ok(v) => Ok(v),
        Err(e) => Err(Errors::ValueError),
    };

    // Match first and second token
    match tokens[0] {
        "set" => match tokens[1] {
            "pwm" => (State::SetPWM, value),
            _ => (State::Default, value),
        },
        _ => (State::Default, value),
    }
}

fn main() -> io::Result<()> {
    let input: String<U64> = String::from("set pwm 100");

    let (state, value) = process(input);
    println!("{:?} {}", state, value.unwrap());

    loop {}
    return Ok(());
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_val() {
        let input: String<U64> = String::from("set pwm 100");
        let (state, value) = process(input);
        assert_eq!(value.unwrap(), 100);
    }

    #[test]
    fn t_state() {
        let input: String<U64> = String::from("set pwm 100");
        let (state, value) = process(input);
        assert!(state == State::SetPWM);
        assert!(state != State::Default);
    }

    #[test]
    fn t_cmd_error() {
        let input: String<U64> = String::from("st psm 100");
        let (state, value) = process(input);
        assert!(state == State::Default);
    }

    #[test]
    #[should_panic]
    fn t_val_error() {
        let input: String<U64> = String::from("st psm ddd");
        let (state, value) = process(input);
        value.unwrap(); //panic!
    }

    #[test]
    fn t_wrong_length() {
        let input: String<U64> = String::from("set psmddd");
        let (state, value) = process(input);
        assert!(state == State::Default);
        assert!(value == Err(Errors::ValueError));
    }
}
