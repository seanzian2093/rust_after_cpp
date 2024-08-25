/// # Ch6.2 - Match Control Flow Construct
#[derive(Debug)]
pub struct MatchControlFlow{
}

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Power of Match Control Flow Construct
    // - expresiveness of patterns
    // - compiler confirms that all possible cases are handled

impl MatchControlFlow{
    pub fn print(&self) {
        println!("\n======The note of MatchControlFlow======");
    // Match with Pattern
        // - variable after `match` keyword can be any time, as opposed to bool to `if`
        // - each arm consists of pattern and code to run, separated by `=>` operator
            // - code could be in a curly brackets block. make sure it returns a valid value
        // - pattern can be cound to a value for later use
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Luck Penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                // when Coin::Quarter matches, the state variable will bind to the value of that quarter's state
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}", state);
                    25
                }
            }
        }
        value_in_cents(Coin::Dime);
        value_in_cents(Coin::Penny);
        value_in_cents(Coin::Quarter(UsState::Alabama));
    // Match with Option<T>
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        println!("\nsix is: {:?}", six);
        let none = plus_one(None);
        println!("none is: {:?}", none);

    // Matches are exhaustive
        // - the last arm, other arm handles all cases that are not 3 or 7
        // - _ is a special pattern that matches any but does not bind patterns, i.e., discard pattern
        // - use unit value(empty tuple type) to do nothing
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            // - must only one of either
            other => move_player(other),
            // _ => (),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {println!("Move {num_spaces} steps");}

    // How Match interacts with Ownership
        let opt: Option<String> = Some(String::from("Hello world"));

        // - match String and does not bind
        fn match_string(opt: Option<String>){
            match opt {
                // - do not bind pattern
                Some(_) => println!("\nMatched Some arm"),
                None => println!("Matched None arm")
            };
            println!("At the end of match_string(opt), opt is: {:?}", opt);
        }
            // - match_string takes ownership of opt
        match_string(opt);
            // - so opt is unusable
        // println!("After match_string(opt), opt is: {:?}", opt);

        // - match String and bind to a variable
        fn _match_string_bind(opt: Option<String>){
            match opt {
                // bind pattern to s
                Some(s) => println!("\ns is:{s}, matched Some arm"),
                None => println!("Matched None arm")
            };
                // - opt is not usable since s takes ownership
            // println!("After match_string(opt), opt is: {:?}", opt);
        }

        // - match String ref and bind
        fn match_string_ref_bind(opt: &Option<String>){
            match opt {
                // bind pattern to s
                Some(s) => println!("\ns is:{s}, matched Some arm"),
                None => println!("Matched None arm")
            };
                // - opt is not usable since s takes ownership
            println!("At the end of match_string_ref_bind(opt), opt is: {:?}", opt);
        }
        let opt: Option<String> = Some(String::from("Hello world"));
        match_string_ref_bind(&opt);
        println!("After match_string_ref_bind(opt), opt is: {:?}", opt);
    
    // Match Construct is an Expression
        #[allow(dead_code)]
        enum Location {
            Point(i32),
            Range(i32, i32)
        }
        let l: Location = Location::Range(0, 5);
        let n = match l {
          Location::Point(_) => -1,
          Location::Range(_, n) => n,
          Location::Range(0, _) => 0,
          _ => -2
        };
        println!("Matching l returns: {n}");
    
    // if let statement and two-arm match
        let mut count = 0;

        let coin1 = Coin::Quarter(UsState::Alaska);
        let coin2 = Coin::Penny;

        fn match_coin(coin: &Coin, count: &mut i32) {
            match coin {
                Coin::Quarter(state) => println!("\nState quarter from {:?}!", state),
                _ => *count += 1,
            }
        }
        match_coin(&coin1, &mut count);
        match_coin(&coin2, &mut count);
        // - above match construct is equivalent to below `if let` statement
        fn match_coin2(coin: &Coin, count: &mut i32) {
            if let Coin::Quarter(state) = coin {
                println!("State quarter from {:?}!", state);
            } else {
                *count += 1;
            }
        }
        match_coin2(&coin1, &mut count);
        match_coin2(&coin2, &mut count);
    
        println!("Now count is: {count}");

    }
}
