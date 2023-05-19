/* POEE calendar written in Rust.
 *
 * And then it came to pass that the ddate program, once standard on every Linux system, was censored by
 * greyface from util-linux for being "unsafe" or "unnecessary" and other unenlightened reasons.
 *
 * This removes any of those concerns since Rust code is safe, and necessary.
 *
 * We stand on the shoulders of giants:
 *
 * Original program written on the 65th day of The Aftermath in 3157 YOLD by Druel the Chaotic;
 * further hacked on the 42th day of Bureaucracy in 3161 YOLD by Lee H:. O:. Smith, KYTP;
 * slighly crackled on the 53rd day of Bureaucracy, 3179 YOLD by Chaplain Nyan the Wiser;
 * hacked from scratch in Rust on the 66th day of Discord, in the  YOLD 3189.
 *
 * Okay, maybe not giants. But in any case they are someone else's shoulders.
 */

use std::cmp::Ordering;
use std::env;
use std::ops::Add;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use time::{macros::date, Date, Month};

mod discord;
mod formatter;

pub fn now() -> Date {
    let days_since_beard = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("your UNIX timestamps are interesting.");

    date!(1970 - 01 - 01).add(days_since_beard)
}

fn assistance() {
    println!("ddate [+format] [day month year]");
}

fn parse_date(day: Option<String>, month: Option<String>, year: Option<String>) -> Option<Date> {
    let aneristic_day = str::parse::<u8>(&day?).ok()?;
    let aneristic_month = month.and_then(|text| {
        if let Ok(number) = str::parse::<u8>(&text) {
            Month::try_from(number).ok()
        } else {
            Month::from_str(&text).ok()
        }
    })?;
    let aneristic_year = str::parse::<i32>(&year?)
        .ok()
        .and_then(|year| match year.cmp(&0) {
            Ordering::Greater => Some(year),
            Ordering::Less => Some(year + 1),
            _ => None,
        })?;

    Date::from_calendar_date(aneristic_year, aneristic_month, aneristic_day).ok()
}

fn main() {
    let fmt;

    let mut user_wishes = env::args();
    let mut arg1 = user_wishes.nth(1);
    match arg1 {
        Some(arg) if arg.starts_with('+') => {
            fmt = arg[1..].to_string();
            arg1 = user_wishes.next();
        }
        _ => fmt = "Today is %{%A, the %e day of %B%} in the YOLD %Y%N%nCelebrate %H".to_string(),
    }

    let the_date = if arg1.is_some() {
        let Some(parsed_date) = parse_date(arg1, user_wishes.next(), user_wishes.next()) else {
            return assistance()
        };
        if user_wishes.next().is_some() {
            return assistance();
        }

        parsed_date
    } else {
        now()
    };

    if let Some(blessing) = formatter::ddate(&fmt, the_date) {
        println!("{blessing}");
    } else {
        println!("First you must sprinkle me with fairy dust.");
    }
}
