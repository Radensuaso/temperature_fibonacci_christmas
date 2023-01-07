pub(crate) fn nth_fibonacci() {
    let n: u128 = loop {
        let mut n = String::new();

        println!("Type the nth integer you want to know the correspondent fibonacci.");

        std::io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let value: u128 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break value;
    };

    let n = n;

    println!("The {n} number in fibonacci sequence is {}.", fibonacci(n));
}

fn fibonacci(n: u128) -> u128 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    let mut result = 0;

    for _ in 0..n {
        result = c;
        c = a + b;
        a = b;
        b = c;
    }

    return result;
}
