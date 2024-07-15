const UNIT_TEENS: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

/// Convert a number from 0-99 to a String
fn tens(input: usize) -> Result<String, ()> {
    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    // Check not greater than 99.
    // match first digit: if in 2..=9 then take the TENS then the UNIT_TEENS
    // else, take only the UNIT_TEENS

    match input {
        100.. => Err(()),
        number @ 20..=99 => {
            let tens = number / 10;
            let units = number % 10;
            if units == 0 {
                Ok(TENS[tens].into())
            } else {
                Ok(format!("{} {}", TENS[tens], UNIT_TEENS[units]))
            }
        }
        unit_teens @ 0..=19 => Ok(UNIT_TEENS[unit_teens].into()),
    }
}

/// Convert a number from 0 to 100 to a String
fn hundreds(input: usize) -> Result<String, ()> {
    match input {
        1000.. => Err(()),
        number @ 100..=999 => {
            let hundreds = number / 100;
            let hundreds_str = format!("{} hundred", UNIT_TEENS[hundreds]);
            let small = number % 100;
            Ok(if small == 0 {
                hundreds_str
            } else {
                format!("{} and {}", hundreds_str, tens(small)?)
            })
        }
        small @ 0..=99 => tens(small),
    }
}

const MAGNITUDE: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

/// Convert any number into a String.
fn name_number(mut input: usize) -> String {
    // One hundred and thirty million, three hundred and eight thousand etc.
    // Per element, and by its position, convert. Then reverse order and add commas.
    if input == 0 {
        return "zero".into();
    }

    let thousands = std::iter::from_fn(move || {
        if input > 0 {
            let output = input % 1000;
            input /= 1000;
            Some(output)
        } else {
            None
        }
    });

    std::iter::zip(thousands, MAGNITUDE)
        .filter_map(|(value, label)| {
            if value != 0 {
                Some(format!(
                    "{} {label}",
                    hundreds(value).expect("Given number less than 1000")
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(", ")
        .trim_end()
        .to_string()
}

fn main() -> Result<(), &'static str> {
    let num: usize = std::env::args().nth(1).map_or_else(
        || text_io::try_read!("{}").map_err(|_| "Failed_to read from stdin"),
        |x| x.parse().map_err(|_| "Please use a correct number format"),
    )?;

    println!("{}", name_number(num));

    Ok(())
}
