// Ch3.2 - Data Types
#[derive(Debug)]
pub struct DataTypes {
}
impl DataTypes {
#[allow(unused_variables)]

    pub fn print(&self) {
        println!("\n======The note of data types======");
        // Integer
            // - default to i32
            // - signed vs unsigned
        let _i0= 1;
        let _i1: i64 = 1;

        // Float
            // - default to f64
            // - f32 is single precision, f64 is double, almost same performance
            // - signed
        let _x = 2.0;
        let _y: f32 = 3.0;

        // Boolean
        let _t = true;
        let _f: bool = false;

        // Char
            // - in single quotes, as opposed to string literal in double quotes
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';

        // Tuple
            // - elements may be of differing types
            // - fixed length, no growth or shrinkage after declaration
            // - compiler can infer the types
        let tup: (i32, f64, u8) = (500, 6.4, 1);
            // - or we can declare types explicitly
        let tup = (500, 6.4, 1);
            // - Unpack a tuple
                // - tup is not destructed after unpacking
        let (x, y, z) = tup;
        println!("\nThe value of y is: {y}");
        println!("The values of tup is: {:?}", tup);
        let x: (i32, f64, u8) = (500, 6.4, 1);
            // - use dot(.) operator to access individual elements
                // - read or write/mutate
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;

                // - tup must be mutable
        let mut x: (i32, i32) = (1, 2);
        println!("\nThe values of x is: {:?}", x);
        x.0 = 0;
        x.1 += 5;
        println!("After mutation, the values of x is: {:?}", x);

        // Array
            // - fixed length, unlike in c++
            // - same element type
                // - compiler can infer type
        let a = [1, 2, 3, 4, 5];
                // - we can also annotate explicitly
        let a: [i32; 5] = [1, 2, 3, 4, 5];
            // - syntax sugar for initialize array of same values for each element
                // - [3; 5] equivalent to [3,3,3,3,5]
            // - stored in a single chunk of memory in stack
            // - access element using indexing
                // - start at 0
                // - if index out of range, program will panic at runtime
                    // - compiler will check and raise error if it can
        let a = [3; 5];
        let first = a[0];
        let second = a[1];
                    // - e.g., compiler will raise error for below statement 
        // let to_panic = a[10];

    }
} 