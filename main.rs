
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

    // variables
    let integer = 3i32;
    let floatingPoint = 2.1137;

    let mut uInteger = 3u32;
    //integer += 1; error: an immutable variable
    uInteger += 1;

    println!("{}, {}, {}", integer, floatingPoint, uInteger);
    // TODO find out why snake case instead of camel case

    // shadowing
    let first_var = 4;
    {
        let first_var = 51;
        println!("first_var = {}", first_var);
    }
    println!("first_var = {}", first_var);

    // unitialized variables
    let uninit_var;
    //println!("uninit_var = {}", uninit_var); error: use of an init variable
    uninit_var = 231.2;
    println!("already initialized uninit_var = {}", uninit_var);

    // type casting
    let a_integer = 0x40_u8;
    let a_char = a_integer as char;
    println!("char cast from integer {}: {}", a_integer, a_char);
    
    let a_float = 8.45511;
    println!("integer cast from float {}: {}", a_float, a_float as i32);
    
    println!("Binary represntation of {}: {:08b}", -1_i8, -1_i8);
    println!("Binary represntation of {}: {:08b}", -2_i8, -2_i8);

    // aliases
    type FloatingPoint32 = f32;
    let a_float_2 : FloatingPoint32 = 3 as f32;
    println!("Type alias: {}, size of var {}", a_float_2, std::mem::size_of_val(&a_float_2));

    // expressions
    let expr = {
        let x = 4.09;
        let y = 1.28;
        println!("x = {}, y = {}", x, y);
        x / y
    };
    println!("expr = {}", expr);

    // if-statement (every if-statement is an expression too, which is really a cool thing)
    let speed = 10;
    let mass = 890;
    let consumption =
        if speed < 0 {
            -1.0
        }
        else if speed == 0 {
            0.8
        }
        else if speed > 0 && speed < 10 {
            18.52
        }
        else if speed >= 10 && speed < 30 {
            let x = mass / speed;
            (x as f32) * 0.05
        }
        else {
            11.71
        };
        println!("Speed = {}, consumption = {}", speed, consumption);

        // loop
        let mut count = 0u32;
        loop {
            println!("loop iteration #{}", count);
            count += 1;
            if count > 9 {
            break;
            }
        }

        count = 0;
        while count < 10 {
            println!("while loop iteration #{}", count);
            count += 1;
        }

        for n in 0u32..10 {
            println!("for loop iteration #{}", n);
        }


}
