fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let index = 5;

    match vec.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    }

    //Safer way to access vector elements:
    let safe_index = if index < vec.len(){index} else {0}; //Handle out of bound error by selecting the first element if index is out of bound
    match vec.get(safe_index) {
        Some(value) => println!("Value at index {}: {}", safe_index, value),
        None => println!("Index {} is out of bounds", safe_index),
    }
} 