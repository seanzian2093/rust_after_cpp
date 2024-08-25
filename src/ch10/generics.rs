/// # Ch10.1 - Generic Data Types

use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct Generics{
}

impl Generics{
    pub fn print(&self) {
        println!("\n======The note on generic data types======");
    // In Function Definitions
        // - use generics where we specifying types of parameters and return value
        // - we need to restrict generic types, discussed in ch10.2
            // - e.g. we restrict `largest` to those types that implement `PartialOrd` from `std::cmp` library
        fn largest<T: PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];
        
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
        
            largest
        }
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("\nThe largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);

    // In Struct Definitions
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }
        
        let integer = Point { x: 5, y: 10 };
        println!("\ninteger is {:?}", integer);
        let float = Point { x: 1.0, y: 4.0 };
        println!("float is {:?}", float);
    
    // In Enum Definitions
        // - Option
        enum Option<T> {
            Some(T),
            None,
        }
        // - Result
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    
    // In method Definition
        // - e.g. to define a method `x` for `Point`
        // - we declare `T` immediately after keyword `impl` as a generic type
        // - so in later `Point<T>`, `T` will be treated as a generic type, not a concrete one
        // - we could use other name but use the same as in `Point` definition is conventional
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        println!("\ninteger.x() is {}", integer.x());
        println!("float.x() is {}", float.x());
        // - we could implement a method to a specific concrete type only, e.g., f32
            // - we do declare generic type after `impl`
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        println!("float.distance_from_origin() is {}", float.distance_from_origin());
        // - in struct definition, we may need to use different generic types for fields 
        struct Point2<X1, Y1> {
            x: X1,
            y: Y1,
        }
        
        impl<X1, Y1> Point2<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
                Point2 {
                    x: self.x,
                    y: other.y,
                }
            }
        }
        
        let p1 = Point2 { x: 5, y: 10.4 };
        let p2 = Point2 { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);
        println!("\np3.x = {}, p3.y = {}", p3.x, p3.y);

    }
}
