// Ch6.1 - Define an Enum
#[derive(Debug)]
pub struct DefineEnum {
}

impl DefineEnum {
    pub fn print(&self) {
        println!("\n======The note of define an enum ======");
    // Enum Values
        // - we can put data directly into each enum variant
        // - each variant can have different types
        #[derive(Debug)]
        #[allow(dead_code)]
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr::V4(127, 0, 0, 1);
        println!("\nhome is: {:?}", home);
        let loopback = IpAddr::V6(String::from("::1"));
        println!("loopback is: {:?}", loopback);

    // Methods on Enum
        // - use `impl` block, similar to struct
        #[derive(Debug)]
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                println!("Calling from Message::call().")
            }
        }
        let m = Message::Write("hello".into());
        m.call();
    
    // Option Enum
        // - Rust does not have nulls
        // - Option<T> is an Enum that encode concept of a value being present or absent 
        // - Option<T> is defined as follow

        // enum Option<T> {
        //     None,
        //     Some(T),
        // }
        let _some_number = Some(5);
        let _some_char = Some('e');
        
        let _absent_number: Option<i32> = None;
    }
}
