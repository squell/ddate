use super::discord::{Discordian, Longform, Shortform};
use getrandom::getrandom;
use num_ordinal::{Ordinal, O8};
use time::Date;

pub fn ddate(fmt: &str, date: Date) -> Option<String> {
    let st_tib = "St. Tib's Day";
    let mut output = String::new();
    let mut subgenius = String::new();
    let mut str = &mut output;
    let mut iter = fmt.chars();
    while let Some(c) = iter.next() {
        match c {
            '%' => match iter.next() {
                Some('Y') => str.push_str(&format!("{}", date.discordian_year())),
                Some('A') => str.push_str(
                    date.discordian_weekday()
                        .map(|day| day.full_name())
                        .unwrap_or(st_tib),
                ),
                Some('a') => str.push_str(date.discordian_weekday()?.short_name()),
                Some('B') => str.push_str(date.discordian_season()?.full_name()),
                Some('b') => str.push_str(date.discordian_season()?.short_name()),
                Some('d') => str.push_str(&format!("{}", date.discordian_day()?)),
                Some('e') => str.push_str(&format!("{}", O8::from1(date.discordian_day()?))),

                Some('H') => str.push_str(if date.is_sttibs_day() {
                    "St. Tibb's Day"
                } else {
                    date.discordian_holyday()?.full_name()
                }),
                Some('N') => {
                    if date.discordian_holyday().is_none() {
                        break;
                    }
                }
                Some('n') => str.push('\n'),
                Some('t') => str.push('\t'),
                Some('X') => str.push_str("X-Day has passed, the aliens have taken over."),
                Some('.') => {
                    let mut eris = [0];
                    getrandom(&mut eris).ok()?;
                    let fnord = eris[0] as usize % WORDS_OF_WISDOM.len();
                    str.push_str(WORDS_OF_WISDOM[fnord]);
                }

                Some('{') => {
                    if date.is_sttibs_day() {
                        str.push_str("St. Tib's Day");
                        str = &mut subgenius;
                    }
                }
                Some('}') => str = &mut output,

                None => str.push('%'),

                Some(c) => {
                    str.push('%');
                    str.push(c);
                }
            },
            _ => str.push(c),
        }
    }

    Some(output)
}

const WORDS_OF_WISDOM: &[&str] = &[
    "Hail Eris!",
    "All Hail Discordia!",
    "Kallisti!",
    "Fnord.",
    "Or not.",
    "Wibble.",
    "Pzat!",
    "P'tang!",
    "Frink!",
    "Grudnuk demand sustenance!",
    "Keep the Lasagna flying!",
    "You are what you see.",
    "Or is it?",
    "This statement is false.",
    "Lies and slander, sire!",
    "Hee hee hee!",
    "Rusty greyface!",
    "",
];
