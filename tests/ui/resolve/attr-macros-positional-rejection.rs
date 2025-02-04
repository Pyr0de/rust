//! Check that certain positions (listed below) only permit *non-macro* attributes and reject
//! attribute macros:
//!
//! - Enum variants
//! - Struct fields
//! - Field in a struct pattern
//! - Match arm
//! - Field in struct initialization expression

enum FooEnum {
    #[test]
    //~^ ERROR expected non-macro attribute, found attribute macro
    Bar(i32),
}

struct FooStruct {
    #[test]
    //~^ ERROR expected non-macro attribute, found attribute macro
    bar: i32,
}

fn main() {
    let foo_enum_bar = FooEnum::Bar(1);
    match foo_enum_bar {
        FooEnum::Bar(x) => {},
        _ => {}
    }

    let foo_struct = FooStruct { bar: 1 };
    match foo_struct {
        FooStruct {
            #[test] bar
            //~^ ERROR expected non-macro attribute, found attribute macro
        } => {}
    }

    match 1 {
        0 => {}
        #[test]
        //~^ ERROR expected non-macro attribute, found attribute macro
        _ => {}
    }

    let _another_foo_struct = FooStruct {
        #[test]
        //~^ ERROR expected non-macro attribute, found attribute macro
        bar: 1,
    };
}
