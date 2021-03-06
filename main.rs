
// library
extern crate my_extern_lib;

// modules
mod simple_functions {
    pub fn print_hello_world() {
        println!("Hello, World! (from the simple_functions module)");
     }

    // nested modules
    pub mod simple_math {
        pub fn multiple_two_integers(a: i32, b: i32) -> i32 {
            a * b
        }
    }
}

use simple_functions::simple_math::multiple_two_integers;

fn print_some_msg() {
    println!("Modules: printing from the outer function");
}

mod sample_module {

    pub fn call_sample_module() {
        use second_module::print_some_msg as print_second;
        use self::second_module::print_some_msg as print_second_sample; 
        use super::print_some_msg as print_outside;
        print_outside();
        print_second();
        print_some_msg();
        print_second_sample();
    }
    
    mod second_module {
    
        pub fn print_some_msg() {
            println!("Modules: printing from the second module inside the sample module");
        }    

    }

    fn print_some_msg() {
        println!("Modules: printing from the inside of the sample module");
    }

}

mod second_module {
    pub fn print_some_msg() {
        println!("Modules: printing from the second module");
    }
}

mod modules;

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
    let floating_point = 2.1137;

    let mut u_integer = 3u32;
    //integer += 1; error: an immutable variable
    u_integer += 1;

    println!("{}, {}, {}", integer, floating_point, u_integer);
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

        // matching
        let age = 19;
        match age {
            1  => println!("Seems like 1"),
            5  => println!("It's 5"),
            18 => println!("Exactly 18!"),
            _  => println!("I couldn't guess :("),
        }

        match age {
            a if a > 1 && a <= 9  => println!("The age is in range [1,9]"),
            a if a > 9 && a <= 19 => println!("The age is in range [10, 19]"),
            _                     => println!("Nothing, I don't know what range the age is in :("),
        }

        // the previous match is equivalent to the following:
        match age {
            a @ 1...9   => println!("The age is {} and in range [1,9]", a),
            a @ 10...19 => println!("The age is {} and in range [10, 19]", a),
            a           => println!("Nothing, I don't know what range the age {} is in :(", a),
        }

        // if let
        let a_some = Some(0.0008);
        if let Some(i) = a_some {
            println!("a_some is not None, {}", i);
            println!("result {}",  9.0 / i);
        }
        else {
            println!("None");
        }

        // while let
        let mut a_optional = Some(10);

        while let Some(i) = a_optional {
            if i == 20 {
                println!("while let: i equals 20, leave");
                a_optional = None;
            }
            else {
                a_optional = Some(i + 1);
            }
        }

        // functions, finally!
        let first_integer: i32 = -20;
        let second_integer: i32 = 8;
        let result_of_addition = add_two_integers(first_integer, second_integer);
        println!("{} + {} = {}", first_integer, second_integer, result_of_addition); 


        simple_functions::print_hello_world();
        let mul_result = simple_functions::simple_math::multiple_two_integers(3, 15);
        println!("Modules: multiplication result = {}", mul_result);

        // this doesn't work.. why?
        println!("Modules: multiplication result 2 = {}", multiple_two_integers(-1, 8));

        // "super" and "self"
        sample_module::call_sample_module();

        // file hierarchy
        {
            let a : i32 = 4;
            let b : i32 = 15;
            println!("modules::math {} + {} = {}", a, b, modules::math::add_two_int(a, b));
        }

        // call external library
        my_extern_lib::run(5);

        // tuples
        let pair = ("key", 4);
        println!("tuple value: ({}, {})", pair.0, pair.1);
        println!("tuple value: {:?}", pair);

        // structs
        let bmw: Car = Car {color: "Blue".to_string(), weight: 940};
        println!("BMW: {} - {}", bmw.color, bmw.weight);

        // tuple structs
        let color_1: RgbColor = RgbColor(123, 33, 81);
        println!("RGB color: {}, {}, {}", color_1.0, color_1.1, color_1.2);

        // unit structs
        let _nil = Nil;

        // a struct with a constructor and getter
        let rabbit = my_extern_lib::Animal::new(2, 4, "The King");
        println!("Rabbit: {}, {}, {}", rabbit.age, rabbit.weight, rabbit.get_secret_info()); 

        // generics
        {
            struct ThreeValues<T, U, V> {
                first: T,
                second: U,
                third: V,
            }

            let three_values = ThreeValues { first: 4, second: 2.01, third: "hello" };
            println!("Generic data structure: {}, {}, {}", three_values.first, three_values.second, three_values.third);

            // generic function
            fn swap<U, V>(first_value: U, second_value: V) -> (V, U) {
                (second_value, first_value)
            }

            let random_tuple = swap(4, 2);
            println!("Use unused variables: {} {}", random_tuple.0, random_tuple.1);

        }

        // box (heap allocation)
        {

            let toyota: Car = Car {color: "Green".to_string(), weight: 1040,};
            let porche: Box<Car> = Box::new(Car {color: "Black".to_string(), weight: 876, });

            println!("Toyota's size on stack is {}", std::mem::size_of_val(&toyota));
            println!("Boxed porche's size on stack is {}", std::mem::size_of_val(&porche));

            let unboxed_porche: Car = *porche;
            println!("Unboxed porche's size on stack is {}", std::mem::size_of_val(&unboxed_porche));

            // ownership
            let bmw : Box<Car> = Box::new(Car {color: "Yellow".to_string(), weight: 1620});
            println!("Boxed BMW at {:p}: {}/{}", &bmw, bmw.color, bmw.weight);
            let moved_bmw: Box<Car> = bmw;
            println!("Moved BMW at {:p}: {}/{}", &moved_bmw, moved_bmw.color, moved_bmw.weight);
            // can't do the following, the ownership has been moved
            // println!("Boxed BMW at {:p}: {}/{}", &bmw, bmw.color, bmw.weight);

            // mutability
            let immutable_car = Box::new(Car {color: "Pink".to_string(), weight: 510});
            // can't do this
            // immutable_car.weight = 403;

            // but we can change mutability
            let mut mutable_car = immutable_car;
            mutable_car.weight = 401;

        }

        // ref
        {

            let mut bmw: Car = Car {color: "Green".to_string(), weight: 870};
            println!("Ref: bmw {},{}", bmw.color, bmw.weight);

            {
                let Car {color: _, weight: ref weight_reference} = bmw;

                println!("Ref: {}", *weight_reference);
            }

            {
                let Car {color: _, weight: ref mut weight_mut_ref} = bmw;

                *weight_mut_ref = 200;
            }

            println!("Ref: bmw {},{}", bmw.color, bmw.weight);

        }

        // try some enums
        {

            enum TwoValues {
                First(i8),
                Second(f32),
            }

            fn test_enum(some_number: i32) -> TwoValues {

                if some_number > 0 {
                    TwoValues::First(120)
                }
                else {
                    TwoValues::Second(3.09)
                }

            }

            match test_enum(-2) {
                TwoValues::First(value) => println!("TwoValues enum: {}", value),
                TwoValues::Second(value) => println!("TwoValues enum: {}", value),
            }

            let two_values_instance = TwoValues::First(9);
            println!("TwoValues enum: size of a variant {}", std::mem::size_of_val(&two_values_instance));

        }

        // closures
        {
            let say = |phrase: String| {println!("{}", phrase)};
            say("Hello, closure!".to_string()); // this is probably worse than just println!

            fn mutate_integer<F: Fn(i32) -> i32>(value: i32, mutator: F) -> i32 {
                mutator(value)
            }

            let modulo_three = |v: i32| {v % 3};

            let value: i32 = 23;
            println!("Closures: input value = {}, result = {}", value, mutate_integer(value, modulo_three));

            let add_five = |v: u32| {v + 5};
            let mul_ten = |v: u32| {v * 10};

            fn mutate_integer_2<F, G>(value: u32, mutator_1: F, mutator_2: G) -> u32
                where F: Fn(u32) -> u32, G: Fn(u32) -> u32 {
                
                mutator_2(mutator_1(value))

            }

            let value_2: u32 = 7194682;
            println!("Closures #2: input value = {}, result = {}", value_2, mutate_integer_2(value_2, add_five, mul_ten));
        }      

        // threads
        {
            use std::io;
            use std::thread;
            use std::sync::mpsc;
            
            // without a guard
            thread::spawn(|| {
                println!("Very first thread EEEEEEYYYYYEEEECCCCAAAAAAATTTCCCHHHEEEERRR!!!");
            });
            thread::sleep_ms(50); // we need it because main terminates and kills all the threads
            
            // with a guard (got unstable because of the memory leak issue #24292)
            /*
            let thread_guard: thread::JoinGuard<()> = thread::scoped(|| {
                println!("Thread with a guard here!!");
            });
            println!("Thread guard: {:?}", thread_guard.thread().name());
            */

            // another example
            for i in 0..16 {

                thread::spawn(move || {
                    println!("Thread #{} started", i);
                    thread::sleep_ms(500 % (i + 1));
                    println!("Thread #{} finished", i);
                });

            }
            thread::sleep_ms(2500);

            // channels

            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                for _ in 10..20 {
                    let mut read_buffer: String = String::new();
                    io::stdin().read_line(&mut read_buffer).ok().expect("failed to read line");
                    tx.send(read_buffer).unwrap();
                }
            });

            for _ in 0..5 {
                println!("Got value {:?} from another thread", rx.recv().unwrap());
            }
            
        }
}

// a C-like struct
struct Car {
    color: String,
    weight: u32,
}

// a tuple struct s
struct RgbColor(i8, i8, i8);

// a unit struct
struct Nil;

fn add_two_integers(a: i32, b: i32) -> i32 {
    a + b
}
