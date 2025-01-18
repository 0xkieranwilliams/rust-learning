#![allow(unreachable_code, unused_labels)]

pub fn run () {
    let mut count = 0u32;
    println!("Let's count until infinity");
    loop {
        count += 1;
        println!("{}", count);

        if count == 3 {
            println!("threeee");
            continue;
        }

        if count == 5 {
            println!("Okeyyyyyyy, that's enough");
            break;
        }
    }

    // Nesting and Labels

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            //This would break only the inner loop
            //break;
            
            //This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");


    // Returning from loops
    let mut counter = 0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
