// Ch3.5 - Control Flow
#[derive(Debug)]
pub struct ControlFlow{
}
    /// # if expression
    /// * condition must be a bool type
    /// * otherwise compiler will raise error
    /// * compiler does not automatically conver other types to a bool type
    /// * e.g. below expression will cause error
    /// ```
    /// let number = 3;
    /// if number {
    ///     println!("number was three");
    /// }
    /// ```

impl ControlFlow{
#[allow(unused_variables)]
    pub fn print(&self) {
        println!("\n======The note of control flow======");
    // if expression
        let number = 3;
    
        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }

        // - condition must be a bool type
            // - otherwise compiler will raise error
            // - compiler does not automatically conver other types to a bool type
            // - e.g. below expression will cause error
        // if number {
        //     println!("number was three");
        // }

        // - use if in let statement
            // - becase if is expression
        let condition = true;
            // - in the {}, there must be expressions, i.e. return values
        let number = if condition { 5 } else { 6 };
        println!("The value of if expression is: {number}");
            // - in below if expression, no value will be assigned to number
        let _number = if condition { 5; } else { 6; };
        println!("The value of if expression is: {:?}", _number);
            // - return value types in branches must be the same
                // - .e.g, below statements will compile
        // let _number = if condition { 5 } else { "six" };

    // Repeatition with loops
        // loop
            // - expression
            // - repeat foever untill explicitly stopped
            // - use `break` to go out of loop
                // - add expression whose resultant value we want to return
                    // - as argument to `break` expression.
                // - semicolon in the `break` statement is optional
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is expected to be 20: {result}");
            // - use `continue` to discard remaining code in current iteration and go on to next
            // - use loop labels to disambiguate multiple lopps
                // - by default `break` and `continue` apply to innermost loop
        let mut count = 0;
                // - label outer loop as `conting_up`
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
    
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                // - break from loop specified by `counting_up`
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
        println!("End count = {count}");

        // Conditional loop with `while`
        let mut number = 3;
        while number != 0 {
            println!("{number}!");
            number -= 1;
        }
        println!("\nLIFTOFF!!!");
        // Loop over a collection
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is {element}");
        }
            // - use range as the collection
                // - (1..4) means [1,3) in math
                // - .rev() change the range to reverse order
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}
