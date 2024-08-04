# Declarative generation of enum dispatch


[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/declarative_enum_dispatch.svg
[crates-url]: https://crates.io/crates/declarative_enum_dispatch
[docs-badge]: https://docs.rs/declarative_enum_dispatch/badge.svg
[docs-url]: https://docs.rs/declarative_enum_dispatch
Generate boilerplate code for dynamic dispatch of a trait using an enum.
Also generates From for every enum variant

This is a fully declarative version of [enum_dispatch](https://docs.rs/enum_dispatch) macro

For benchmarks look at [enum_dispatch benchmarks](https://docs.rs/enum_dispatch/latest/enum_dispatch/#performance) crate
```rust
use declarative_enum_dispatch::enum_dispatch;

enum_dispatch!(
    /// Supports trait inheritance + lifetime (although single and after traits)
    pub trait ShapeTrait: Clone + std::fmt::Debug + 'static {
        /// No return + default implementation
        fn print_name(&self) {
            println!("name: `{}`", self.name());
        }
        /// Basic call without arguments
        fn name(&self) -> String;
        fn area(&self) -> i32;

        /// Mutable self + arguments
        fn grow(&mut self, numerator: i32, denominator: i32);

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
        Cube(Cube)
    }
);

#[derive(Debug, Clone)]
pub struct Rect{ w: i32, h: i32 }

#[derive(Debug, Clone)]
pub struct Circle { r: i32 }

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

    fn grow(&mut self, numerator: i32, denominator: i32 ) {
        self.r = self.r * numerator / denominator;
    }

    fn greater(&self, other: &impl ShapeTrait) -> bool {
        self.area() > other.area()
    }

    async fn send(&self) {}
}


assert_eq!(Shape::Rect(Rect { w: 1.0, h: 1.0 }).name(), "Rect".to_string());
assert_eq!(Shape::Circle(Circle { r: 1.0 }).name(), "Circle".to_string());
```
## Roadmap
- [ ] Support generic params
- [ ] Support lifetimes
- [x] Support trait inheritance

## Why?
Because I can... Well... RustRover indexing doesn't work with enum dispatch and in one of the threads about this problem I've read 

> enum_dispatch is a rare example of absolutely IDE-unfriendly macros. It breaks every imaginable rule.
> With current design, enum_dispatch will never be supported. ([source](https://github.com/intellij-rust/intellij-rust/issues/8813#issuecomment-1118761880))

So it got me wondering if it can be implemented using declarative macro for "perfect" IDE support, and so... it can)
### Yes, I am fixing crate to make it index correctly in my paid IDE.
## I WANT MY DAMN AUTOCOMPLETION