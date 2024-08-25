//! Ch17.1 - Using Trait Objects That Allow for Values of Different Types

// Create a public trait
#[allow(unused)]
pub trait Draw {
    fn draw(&self);
}

// Define a struct that use `Draw`
pub struct Screen {
    // components should be a vec ot types that implement `Draw` trait
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

    // - we could define `Screen` using generic type parameter
        // - but generic type parameter can be substitued by one concrete type, i.e.,
        // - elements in components must be of same concrete type
pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// A struct that implements `Draw`
pub struct  Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}


// Another struct that implements `Draw`
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}


#[derive(Debug)]
#[allow(unused)]
pub struct DynTraits{
}

#[allow(unused)]
impl DynTraits{
    pub fn print(&self) {
        println!("\n======The note on dyn traits======");
    // Using trait
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };
    
        screen.run();
    // Using trait
        // - in this situation, compile does not know components accepts any type that implement `Draw`
        // - because we are define components outside a `Screen` so compile loses that information
        // let components = vec![
        //     Box::new(SelectBox {
        //         width: 75,
        //         height: 10,
        //         options: vec![
        //             String::from("Yes"),
        //             String::from("Maybe"),
        //             String::from("No"),
        //         ],
        //     }),
        //     Box::new(Button {
        //         width: 50,
        //         height: 10,
        //         label: String::from("OK"),
        //     }),
        // ];

            // - remedy 1
        let components2 = vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }) as Box<dyn Draw>,
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ];

            // - remedy 2
        let components3: Vec<Box<dyn Draw>> = vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ];

        // let screen = Screen {components};
        // screen.run();

        let screen2 = Screen {components: components2};
        screen2.run();
        let screen3 = Screen {components: components3};
        screen3.run();
    }
}