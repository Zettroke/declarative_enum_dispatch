use declarative_enum_dispatch::enum_dispatch;

enum_dispatch!(
    pub trait ShapeTrait: Clone + std::fmt::Debug + 'static {
        /// No return + default implementation
        fn print_name(&self) {
            println!("name: `{}`", self.name());
        }
        /// Basic call without arguments
        fn name(&self) -> String;
        fn area(&self) -> i32;

        /// Mutable self + arguments
        fn grow(&mut self, numerator: i32, denominator: i32,);

        /// Kinda supports generics :) Bot not generic parameters, only `impl Trait`
        fn greater(&self, other: &impl ShapeTrait) -> bool;

        /// Supports async methods
        async fn send(&self);

        /// Works with attributes
        #[cfg(feature = "platform_specific")]
        fn platform_specific(self);
    }

    #[derive(Debug, Clone)]
    pub enum Shape {
        Rect(Rect),
        Circle(Circle),
        #[cfg(feature = "platform_specific")]
        Cube(Cube),
    }
);

#[derive(Debug, Clone)]
pub struct Rect {
    w: i32,
    h: i32,
}

#[derive(Debug, Clone)]
pub struct Circle {
    r: i32,
}

impl ShapeTrait for Rect {
    fn print_name(&self) {
        println!("rect name: `{}`", self.name());
    }
    fn name(&self) -> String {
        "Rect".to_string()
    }

    fn area(&self) -> i32 {
        self.w * self.h
    }

    fn grow(&mut self, numerator: i32, denominator: i32) {
        self.w = self.w * numerator / denominator;
        self.h = self.h * numerator / denominator;
    }

    fn greater(&self, other: &impl ShapeTrait) -> bool {
        self.area() > other.area()
    }

    async fn send(&self) {}
}

impl ShapeTrait for Circle {
    fn name(&self) -> String {
        "Circle".to_string()
    }

    fn area(&self) -> i32 {
        // close enough PI approximation :)
        3 * self.r * self.r
    }

    fn grow(&mut self, numerator: i32, denominator: i32) {
        self.r = self.r * numerator / denominator;
    }

    fn greater(&self, other: &impl ShapeTrait) -> bool {
        self.area() > other.area()
    }

    async fn send(&self) {}
}

fn main() {}
