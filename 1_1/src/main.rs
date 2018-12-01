use std::io;

fn main() {

    let mut sum : i32 = 0;
    while true {
        // Declare variable and read it in
        let mut val = String::new();
        let num_bytes = io::stdin().read_line(&mut val).ok().expect("Failed to read line");
        if num_bytes == 0 {
            break;
        }
        let val_num: i32 = val.trim().parse().ok().expect("Expected numeric value");
        sum += val_num;
    }
    println!("{}", sum);
}
