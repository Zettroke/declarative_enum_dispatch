# Declarative generation of enum dispatch


Generate boilerplate code for dynamic dispatch of a trait using an enum.
Also generates From for every enum variant

This is a fully declarative version of [enum_dispatch](https://docs.rs/enum_dispatch) macro

For benchmarks look at [enum_dispatch benchmarks](https://docs.rs/enum_dispatch/latest/enum_dispatch/#performance) crate
```rust
use declarative_enum_dispatch::enum_dispatch;

enum_dispatch!(
    pub trait ShapeTrait {
        /// No return + default implementation
        fn print_name(&self) {
            println!("name: `{}`", self.name());
        }
        /// Basic call without arguments
        fn name(&self) -> String;
        fn area(&self) -> f32;
        /// Mutable self + argument
        fn grow(&mut self, times: f32);
        /// Works with attributes
        #[cfg(feature = "platform_specific")]
        fn platform_specific(self);
    }
    pub enum Shape {
        Rect(Rect),
        Circle(Circle),
    }
);

pub struct Rect {
    w: f32,
    h: f32
}
pub struct Circle {
    r: f32
}
impl ShapeTrait for Rect {
    fn print_name(&self) {
        println!("rect name: `{}`", self.name());
    }
    fn name(&self) -> String {
        "Rect".to_string()
    }
    fn area(&self) -> f32 {
        self.w * self.h
    }
    fn grow(&mut self, times: f32) {
        self.w *= times;
        self.h *= times;
    }
}
impl ShapeTrait for Circle {
    fn name(&self) -> String {
        "Circle".to_string()
    }
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.r * self.r
    }
    fn grow(&mut self, times: f32) {
        self.r *= times;
    }
}

assert_eq!(Shape::Rect(Rect { w: 1.0, h: 1.0 }).name(), "Rect".to_string());
assert_eq!(Shape::Circle(Circle { r: 1.0 }).name(), "Circle".to_string());
```
