fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safe way to modify
    println!("v: {:?}", v);
}