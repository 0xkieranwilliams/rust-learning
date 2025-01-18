pub fn run () {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzsbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n %  5 == 0 {
            println!("buzzzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}
