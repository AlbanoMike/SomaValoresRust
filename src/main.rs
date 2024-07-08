
use core::num;
use std::{f32::DIGITS, io, str::Chars};
fn convert_to_int(data: &str) -> i32{
  let x = data.trim().parse::<i32>().unwrap();
  x
}

fn main() {
  let mut soma = 0;
  let mut valorEntrada = String::new();
  io::stdin().read_line(&mut valorEntrada).expect("ERROR");
  let mut valor = convert_to_int(&valorEntrada);
  while valor != 0 {
      let mut r = valor % 10;
      soma += r;
      valor = valor/10;
  }
  
  print!("{}",soma);
  


}
  