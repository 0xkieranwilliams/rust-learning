#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}


pub fn run() {
    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);

    /////////////////////////////////////////////
    //
    // Match can be used to destructure a tuple
    //
    /////////////////////////////////////////////
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }


    //////////////////////////////////////////////////////
    //
    // Match can be used to destructure an array/ slice
    //
    //////////////////////////////////////////////////////

    // Try changing the values in the array, or make it a slice!
    let array = [4, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }


    ///////////////////////////////////////////
    //
    // Match can be used to destructure enums
    //
    ///////////////////////////////////////////


    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }


    ///////////////////////////////////////////
    //
    // Match can be used with pointers / ref
    //
    ///////////////////////////////////////////


    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring {:?}", val)
    }

    // dereference before matching 
    match *reference {
        val => println!("Got a value via dereferencing {:?}", val) 
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r)
    }

    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can add anything to it
            *m += 10;
            println!("we added 10. `mut_value`: {:?}", m);
        }
    }
 


    ///////////////////////////////////////////
    //
    // Match can be used to descructure structs 
    //
    ///////////////////////////////////////////

    struct Foo {
        x: (u32, u32),
        y: u32
    }

    let foo = Foo {x : (1,2), y: 3};
    match foo {
        Foo { x: (1, b), y} => println!("First of x is 1, b = {}, y = {}", b, y),

        // you can rename the variables (x -> i)
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // you can ignore variables
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    let faa = Foo { x: (1,2), y: 3 };
    // You don't need a match block to destructure structs:
    let Foo { x : x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    // Destructuring works with nested structs too:
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y }  } = bar;
    println!("Nested:\n\tnested_x = {nested_x:?},\n\tnested_y = {nested_y:?}");


    let temperature = Temperature::Celsius(35);
    // ^ TODO try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
    }



    //////////////////////////////////////////
    //
    // Match can use binding using @ x0 ..=x
    //
    //////////////////////////////////////////


    println!("Tell me what type of person you are");

    fn age() -> u32 { 15 }
    match age() {
        0             => println!("I haven't celebrated my first bday yet"),
        n @ 1  ..=12  => println!("I am a child of age {:?}", n),
        n @ 13 ..=19  => println!("I am a teen of age {:?}", n),
        n             => println!("I'm an old person of age {:?}", n),

    }


    // you can also use the binding to destructure enum vairients such as Option
    fn some_number() -> Option<u32> { Some(42) }
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n)      => println!("Not interesting ... {}", n),
        _            => ()
    }
}










