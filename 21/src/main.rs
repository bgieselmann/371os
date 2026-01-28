fn main() {
    unsafe {
        let hello_world_chars: [i32;12] = [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
        let mut hello_world_str = String::from("");

        for n in 0..12 {
            hello_world_str.push(std::mem::transmute(hello_world_chars[n]));
        }
        println!("{}", hello_world_str);
    }

}


