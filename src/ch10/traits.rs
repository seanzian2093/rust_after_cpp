/// # Ch10.2 - Traits: Defining Shared Behavior
/// * a trait defines functionality a particular type has and can share with other types
/// * we use traits to define shared behavior in an abstract way
/// * `trait bounds` to specify that a generic type can be any type that has certain behavior
/// * peers in other language - interfaces

use std::fmt::{Debug, Display};
#[allow(unused_mut)]
#[derive(Debug)]
pub struct Traits{
}

impl Traits{
    pub fn print(&self) {
        println!("\n======The note on traits======");
    // Define a Trait
        // - abstract, i.e., just function signature, not function body
            // - we can define default implementation
        trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }
        // - a struct that will implement `Summary`
        struct NewsArticle {
            headline: String,
            location: String,
            author: String,
            content: String,
        }
            // - implement `Summary` for `NewsArticle` in concrete way
        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }
        
        // - another struct that will implement `Summary`
        #[derive(Debug)]
        struct Tweet {
            username: String,
            content: String,
            reply: bool,
            retweet: bool,
        }
        
            // - implement `Summary` for `NewsArticle` in concrete way
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        // - Third struct that will implement default `Summary`
        struct Page {
            username: String,
            content: String,
        }

        impl Summary for Page {
            // empty block for default implementation
        }
    // Use a Trait
        // - `Tweet`
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize());

        // - `NewsArticle`
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        };
        println!("New article available! {}", article.summarize());

        // - `Page`
        let page = Page{
            username: String::from("netizen"),
            content: String::from("The page you requested is not found")
        };
        println!("New page available! {}", page.summarize());
    
    // Default implementation can call other methods in the trait
        // - even a method that has no default implementation
        trait Summary2 {
            fn summarize_author(&self) -> String;
        
            fn summarize2(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }
        // - implement for tweet
        impl Summary2 for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }
        println!("\n1 new tweet: {}", tweet.summarize2());
    
    // Trait as Parameter
        // - use traits to define functions that accept many different types
            // - e.g., `notify` accepts any time that implements `Summary`
            // - within body of `notify` we can use any methods that implemented in `Summary` 
        fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
        println!("");
        notify(&page);
        notify(&tweet);
        notify(&article);
        
        // - Trait Bound Syntax
            // - above `notify` uses a syntax sugar for `trait bound`
            // - e.g., `notify` could be defined as
        fn notify2<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize());
        }
        println!("");
        notify2(&page);
        notify2(&tweet);
        notify2(&article);
        
            // - specifying multiple traits bouds with + syntax
                // - e.g., `notify3` accepts those types that implement `Summary` and `Debug`
                // - only `Tweet` implements both
        fn notify3<T: Summary + Debug>(item: &T) {
            println!("\nBreaking news! {}", item.summarize());
        }
        println!("");
        notify3(&tweet);
                // - will not compile if we call `notify3` on page or article
        // notify3(&page);
        // notify3(&article);

            // - clearer trait bounds with where clauses
                // - e.g. below function can be rewritten with where clauses
        fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {0}
                // - e.g., with where clause
        fn some_function2<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {0}

    // Returning Types that Implement Traits
        // - we can also use `impl Trait` syntax in the return position to return a value of type that implement a trait
        // - below function returns a type that implement `Summary`
        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            }
        }
        // - this works only if we are returning a single type
            // - e.g., below function will not compile because it reutrns either `NewsArticle` or `Tweet`

        // fn returns_summarizables(switch: bool) -> impl Summary {
        //     if switch {
        //         NewsArticle {
        //             headline: String::from(
        //                 "Penguins win the Stanley Cup Championship!",
        //             ),
        //             location: String::from("Pittsburgh, PA, USA"),
        //             author: String::from("Iceburgh"),
        //             content: String::from(
        //                 "The Pittsburgh Penguins once again are the best \
        //                  hockey team in the NHL.",
        //             ),
        //         }
        //     } else {
        //         Tweet {
        //             username: String::from("horse_ebooks"),
        //             content: String::from(
        //                 "of course, as you probably already know, people",
        //             ),
        //             reply: false,
        //             retweet: false,
        //         }
        //     }
        // }

    // Use Trait Bound to conditionally Implement Methods
        struct Pair<T> {
            x: T,
            y: T,
        }
        // - implement .new method for any type
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        // - implement .cmp_display method only for types that implement Display and PartialOrd
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }
        // Quiz - below program does not compile
            // - because `displayable` returns a type that implements Display
            // - so s2 does not have .push_str method
            // - it would compile if return type is T 
        fn displayable<T: Display>(t: T) -> impl Display { t }
        let s = String::from("hello");
        let mut s2 = displayable(s);
        // s2.push_str(" world");
        println!("{s2}");

    }
}