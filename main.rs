
fn main() {

    // formatted output
    println!("Hello, Rust!");
    println!("Integer number = {}", 23);
    println!("Float number = {}", 1.012);
    println!("Repeat printing: {0}, {1}, {1}, {3}, {2}", 2, 1, 44, 9);
    println!("Named arguments: {name}, {age}, {occupation}", age="24", name="Ann", occupation="Designer");
    println!("{:08b} XOR {:08b} = {:08b}", 3i8, 32i8, 3i8 ^ 34i8);
    println!("Floating point number {:?}", 2.910210);
    println!("Hex: {:08X}, {:08x}", 131342112, 131342112);
    // TODO find out why this doesn't work:
    //println!("{0:08X}, {0:08x}", 7661);

}
