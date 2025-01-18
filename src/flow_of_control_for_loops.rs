pub fn run () {

    // runs 1 -> 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // runs 1 -> 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // for and iterators
    // for in interacts with iterators:
    //  * for loop applies the into_iter function to the collection
    //  * into_iter, iter, and iter_mut are ways of handling the conversion of a collection into an
    //    iterator, providing different views into the data within
    //       * iter - borrows each element of the collection through each iteration, thus leaving the
    //         collection untouched and available for reuse after the loop.

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }
    }
    println!("name: {:?}", names);

    //       * into_iter - consumes the collection so that on each iteration the exact data is
    //         provided. Once the collection has been consumed it is no longer available for reuse
    //         as it has been 'moved' within the loop.

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }
    }

    // Line below doesn't work because the names variable has now been consumed
    // println!("names: {:?}", names);

    //      * iter_mut - this mutably borrows each element of the collection, allowing for the
    //        collection to be modified in place.
}
