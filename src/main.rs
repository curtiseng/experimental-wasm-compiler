use std::error::Error;
use crate::parse::{func, module, parse_white_space};

mod parse;
mod compile;

fn main() -> Result<(), Box<dyn Error>> {
    let wat = r#"
        (module
            (func (export "add") (param i32 i32) (result i32)
                local.get 0
                local.get 1
                i32.add
            ))
    "#;
    let (remainder, ws) = parse_white_space(wat)?;
    println!("remainder: {remainder}, ws: {ws}");
    let (remainder, module) = module(remainder)?;
    println!("remainder: {remainder}, module: {module}");
    let (remainder, func) = func(remainder)?;
    println!("remainder: {remainder}, func: {func}");
    Ok(())
}

