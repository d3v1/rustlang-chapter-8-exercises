use std::collections::HashMap;

fn main() {

    let number_list = vec![1,2,2,5,12,32,13,235,12,2,245,432,532,1,23,45,324,23,5,234,23,5,32];

    let mut total:i32 = 0;

    // get the mean
    for i in &number_list {
        total += *i;
    }

    println!("total {}", total);


    // counts
    // let mut integercounts = HashMap::new();


}