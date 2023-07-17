#![allow(unused)]
use std::io::{stdin, self};
use rand::Rng;

fn main() {
    println!("what is your name?");
    let mut name: String = String::new();
    let greet: &str="Nice to meet you";
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");
    println!("{},{}", greet,name.trim());

}