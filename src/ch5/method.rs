/// # Ch5.2 - Method Syntax
#[derive(Debug)]
pub struct Method{
}

    // Define a struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, Copy, Clone)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

    // Define a method for the struct
        // - everything in this impl block will be associated with REctangle
        // - there could be multiple impl blocks for each struct
        // - first parameter is always `self` which represents the instance of the struct the method is being called
            // - method can take ownershipt of self, or borrow mutably, or immutably
        // - could have same name as struct field
    // Associated Functions
        // - all functions defined within an `impl` block are called associated functions because they are associated with the type named after `impl`
            // - we can define associated functions that don't have `self` as first parameter
            // - they are not method since they don't need an instance to work with, .e.g, String::from()
    // Methods Calls are Syntactic Sugar for Function Calls
        // - Rust automatically reference and dereference the method receiver when use the dot operator
            // - no such operator as r->area(), as in c++ and c
    // Methods and Ownership
        // - methods must be called on structs that have necessary permissions.
        // - in some situation, it is safe to move out of `*self` even though Rust does not let us do it
            // - in Recaangle case, Rectangle does not own heap data
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

        // Self keyword is aliases for the Struct type 
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // - set_width requires write permission
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // - max takes ownership of self and other, i.e., move other
        // - other is not usable after max
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        // self.max() take ownership of self, and moves `*self`
            // - because copy or clone is not implemented in Rectangle
        // let max = self.max(other);
        // so *self is not usable here
        // *self = max;
    }
}

impl Rectangle2 {
    fn max(self, other: Rectangle2) -> Rectangle2 {
        Rectangle2 {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle2) {
        // self.max() take ownership of self, and moves `*self`
            // - because copy or clone is implemented in Rectangle2
        // so *self is usable here
        *self = self.max(other);
    }
}

impl Method{
    pub fn print(&self) {
        println!("\n======The note of method syntax======");
        
    // - Instantiate a instance and call its method
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };

        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("The area of the rectangle is {} square pixels", rect1.area());
        println!("The withd of the rectangle is {}, at {}", rect1.width(), rect1.width);
        println!("Can rect1 hold rect2? - {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? - {}", rect1.can_hold(&rect3));

        // - Method call is a short hand calling associated function
        let area1 = rect1.area();
        let area2 = Rectangle::area(&rect1);
        assert_eq!(area1, area2);

        let max_rect = rect1.max(rect2);
        println!("max(rect1, rect2) - {:?}", max_rect);
        // - rect2 and rect1 are not usable after max
        // println!("After max(rect1, rect2), rect2 is - {:?}", rect2);
        // println!("After max(rect1, rect2), rect1 is - {:?}", rect1);

        // - can not call set_width() on rect3 due to lack of write permission - rect3 is immutable
        // rect3.set_width(77);

        let mut rect4 = Rectangle {
            width: 0,
            height: 1,
        };
        rect4.set_width(77);
        println!("After set_width, rect4 is: {:#?}", rect4);

        let mut r1 = Rectangle2 {
            width: 9,
            height: 9,
        };
        let r2 = Rectangle2 {
            width: 11,
            height: 11,
        };
        r1.set_to_max(r2);
        println!("\nAfter set_to_max, r1 is: {:#?}", r1);
        println!("After set_to_max, r2 is: {:#?}", r2);
    }
}
