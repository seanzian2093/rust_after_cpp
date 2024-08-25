// Ch3.1 - Variable and Mutability    
const ONE_HOUR_IN_SECONDS: u32 = 60 * 60 * 1;

#[derive(Debug)]
pub struct VarMutability {
}

impl VarMutability {

    pub fn print(&self) {

        println!("\n======The note of varible and mutability======");
        // By default a var is imutable 
        // let x = 5;
            // - unliess explicitly make it
        let mut x = 5;
        println!("\nThe value of x is: {x}");
        x = 6;
            // - but x's type is not immutable
        // x = "x";
        println!("The value of x is: {x}");

        // Constant is not just immutable
            // - use `const` key word for declaration and definition
            // - must be always annotated
            // - can be declared in any scope, including global
            // - must be set to const expression
                // - not result from expression that is evaluated at run-time
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        println!("\nThere are: {THREE_HOURS_IN_SECONDS} seconds in three hours.");
        const TWO_HOURS_IN_SECONDS: u32 = ONE_HOUR_IN_SECONDS * 2;
        println!("There are: {TWO_HOURS_IN_SECONDS} seconds in two hours.");

        // Shadowing
            // - declaring a new var with same name shadows the previously defined one
                // - i.e. compiler only sees the newly decalred one
        let y = 5;
                // y with value of 5 is shadowed by y = y+ 1
        let y = y + 1;
        {
                // y with value of 6 is shadowed by y = y * 1
            let y = y * 2;
            println!("\nThe value of y in the inner scope is: {y}");
                // y in this inner scope ceases to exist after this point
        }
        println!("The value of y is: {y}");
            // - we can shadow with a different type
        let spaces = "   ";
        println!("\nThe value of spaces is: {spaces}");
        let spaces = spaces.len();
        println!("The value of spaces after shadowing is: {spaces}");

    }
}