fn linear_search(a: [u32; 6], b: u32) {
    let mut position = 0;
    // Returns the index position of the target if found, else returns None
    for index in 0..a.len() {
        if a[index] == b {
            position = index;
            break;
        }
    }
    if position == 0 {
        println!("{b} not available in the array");
    } else {
        println!("{b} is available in index: {position}");
    }
}
fn main() {
    let a = [5, 9, 7, 4, 3, 13];
    let b = 3;

    linear_search(a, b);
}
