use std::io;
use std::collections::HashSet;

fn main() {

    let mut found = false;
    
    let mut vec = Vec::new();
    let mut sum : i32 = 0;
    let mut vals : HashSet<i32> = HashSet::new();
    while !found
    {
        // Declare variable and read it in
        let mut val = String::new();
        let num_bytes = io::stdin().read_line(&mut val).ok().expect("Failed to read line");
        if num_bytes == 0 {
            break;
        }
        let val_num: i32 = val.trim().parse().ok().expect("Expected numeric value");
        vec.push(val_num);
        sum += val_num;
        let cur_sum = sum;
        if vals.contains(&cur_sum)
        {
            println!("{}", sum);
            found = true;
            break;
        }
        vals.insert(cur_sum);
    }
    while !found
    {
        for x in &vec
        {
            sum += x;
            let cur_sum = sum;
            if vals.contains(&cur_sum)
            {
                println!("{}", sum);
                found = true;
                break;
            }
            vals.insert(cur_sum);
        }
    }
}
