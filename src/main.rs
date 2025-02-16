use hashcash::hashcash;

fn main() {
    let result = hashcash("0", 4);
    println!("{}", result);
}
