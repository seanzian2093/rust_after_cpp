/// # Ch13.2 - Processing a Series of Items with Iterators
/// * The iterator pattern allows you to perform some task on sequence of items in turn.
///     * An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished
///     * we use iterator so we don't have implement the logic ourself
/// * iterators are lazy, i.e., they have no effect until we call methods that consume the iterator to use it up
/// * All iterators implement a trait named `Iterator` which only requires implementors to define one method - `next` method
///     * which returns one item of the iterator at a time wrapped in `Some` and when iteration is over, returns `None`
/// * Methods that consumer the iterator
///     * the `Iterator` trait has a number of different methods with default implementations provided by standard library
///     * some of these methods call `next` method in their definition, and are called `consuming adaptor`
///         * because calling them uses up the iterator, e.g., `sum`
/// * Methods that Produces Other Iterators
///     * Iterator adaptors are methods defined on the `Iterator` trait that don't consume the iterator, instead
///     * produce different iterators by changing some aspects of the original iterator, e.g., `map`
///     * iterator adaptors are lazy, just as regular iterator
///     * Many iterator adaptors take closures as arguments, which commonly capture their environment, e.g., filer
#[derive(Debug)]
pub struct Iterators{
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

impl Iterators{
    pub fn print(&self) {
        println!("\n======The note on iterators======");
    // Create an iterator
        // - the code by itself does not do anything useful so far
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        // - consume the iterator
            // - v1_iter cannot be used after this for loop
        for val in v1_iter {
            println!("{val}");
        }
        println!("\nAfter for loop, v1 is {:?} ", v1);
    
    // Consume an iterator
        let v2: Vec<i32> = vec![4, 5, 6];
        let v2_iter = v2.iter();
        let total: i32 = v2_iter.sum();
        println!("\nsum of {:?} is {}", v2, total);

    // Iterator adaptor
        let v3: Vec<i32> = vec![7, 8, 9];
        // - iterator adaptor is lazy, so far it does nothing so v3 looks like unchagned
        let v3_map = v3.iter().map(|x| x + 1);
        println!("\nAfter map, v3 is: {:?}", v3);
            // - when collect(), we may need to explicit annotatation
        let v4: Vec<i32> = v3_map.collect();
        println!("After collect, v4 is: {:?}", v4);
        println!("After collect, v3 is: {:?}", v3);

        // - taking closures as argument
            // - closures may capture their environment
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let shoe_size = 10;
        // - closure provided to `filter` takes ownership of s but shoe_size is copied because it is i32
        let in_my_size: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == shoe_size).collect();
        println!("\nShoes in my size {} is {:?}", shoe_size, in_my_size);

    // Quiz
    let v = vec![1, 2, 3, 4];
        // - `.iter()` produces an `Iterator<Item = &i32>`

        // - `.filter()` takes `Iterator<Item = T>` as input and passes `&T` to its predicate
            // - therefor in closure, `x:&&i32` as input
        // - Rust standard library implements remainder operator `%` for `&i32` on the left side but not for `&&i32`, 
            // - so need to deref, i.e., `*x`, to `&i32`, to use it in `*x % 2`

        // - `.map()` takes `Iterator<Item = T>` as input, and passes `T` to is closure
            // - therefore in its closure, `&i32` as input
        // - Rust implements remainder operator `%` and multiplication operator `*` for `&i32`
            // - so no need to deref, i.e., `*x`


    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();
    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();
    println!("\nAfter above operations, a is {:?}, b is {:?}", a, b);

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

}