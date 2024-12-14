fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This will panic if the vector is empty
    let first = vec[0];
    println!("First element: {}", first);

    // This will not panic because it checks for emptiness
    if let Some(first) = vec.get(0) {
        println!("First element (safe): {}", first);
    }
}