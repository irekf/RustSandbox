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

// an animal struct
pub struct Animal<T> {
    pub age: u8,
    pub weight: u16,
    secret_info: T,


}

impl<T> Animal<T> {

    pub fn new(animal_age: u8, animal_weight: u16, animal_secret_info: T) -> Animal<T> {
        
        Animal {
            age: animal_age,
            weight: animal_weight,
            secret_info: animal_secret_info,
        }

    }

    pub fn get_secret_info(&self) -> &T {
        &self.secret_info
    }
}
