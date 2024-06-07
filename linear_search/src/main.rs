
fn linear_search(a:[u32; 6], b: u32 ) {

    let mut position = 0;
    // Returns the index position of the target if found, else returns None
    for index in 0..a.len() {
        if (a[index] == b){
            position = index;
            break;
        }
    }
    let solution = if (position == 0) {"{b} not available in the array"} else {"{b} is available in {position}"};

    println!("{solution}");
}
fn main() {

    println!("Hello, world!");
}
