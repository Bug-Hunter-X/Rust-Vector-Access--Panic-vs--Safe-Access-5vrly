fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe access using get()
    if let Some(first) = vec.get(0) {
        println!("First element: {}", first);
    } else {
        println!("Vector is empty");
    }

    // Handling potential None value
    let first = vec.get(0).unwrap_or(&0);
    println!("First element (or 0): {}", first);
} 