/*!
# Declarative generation of enum_dispatch

Usage example:
```
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

## Macro expansion
Expansion of the macro above
```no_run
use declarative_enum_dispatch::enum_dispatch;
pub trait ShapeTrait {
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
    /// Works with attributes
    #[cfg(feature = "platform_specific")]
    fn platform_specific(self);
}
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
    #[cfg(feature = "platform_specific")]
    Cube(Cube),
}
impl ShapeTrait for Shape {
    /// No return + default implementation
    fn print_name(&self) {
        match self {
            Shape::Rect(v) => v.print_name(),
            Shape::Circle(v) => v.print_name(),
            #[cfg(feature = "platform_specific")]
            Shape::Cube(v) => v.print_name(),
        }
    }
    /// Basic call without arguments
    fn name(&self) -> String {
        match self {
            Shape::Rect(v) => v.name(),
            Shape::Circle(v) => v.name(),
            #[cfg(feature = "platform_specific")]
            Shape::Cube(v) => v.name(),
        }
    }
    fn area(&self) -> i32 {
        match self {
            Shape::Rect(v) => v.area(),
            Shape::Circle(v) => v.area(),
            #[cfg(feature = "platform_specific")]
            Shape::Cube(v) => v.area(),
        }
    }
    /// Mutable self + arguments
    fn grow(&mut self, numerator: i32, denominator: i32) {
        match self {
            Shape::Rect(v) => v.grow(numerator, denominator),
            Shape::Circle(v) => v.grow(numerator, denominator),
            #[cfg(feature = "platform_specific")]
            Shape::Cube(v) => v.grow(numerator, denominator),
        }
    }
    /// Kinda supports generics :) Bot not generic parameters, only `impl Trait`
    fn greater(&self, other: &impl ShapeTrait) -> bool {
        match self {
            Shape::Rect(v) => v.greater(other),
            Shape::Circle(v) => v.greater(other),
            #[cfg(feature = "platform_specific")]
            Shape::Cube(v) => v.greater(other),
        }
    }
    /// Works with attributes
    #[cfg(feature = "platform_specific")]
    fn platform_specific(self) {
        match self {
            Shape::Rect(v) => v.platform_specific(),
            Shape::Circle(v) => v.platform_specific(),
            #[cfg(feature = "platform_specific")]
            Shape::Cube(v) => v.platform_specific(),
        }
    }
}
# pub struct Rect {
#     w: i32,
#     h: i32,
# }
# pub struct Circle {
#     r: i32,
# }
# impl ShapeTrait for Rect {
#     fn print_name(&self) {
#         println!("name: `{}`", self.name());
#     }
#     fn name(&self) -> String {
#         "Rect".to_string()
#     }
#     fn area(&self) -> i32 {
#         self.w * self.h
#     }
#     fn grow(&mut self, numerator: i32, denominator: i32) {
#         self.w = self.w * numerator / denominator;
#         self.h = self.h * numerator / denominator;
#     }
#     fn greater(&self, other: &impl ShapeTrait) -> bool {
#         self.area() > other.area()
#     }
# }
# impl ShapeTrait for Circle {
#     fn name(&self) -> String {
#         "Circle".to_string()
#     }
#     fn area(&self) -> i32 {
#         3 * self.r * self.r
#     }
#     fn grow(&mut self, numerator: i32, denominator: i32) {
#         self.r = self.r * numerator / denominator;
#     }
#     fn greater(&self, other: &impl ShapeTrait) -> bool {
#         self.area() > other.area()
#     }
# }

```

*/

#[macro_export]
#[doc(hidden)]
macro_rules! __build_method {
    ($(#[$attr:meta])* $method:ident; {$($self_ref:tt)*}; $self_:ident; { $($arg:ident: $arg_ty:ty),* }; $( -> $return_type:ty)?; [$($(#[$var_attr:meta])* $variant:ident),+]; $enum_name:ident) => {
        $(#[$attr])* fn $method($($self_ref)* $self_, $($arg: $arg_ty),*) $( -> $return_type)? {
            $crate::__build_method!(@make_match $self_; $method; $enum_name; [$($(#[$var_attr])* $variant),+]; ($($arg),*))
        }
    };

    (@make_match $self_:ident; $method:ident; $enum_name:ident; [$($(#[$var_attr:meta])* $variant:ident),+]; $args:tt) => {
        match $self_ {
            $(
                $(#[$var_attr])*
                $enum_name::$variant(v) => v.$method $args
            ),+
        }

    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __munch_methods {
    ({ }; [$($(#[$var_attr:meta])* $variant:ident),+]; $enum_name:ident) => {};

    // variants without block
    ({ $(#[$attr:meta])* fn $method:ident($self_:ident $(, $($arg:ident: $arg_ty:ty),*)? ) $( -> $return_type:ty)?; $($rest:tt)* }; [$($(#[$var_attr:meta])* $variant:ident),+]; $enum_name:ident) => {
        $crate::__build_method!($(#[$attr])* $method; { }; $self_; { $($($arg: $arg_ty),*)? }; $( -> $return_type)?; [$($(#[$var_attr])* $variant),+]; $enum_name);
        $crate::__munch_methods!({ $($rest)* }; [$($(#[$var_attr])* $variant),+]; $enum_name);
    };
    ({ $(#[$attr:meta])* fn $method:ident(&$self_:ident $(, $($arg:ident: $arg_ty:ty),*)? ) $( -> $return_type:ty)?;  $($rest:tt)* }; [$($(#[$var_attr:meta])* $variant:ident),+]; $enum_name:ident) => {
        $crate::__build_method!($(#[$attr])* $method; { & }; $self_; { $($($arg: $arg_ty),*)? }; $( -> $return_type)?; [$($(#[$var_attr])* $variant),+]; $enum_name);
        $crate::__munch_methods!({ $($rest)* }; [$($(#[$var_attr])* $variant),+]; $enum_name);
    };
    ({ $(#[$attr:meta])* fn $method:ident(&mut $self_:ident $(, $($arg:ident: $arg_ty:ty),*)? ) $( -> $return_type:ty)?; $($rest:tt)* }; [$($(#[$var_attr:meta])* $variant:ident),+]; $enum_name:ident) => {
        $crate::__build_method!($(#[$attr])* $method; { &mut }; $self_; { $($($arg: $arg_ty),*)? }; $( -> $return_type)?; [$($(#[$var_attr])* $variant),+]; $enum_name);
        $crate::__munch_methods!({ $($rest)* }; [$($(#[$var_attr])* $variant),+]; $enum_name);
    };

    // variants with block
    ({ $(#[$attr:meta])* fn $method:ident($self_:ident $(, $($arg:ident: $arg_ty:ty),*)? ) $( -> $return_type:ty)? $body:block $($rest:tt)* }; [$($(#[$var_attr:meta])* $variant:ident),+]; $enum_name:ident) => {
        $crate::__build_method!($(#[$attr])* $method; { }; $self_; { $($($arg: $arg_ty),*)? }; $( -> $return_type)?; [$($(#[$var_attr])* $variant),+]; $enum_name);
        $crate::__munch_methods!({ $($rest)* }; [$($(#[$var_attr])* $variant),+]; $enum_name);
    };
    ({ $(#[$attr:meta])* fn $method:ident(&$self_:ident $(, $($arg:ident: $arg_ty:ty),*)? ) $( -> $return_type:ty)? $body:block $($rest:tt)* }; [$($(#[$var_attr:meta])* $variant:ident),+]; $enum_name:ident) => {
        $crate::__build_method!($(#[$attr])* $method; { & }; $self_; { $($($arg: $arg_ty),*)? }; $( -> $return_type)?; [$($(#[$var_attr])* $variant),+]; $enum_name);
        $crate::__munch_methods!({ $($rest)* }; [$($(#[$var_attr])* $variant),+]; $enum_name);
    };
    ({ $(#[$attr:meta])* fn $method:ident(&mut $self_:ident $(, $($arg:ident: $arg_ty:ty),*)? ) $( -> $return_type:ty)? $body:block $($rest:tt)* }; [$($(#[$var_attr:meta])* $variant:ident),+]; $enum_name:ident) => {
        $crate::__build_method!($(#[$attr])* $method; { &mut }; $self_; { $($($arg: $arg_ty),*)? }; $( -> $return_type)?; [$($(#[$var_attr])* $variant),+]; $enum_name);
        $crate::__munch_methods!({ $($rest)* }; [$($(#[$var_attr])* $variant),+]; $enum_name);
    };

    ({ fn $method:ident $($rest:tt)* }; [$($(#[$var_attr:meta])* $variant:ident),+]; $enum_name:ident ) => {
        compile_error!(concat!("method `", stringify!($method), "` should receive self"));
    }
}

#[macro_export]
macro_rules! enum_dispatch {
    (
        $trait_vis:vis trait $train_name:ident {
            $($any:tt)*
        }
        $enum_vis:vis enum $enum_name:ident {
            $($(#[$var_attr:meta])* $variant:ident($variant_type:ty)),+$(,)?
        }
    ) => {
        $trait_vis trait $train_name {
            $($any)*
        }

        $enum_vis enum $enum_name {
            $($(#[$var_attr])* $variant($variant_type)),+
        }

        impl $train_name for $enum_name {
            $crate::__munch_methods!({ $($any)* }; [$($(#[$var_attr])* $variant),+]; $enum_name);
        }

    };
}
