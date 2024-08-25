// Ch3.3 - Functions
#[derive(Debug)]
pub struct Functions{
}
impl Functions{
#[allow(unused_variables)]
    // Function 
        // - function name should be snake case, i.e., lower case, with underscore connecting words
        // - must declare parameter type
    pub fn print(&self) {
        println!("\n======The note of functions======");

        print_labeled_measurement(5, 'h');
        
        fn print_labeled_measurement(value: i32, unit_label: char) {
            println!("\nThe measurement is: {value}{unit_label}");
        }
    // Statement and expression
        // - statements are instructions that perform some action and do not return a value
        // - expressions evaluate to resultant values
            // - expression does not end with semicolon
            // - we add a semicolon, it becomes a statement

        // - statement examples
            // - a let statement
                // - e.g. below statement will cause error in compile time
                    // - because a let statement does not return a value,thus nothing to assign to x
        // let x = (let y = 6);
            // - a function definition

        // - expresion examples
            // - literals are expression
            // - calling functions
            // - calling macros
            // - a new scope block that is created with curly brackets
                // - return value is the resultant value of last expression
                    // - e.g. below {} evalues to 4 and is assigned to y
            let y = {
                let x = 3;
                x + 1
            };
            println!("\nThe value of y is: {y}");
    
    // Functions with return value
        // five has no parameter, return 5 which is the last expression.
        fn five() -> i32 {5}
        let x = five();
        println!("\nThe value returned by five() is: {x}");

        let x = plus_one(5);
        println!("The value returned by plus_one(5) is: {x}");
        
        fn plus_one(x: i32) -> i32 {
            // this is an expression which determines return value
            x + 1
            // this is a statment, return no value, so will the function
            // x +1;
        }
    }
}