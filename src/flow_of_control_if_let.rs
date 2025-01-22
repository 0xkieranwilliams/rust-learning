enum Foo {
    Bar, 
    Baz,
    Qux(u32)
}

pub fn run () {
    let optional = Some(7);
    match optional {
        Some(i) => println!("This is a reaally long sttring and {:?}", i),
        _ => {},
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    }
    else {
        println!("Didn't match a number. Let's go with a letter")
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number, lets go with letters")
    } else {
        println!("I don't like letters. Lets go with an emoticon :)");
    }




    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // if variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 200) = c {
        println!("c is 200"); // won't print
    }
    if let Foo::Qux(value @ 100) = c {
        println!("c is 100");
    }

    /// challenge
    let a = Foo::Bar;

    // doesn't work
    // if Foo::Bar == a {
    //     println!("a is foobar");
    // }

    if let Foo::Bar = a {
        println!("a is foobar - this worksss");
    }
}

