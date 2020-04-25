
pub fn string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

pub fn usize() -> usize {
    let input = string();
    match input.trim().parse::<usize>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to usize\nError! :{}\n🤗 try again..", er);
            usize()
        }
    }
}

pub fn u128() -> u128 {
    let input = string();
    match input.trim().parse::<u128>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to u128\nError: {}\n🤗 try again..", er);
            u128()
        }
    }
}

pub fn u64() -> u64 {
    let input = string();
    match input.trim().parse::<u64>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to u64\nError: {}\n🤗 try again..", er);
            u64()
        }
    }
}

pub fn u16() -> u16 {
    let input = string();
    match input.trim().parse::<u16>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to u16\nError: {}\n🤗 try again..", er);
            u16()
        }
    }
}

pub fn u8() -> u8 {
    let input = string();
    match input.trim().parse::<u8>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to u8\nError: {}\n🤗 try again..", er);
            u8()
        }
    }
}

pub fn isize() -> isize {
    let input = string();
    match input.trim().parse::<isize>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to isize\nError: {}\n🤗 try again..", er);
            isize()
        }
    }
}
pub fn i128() -> i128 {
    let input = string();
    match input.trim().parse::<i128>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to i128\nError: {}\n🤗 try again..", er);
            i128()
        }
    }
}
pub fn i64() -> i64 {
    let input = string();
    match input.trim().parse::<i64>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to i64\nError: {}\n🤗 try again..", er);
            i64()
        }
    }
}

pub fn i32() -> i32 {
    let input = string();
    match input.trim().parse::<i32>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to i32\nError: {}\n🤗 try again..", er);
            i32()
        }
    }
}

pub fn i16() -> i16 {
    let input = string();
    match input.trim().parse::<i16>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to i16\nError: {}\n🤗 try again..", er);
            i16()
        }
    }
}

pub fn i8() -> i8 {
    let input = string();
    match input.trim().parse::<i8>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to i8\nError: {}\n🤗 try again", er);
            i8()
        }
    }
}

pub fn f64() -> f64 {
    let input = string();
    match input.trim().parse::<f64>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to f64\nError: {}\n🤗 try again", er);
            f64()
        }
    }
}

pub fn f32() -> f32 {
    let input = string();
    match input.trim().parse::<f32>() {
        Ok(input) => {
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to f32\nError: {}\n🤗 try again", er);
            f32()
        }
    }
}

pub fn char() -> char {
    let input = string();
    match input.trim().parse::<char>() {
        Ok(input) => {
            println!("👍");
            return input;
        }
        Err(er) => {
            println!("😟 Failed to parse to char\nError: {}\n🤗 try again..", er);
            char()
        }
    }
}
pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
