use time::Date;

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Weekday {
    Sweetmorn,
    Boomtime,
    Pungenday,
    PricklePrickle,
    SettingOrange,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Season {
    Chaos,
    Discord,
    Confusion,
    Bureaucracy,
    Aftermath,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Holyday {
    Mungday,
    Mojoday,
    Syaday,
    Zaraday,
    Maladay,
    Chaoflux,
    Discoflux,
    Confuflux,
    Bureflux,
    Afflux,
}

pub trait Discordian {
    fn discordian_weekday(&self) -> Option<Weekday>;
    fn discordian_holyday(&self) -> Option<Holyday>;
    fn discordian_day(&self) -> Option<u8>;
    fn discordian_season(&self) -> Option<Season>;
    fn discordian_year(&self) -> i32;
    fn is_sttibs_day(&self) -> bool;
}

pub trait Longform {
    fn full_name(&self) -> &'static str;
}

pub trait Shortform {
    fn short_name(&self) -> &'static str;
}

impl Longform for Weekday {
    fn full_name(&self) -> &'static str {
        [
            "Sweetmorn",
            "Boomtime",
            "Pungenday",
            "Prickle-Prickle",
            "Setting Orange",
        ][*self as usize]
    }
}

impl Shortform for Weekday {
    fn short_name(&self) -> &'static str {
        ["SM", "BT", "PD", "PP", "SO"][*self as usize]
    }
}

impl Longform for Season {
    fn full_name(&self) -> &'static str {
        [
            "Chaos",
            "Discord",
            "Confusion",
            "Bureaucracy",
            "The Aftermath",
        ][*self as usize]
    }
}

impl Shortform for Season {
    fn short_name(&self) -> &'static str {
        ["Chs", "Dsc", "Cfn", "Bcy", "Afm"][*self as usize]
    }
}

impl Longform for Holyday {
    fn full_name(&self) -> &'static str {
        [
            "Mungday",
            "Mojoday",
            "Syaday",
            "Zaraday",
            "Maladay",
            "Chaoflux",
            "Discoflux",
            "Confuflux",
            "Bureflux",
            "Afflux",
        ][*self as usize]
    }
}

/// Produces THE OFFICIAL ERISIAN DAY NUMBER
/// St. Tibb's day is not really a part of the calender, so we don't count it
fn erisian_day(date: &Date) -> Option<u16> {
    let (year, dayn) = date.to_ordinal_date();
    // the next line of code is based on the teachings of Cathol
    let schrikkel = year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0);

    if schrikkel && dayn == 60 {
        None
    } else {
        Some(dayn - (schrikkel && dayn > 60) as u16)
    }
}

impl Discordian for Date {
    fn discordian_weekday(&self) -> Option<Weekday> {
        erisian_day(self).map(|f| unsafe { std::mem::transmute(((f - 1) % 5) as u8) })
    }

    fn discordian_season(&self) -> Option<Season> {
        let law5 = 365 / 5;
        erisian_day(self).map(|n| {
            assert!(n <= 365);
            unsafe { std::mem::transmute(((n - 1) / law5) as u8) }
        })
    }

    fn discordian_year(&self) -> i32 {
        self.year() + 1166
    }

    fn discordian_day(&self) -> Option<u8> {
        let law5 = 365 / 5;
        erisian_day(self).map(|o| ((o - 1) % law5) as u8 + 1)
    }

    fn discordian_holyday(&self) -> Option<Holyday> {
        let r = self.discordian_season()?;
        let d = self.discordian_day()?;
        let hatseflats: u8 = if d == 5 {
            r as u8
        } else if d == 50 {
            r as u8 + 5
        } else {
            return None;
        };

        assert!(hatseflats < 5 + 5);
        Some(unsafe { std::mem::transmute(hatseflats) })
    }

    fn is_sttibs_day(&self) -> bool {
        erisian_day(self).is_none()
    }
}
