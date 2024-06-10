/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n != 1 {
        if n % 2 == 0 {
            n = n/2;
            length += 1;
        } else {
            n = 3 * n + 1;
            length += 1;
        }
    }
    length
  }
  
  fn main() {
println!("Collatz length for n = 3 is {}", collatz_length(3));
  }