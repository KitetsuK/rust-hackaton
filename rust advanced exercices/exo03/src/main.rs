use thiserror::Error;

#[derive(Error, Debug)]
enum CustomError {
    #[error("Could not load config")]
    FileReadErr {
        #[from] 
        source: std::io::Error,
    },
    #[error("Could not parse the number {0}")]
    ParseNumErr(#[from] std::num::ParseIntError),
}

fn main() -> Result<(), CustomError> {
    let input = std::fs::read_to_string("input.txt")?;
    let input = input.split(" ");

    let mut sum = 0;
    for number in input {
        let number = number.parse::<i32>()?;
        sum += number;
    }

    println!("sum: {}", sum);
    Ok(())
}
