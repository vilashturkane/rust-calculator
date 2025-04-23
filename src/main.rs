fn main() {
    println!("---------Rust Calculator-------");
    println!("Mishal, tum addition ka code likho");

    println!("Addition is: {}", addition(10, 20));
    //bhai add  to ho gya ab substract kr de with branch
    println!("Substraction  is: {}", substract(20, 10));
    println!("Multiplication  is: {}", multiply(20, 10));

}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}
fn substract(a: i32, b:i32) -> i32 {
    a - b 
}

fn multiply(a: i32, b:i32) -> i32 {
    a * b 
}

