pub(crate) fn the_twelve_days_of_christmas() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const GIFTS: [&str; 12] = [
        "And a partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut d_index = 0;

    for day in DAYS {
        println!("");
        println!("On the {} day of Christmas", day);
        println!("My true love sent to me");

        for g_index in (0..d_index + 1).rev() {
            let verse = GIFTS[g_index];

            if g_index == 0 && d_index == 0 {
                println!("{}", capitalize(&verse[4..verse.len()]));
            } else if g_index == 10 && d_index == 10 {
                println!("I sent {}", verse);
            } else {
                println!("{}", verse);
            }

            if g_index == 0 && d_index == 11 {
                println!("{}", verse);
            }
        }

        d_index += 1;
    }
}

fn capitalize(verse: &str) -> String {
    verse[0..1].to_uppercase() + &verse[1..]
}
