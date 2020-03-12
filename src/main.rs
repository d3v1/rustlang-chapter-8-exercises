use std::collections::HashMap;

fn main() {

    let mut number_list = vec![1,4,77,2,2,5,12,32,13,235,12,2,245,432,532,1,23,45,324,23,5,234,23,5,32];
    number_list.sort(); // sort this to work out the median later

    let mut total:f32 = 0 as f32;
    let mut number_counts= HashMap::new();

    // get the total
    for i in &number_list {
        total += *i as f32;
        let count = number_counts.entry(i).or_insert(0);
        *count += 1;
    }
    // work out the mean
    let number_list_mean: f32 = total / number_list.len() as f32;

    // get the median
    let number_list_median: f32;
    // check to see if its an odd number, if it is then we need to get the median from the
    // mean of the two middle numbers
    if number_list.len() % 2 == 1 {
        number_list_median = {
            let first_index = (number_list.len() as f32 / 2 as f32).floor() as usize;
            let second_index = (number_list.len() as f32 / 2 as f32).ceil() as usize;
            (number_list[first_index] as f32 + number_list[second_index] as f32) / 2 as f32
        }
    } else {
        number_list_median = number_list[number_list.len() / 2] as f32
    }

    // taken from the latest answer on this SO question
    // https://stackoverflow.com/questions/34555837/sort-hashmap-data-by-value
    // need to understand what this does a bit more
    let number_list_mode = number_counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    println!("total   {}", total);
    println!("mean    {}", number_list_mean);
    println!("median  {}", number_list_median);
    println!("mode    {}", number_list_mode.0);

}
