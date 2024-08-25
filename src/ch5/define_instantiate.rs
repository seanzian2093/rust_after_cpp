/// # Ch5.1 - Defining and Instantiating Struct
#[derive(Debug)]
pub struct DefineInstantiate{
}

/// Like tuple, a struct can hold multiple related values of different types
/// Unlike tuple, a struct's values have names.

    // Define a Struct
        // - use #[derive[Debug]] for println1()
#[derive(Debug)]
struct User {
    active: bool,
    username:String,
    email: String,
    sign_in_count: u64,
}
impl DefineInstantiate{
    pub fn print(&self) {
        println!("\n======The note of Define and Instantiate======");
    // Instantiate a User
        // - directly
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        println!("\n user1 is an instance of User: {:#?}", user1);
        // - or indirectly
        fn build_user(email: String, username: String) -> User {
            User {
                active: true,
                username: username,
                email: email,
                sign_in_count: 1,
            }
        }
        let mut user2 = build_user("email".into(), "username".into());        
        user2.email = "someemail@somecom".into();
        println!("user2 is an instance of User: {:#?}", user2);

    // Using Field Init Shorthand 
        // - when parameter and struct field names are exactly the same
        fn build_user2(email: String, username: String) -> User {
            User {
                active: true,
                username,
                email,
                sign_in_count: 1,
            }
        }
        let user3 = build_user2("email".into(), "username".into());        
        println!("user3 is an instance of User: {:#?}", user3);

    // Creating Instances from Other Instances with Struct Update Syntax
        // - create user4 from user1 with only differring email
        let user4 = User {
            username: "User4".into(),
            ..user1
        };
        println!("user4 is an instance of User: {:#?}", user4);

    // Using Tuple Sructs Without Named Fields to Create Different Types
        // -  no names associated with fields
        // - just types
        #[derive(Debug)]
        struct Color(i32, i32, i32);
        #[derive(Debug)]
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        println!("\nblack is an instance of Color: {:#?}", black);
        let origin = Point(0, 0, 0);
        println!("origin is an instance of Point: {:#?}", origin);

    // Borrowing Fields of a Struct
    struct Point2 { x: i32, y: i32 }
    
    let mut p = Point2 { x: 0, y: 0 };
        // - x borrows p.x so p and p.x lost permissions
        // - but p.y is not impacted
    let x = &mut p.x;
    *x += 1;
    println!("{}, {}", p.x, p.y);
    }
}
