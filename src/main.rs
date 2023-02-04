mod lib;
fn main() {
    println!("Hello, world!");
    let expressions = lib::solve([9, 9, 3, 4]);
    expressions.iter().for_each(|exp| {
        println!("{}", exp);
    });
}
