//! # Ch19.2 - Advanced Traits
//!     * Associated types connect a type placeholder with a trait such that
//!         * the trait method definitions can use these placeholder types in their signatures.
//!         * implementor of a trait will specify the concrete type to be used instead of the placeholder type
//!         * so we can define a trait that uses some types without needing to know exactly what they are until implementation
//!     * Default Generic Type Parameters and Operator Overloading
//!     * Fully Qualified Syntax for Disambiguation: calling methods with same name
//!     * Using Supertraits to Require One Trait's Functionality Within Another trait
//!         * a trait that our trait definition is relying on is called supertrait of our trait
//!     * Using the Newtype Pattern to Implement External Traits on External Types
//!         * orphan rule that we are only allowed to implement a trait on a type if either the triat or type are local to our crate
//!         * using newtype pattern to get around this restriction, i.e. a tupe struct with one field as wrapper around the target type

use std::fmt;
use std::ops::Add;

// Use associated type in trait definiton and implementation
trait MyIterator {
    // - `Item` is a placeholder
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

// - when implement `MyIterator` for `Counter`, we can use `Item` as if it is a concrete type
// - after assign a concrete type to `Item`

// impl MyIterator for Counter {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {}
// }

// Implementing the `Add` trait to overload `+` operator for `Point` instances
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// - implement `Add` trait where operands type are different
struct Millimeters(u32);
struct Meters(u32);

// - definition of `Millimeters` + `Meters`
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Using fully qualified name to avoid ambiguity
trait Pilot {
    fn fly(&self);
    fn name() -> String;
}

trait Wizard {
    fn fly(&self);
    fn name() -> String;
}

struct Human;

// - Tow traits that have `fly` method are implemented on `Human`
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
    fn name() -> String {
        String::from("Captain")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
    fn name() -> String {
        String::from("Master")
    }
}

// - a `fly` method is implemented on `HUman` directly
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
    fn name() -> String {
        String::from("Mr")
    }
}

// out trait `OutlinePrint` relies on `fmt::Display`
// - use `OutlinePrint: fmt::Display` to indicate that `OutlinePrint` only works for types that implement `Display`
// - similar to trait bound
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // - `to_string()` is a method in `Display` trait
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
// - implement `Display` on `Point`
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
// - then implement `OutlinePrint` on `Point` that now has `Display` trait
impl OutlinePrint for Point {}

// Define a tupe struct with one field
struct Wrapper(Vec<String>);
// - implement `Display` for Wrapper, essentially `Vec<String>` which is outside of our crate
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct AdvancedTraits {}

#[allow(unused)]
impl AdvancedTraits {
    pub fn print(&self) {
        println!("\n======The note on AdvancedTraits======");

        // Check `+` operator for `Point` instance
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );

        // when calling `.fly` on a `Human` instance, we must use fully qualified method name
        // - otherwise, Rust defaults to use the directly implemented methods, not those from traits
        let person = Human;
        // - methods that take `self` or `&self` as parameter
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);
        // - methods that do not take `self` or `&self` as parameter
        println!("person's Human name is: {}", Human::name());
        println!("person's Pilot name is: {}", <Human as Pilot>::name());
        println!("person's Wizard name is: {}", <Human as Wizard>::name());
        // Implement Display for Wrapper
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("\nw = {}", w);
    }
}
