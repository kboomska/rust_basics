fn main() {
    let mut index: usize = 0;

    for ordinal in ORDINALS {
        paragraph(ordinal, index);
        index += 1;
    }
}

fn paragraph(ordinal: &str, index: usize) {
    println!("On the {ordinal} day of Christmas,");
    println!("My true love sent to me");

    for i in 0..index + 1 {
        let i = index - i;
        let line = LINES[i];
        if i == 0 {
            let article = if index == 0 { "A " } else { "And a " };
            print!("{article}");
        }
        println!("{line}")
    }

    println!("");
}

const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const LINES: [&str; 12] = [
    "partridge in a pear tree.",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];
