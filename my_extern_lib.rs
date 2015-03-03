#![crate_type = "lib"]

#[cfg(target_os = "linux")]
pub fn run(a : u32) {
    for i in 0..a {
        println!("my_extern_lib from Linux: iteration #{}", i);
    }
}

#[cfg(not(target_os = "linux"))]
pub fn run(a : u32) {
    for i in 0..a {
        println!("my_extern_lib from Non-Linux OS: iteration #{}", i);
    }
}
