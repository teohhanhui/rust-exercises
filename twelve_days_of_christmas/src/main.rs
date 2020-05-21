use console::Term;
use im::ordmap;
use im::ordmap::OrdMap;
use std::iter;

fn main() {
    let day_gifts: OrdMap<usize, &str> = ordmap! {
        1 => "partridge in a pear tree",
        2 => "turtle doves",
        3 => "French hens",
        4 => "calling birds",
        5 => "gold rings",
        6 => "geese a-laying",
        7 => "swans a-swimming",
        8 => "maids a-milking",
        9 => "ladies dancing",
        10 => "lords a-leaping",
        11 => "pipers piping",
        12 => "drummers drumming"
    };
    let cardinal_number_words: OrdMap<usize, &str> = ordmap! {
        1 => "a",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve"
    };
    let ordinal_number_words: OrdMap<usize, &str> = ordmap! {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth"
    };

    let verses = (1..=12).map(|n: usize| {
        iter::once(format!(
            "On the {} day of Christmas",
            ordinal_number_words[&n]
        ))
        .chain(iter::once(String::from("My true love sent to me")))
        .chain(
            (1..=n)
                .rev()
                .map(|m| {
                    format!(
                        "{}{} {}",
                        if m == 1 && n > 1 { "And " } else { "" },
                        cardinal_number_words[&m],
                        day_gifts[&m]
                    )
                })
                .collect::<Vec<String>>(),
        )
        .map(|line| [line[0..1].to_uppercase().as_str(), &line[1..]].concat())
    });

    println!("Twelve Days of Christmas");

    let term = Term::stdout();

    for lines in verses {
        if term.features().is_attended() {
            while term.read_char().expect("Failed to read character") != ' ' {}
        }

        println!();
        for line in lines {
            println!("{}", line)
        }
    }
}
