use std::str::FromStr;

use anyhow::{anyhow, Result};
use chrono::{offset::FixedOffset, prelude::*};
use lazy_static::lazy_static;
use regex::Regex;

/// Parse struct has methods implemented parsers for accepted formats.
#[derive(Debug)]
pub struct Parse<'z, Tz2> {
    tz:           &'z Tz2,
    default_time: NaiveTime,
}

impl<'z, Tz2> Parse<'z, Tz2>
where
    Tz2: TimeZone,
{
    /// Create a new instrance of [`Parse`] with a custom parsing timezone that
    /// handles the datetime string without time offset.
    pub const fn new(tz: &'z Tz2, default_time: NaiveTime) -> Self { Self { tz, default_time } }

    /// This method tries to parse the input datetime string with a list of
    /// accepted formats. See more exmaples from [`Parse`],
    /// [`crate::parse()`] and [`crate::parse_with_timezone()`].
    pub fn parse(&self, input: &str) -> Result<DateTime<Utc>> {
        self.unix_timestamp(input)
            .or_else(|| self.rfc2822(input))
            .or_else(|| self.ymd_family(input))
            .or_else(|| self.ymd_family_ib(input))
            .or_else(|| self.hms_family(input))
            .or_else(|| self.month_ymd(input))
            .or_else(|| self.month_mdy_family(input))
            .or_else(|| self.month_dmy_family(input))
            .or_else(|| self.slash_mdy_family(input))
            .or_else(|| self.slash_ymd_family(input))
            .or_else(|| self.dot_mdy_or_ymd(input))
            .or_else(|| self.mysql_log_timestamp(input))
            .or_else(|| self.chinese_ymd_family(input))
            .unwrap_or_else(|| Err(anyhow!("{} did not match any formats.", input)))
    }

    fn ymd_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}-[0-9]{2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.rfc3339(input)
            .or_else(|| self.postgres_timestamp(input))
            .or_else(|| self.ymd_hms(input))
            .or_else(|| self.ymd_hms_z(input))
            .or_else(|| self.ymd(input))
            .or_else(|| self.ymd_z(input))
    }

    fn ymd_family_ib(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}[0-9]{2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.rfc3339(input)
            .or_else(|| self.ymd_hms_ib(input))
            .or_else(|| self.ymd_hms_z_ib(input))
            .or_else(|| self.ymd_hms_timezone_ib(input))
            .or_else(|| self.ymd(input))
            .or_else(|| self.ymd_z(input))
    }

    fn hms_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{1,2}:[0-9]{2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.hms(input)
            .or_else(|| self.hms_z(input))
            .or_else(|| self.hms_timezone_ib(input))
    }

    fn month_mdy_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z]{3,9}\.?\s+[0-9]{1,2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.month_md_hms(input)
            .or_else(|| self.month_mdy_hms(input))
            .or_else(|| self.month_mdy_hms_z(input))
            .or_else(|| self.month_mdy(input))
    }

    fn month_dmy_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{1,2}\s+[a-zA-Z]{3,9}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.month_dmy_hms(input).or_else(|| self.month_dmy(input))
    }

    fn slash_mdy_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{1,2}/[0-9]{1,2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.slash_mdy_hms(input).or_else(|| self.slash_mdy(input))
    }

    fn slash_ymd_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}/[0-9]{1,2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.slash_ymd_hms(input).or_else(|| self.slash_ymd(input))
    }

    fn chinese_ymd_family(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}年[0-9]{2}月").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }
        self.chinese_ymd_hms(input)
            .or_else(|| self.chinese_ymd(input))
    }

    // unix timestamp
    // - 1511648546
    // - 1620021848429
    // - 1620024872717915000
    fn unix_timestamp(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{10,19}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        input
            .parse::<i64>()
            .ok()
            .and_then(|timestamp| {
                match input.len() {
                    10 => Some(Utc.timestamp_opt(timestamp, 0).unwrap()),
                    13 => Some(Utc.timestamp_millis_opt(timestamp).unwrap()),
                    19 => Some(Utc.timestamp_nanos(timestamp)),
                    _ => None,
                }
                .map(|datetime| datetime.with_timezone(&Utc))
            })
            .map(Ok)
    }

    // rfc3339
    // - 2021-05-01T01:17:02.604456Z
    // - 2017-11-25T22:34:50Z
    fn rfc3339(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        DateTime::parse_from_rfc3339(input)
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // rfc2822
    // - Wed, 02 Jun 2021 06:31:39 GMT
    fn rfc2822(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        DateTime::parse_from_rfc2822(input)
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // postgres timestamp yyyy-mm-dd hh:mm:ss z
    // - 2019-11-29 08:08-08
    // - 2019-11-29 08:08:05-08
    // - 2021-05-02 23:31:36.0741-07
    // - 2021-05-02 23:31:39.12689-07
    // - 2019-11-29 08:15:47.624504-08
    // - 2017-07-19 03:21:51+00:00
    fn postgres_timestamp(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}-[0-9]{2}-[0-9]{2}\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?[+-:0-9]{3,6}$",
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S%#z")
            .or_else(|_| DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S%.f%#z"))
            .or_else(|_| DateTime::parse_from_str(input, "%Y-%m-%d %H:%M%#z"))
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy-mm-dd hh:mm:ss
    // - 2014-04-26 05:24:37 PM
    // - 2021-04-30 21:14
    // - 2021-04-30 21:14:10
    // - 2021-04-30 21:14:10.052282
    // - 2014-04-26 17:24:37.123
    // - 2014-04-26 17:24:37.3186369
    // - 2012-08-03 18:31:59.257000000
    fn ymd_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}-[0-9]{2}-[0-9]{2}\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?\s*(am|pm|AM|PM)?$",
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%Y-%m-%d %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(input, "%Y-%m-%d %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y-%m-%d %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y-%m-%d %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y-%m-%d %I:%M %P"))
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyymmdd hh:mm:ss
    // - 20210430 21:14
    // - 20210430 21:14:10
    // - 20210430 21:14:10.052282
    // - 20140426 17:24:37.123
    // - 20140426 17:24:37.3186369
    // - 20120803 18:31:59.257000000
    fn ymd_hms_ib(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[0-9]{4}[0-9]{2}[0-9]{2}[\s\-]+[0-9]{2}:[0-9]{2}(:[0-9]{2})?$",)
                    .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%Y%m%d %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(input, "%Y%m%d %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y%m%d-%H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y%m%d-%H:%M:%S"))
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy-mm-dd hh:mm:ss z
    // - 2017-11-25 13:31:15 PST
    // - 2017-11-25 13:31 PST
    // - 2014-12-16 06:20:00 UTC
    // - 2014-12-16 06:20:00 GMT
    // - 2014-04-26 13:13:43 +0800
    // - 2014-04-26 13:13:44 +09:00
    // - 2012-08-03 18:31:59.257000000 +0000
    // - 2015-09-30 18:48:56.35272715 UTC
    fn ymd_hms_z(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}-[0-9]{2}-[0-9]{2}\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?(?P<tz>\s*[+-:a-zA-Z0-9]{3,6})$",
            ).unwrap();
        }

        if !RE.is_match(input) {
            return None;
        }
        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                let parse_from_str = NaiveDateTime::parse_from_str;
                return match parse_timezone(matched_tz.as_str().trim()) {
                    Ok(offset) => {
                        parse_from_str(input, "%Y-%m-%d %H:%M:%S %Z")
                            .or_else(|_| parse_from_str(input, "%Y-%m-%d %H:%M %Z"))
                            .or_else(|_| parse_from_str(input, "%Y-%m-%d %H:%M:%S%.f %Z"))
                            .ok()
                            .and_then(|parsed| offset.from_local_datetime(&parsed).single())
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok)
                    },
                    Err(err) => Some(Err(err)),
                };
            }
        }
        None
    }

    // yyyymmdd hh:mm:ss z
    // - 20171125 13:31:15 PST
    // - 20171125 13:31 PST
    // - 20141216 06:20:00 UTC
    // - 20141216 06:20:00 GMT
    // - 20140426 13:13:43 +0800
    // - 20140426 13:13:44 +09:00
    fn ymd_hms_z_ib(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}[0-9]{2}[0-9]{2}[\s\-]?[0-9]{2}:[0-9]{2}(:[0-9]{2})?(?P<tz>\s*[+-:a-zA-Z0-9]{3,6})$",
            ).unwrap();
        }

        if !RE.is_match(input) {
            return None;
        }
        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                let parse_from_str = NaiveDateTime::parse_from_str;
                return match parse_timezone(matched_tz.as_str().trim()) {
                    Ok(offset) => {
                        parse_from_str(input, "%Y%m%d %H:%M:%S %Z")
                            .or_else(|_| parse_from_str(input, "%Y%m%d %H:%M %Z"))
                            .or_else(|_| parse_from_str(input, "%Y%m%d-%H:%M:%S %Z"))
                            .or_else(|_| parse_from_str(input, "%Y%m%d-%H:%M %Z"))
                            .ok()
                            .and_then(|parsed| offset.from_local_datetime(&parsed).single())
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok)
                    },
                    Err(err) => Some(Err(err)),
                };
            }
        }
        None
    }

    // yyyymmdd hh:mm:ss www/www
    // - 20171125 13:31:15 Europe/Berlin
    // - 20171125-13:31:15 Europe/Berlin
    fn ymd_hms_timezone_ib(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}[0-9]{2}[0-9]{2}[\s\-]?[0-9]{2}:[0-9]{2}(:[0-9]{2})?(?P<tz>\s*\w+/\w+)$",
            )
            .unwrap();
        }

        if !RE.is_match(input) {
            return None;
        }
        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                let parse_from_str = NaiveDateTime::parse_from_str;
                return match chrono_tz::Tz::from_str(matched_tz.as_str().trim()) {
                    Ok(parsed_tz) => {
                        parse_from_str(input, "%Y%m%d %H:%M:%S %Z")
                            .or_else(|_| parse_from_str(input, "%Y%m%d %H:%M %Z"))
                            .or_else(|_| parse_from_str(input, "%Y%m%d-%H:%M:%S %Z"))
                            .or_else(|_| parse_from_str(input, "%Y%m%d-%H:%M %Z"))
                            .ok()
                            .and_then(|parsed| parsed_tz.from_local_datetime(&parsed).single())
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok)
                    },
                    Err(err) => Some(Err(anyhow!("{} - cannot parse string {}", err, input))),
                };
            }
        }
        None
    }

    // yyyymmdd hh:mm:ss www/www
    // - 13:31:15 Europe/Berlin
    #[allow(deprecated)]
    fn hms_timezone_ib(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<time>[0-9]{2}:[0-9]{2}(:[0-9]{2})?)(?P<tz>\s*\w*/\w*)$",)
                    .unwrap();
        }

        if !RE.is_match(input) {
            return None;
        }
        if let Some(caps) = RE.captures(input) {
            if let (Some(matched_tz), Some(matched_time)) = (caps.name("tz"), caps.name("time")) {
                return match chrono_tz::Tz::from_str(matched_tz.as_str().trim()) {
                    Ok(parsed_tz) => {
                        let now = Utc::now().with_timezone(&parsed_tz);
                        let time = matched_time.as_str().trim();
                        NaiveTime::parse_from_str(time, "%H:%M:%S")
                            .or_else(|_| NaiveTime::parse_from_str(time, "%H:%M"))
                            .ok()
                            .and_then(|parsed| now.date().and_time(parsed))
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok)
                    },
                    Err(err) => Some(Err(anyhow!("{} - cannot parse string {}", err, input))),
                };
            }
        }
        None
    }

    // yyyy-mm-dd
    // - 2021-02-21
    #[allow(deprecated)]
    fn ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap();
        }

        if !RE.is_match(input) {
            return None;
        }
        let now = Utc::now()
            .date()
            .and_time(self.default_time)?
            .with_timezone(self.tz);
        NaiveDate::parse_from_str(input, "%Y-%m-%d")
            .ok()
            .map(|parsed| parsed.and_time(now.time()))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy-mm-dd z
    // - 2021-02-21 PST
    // - 2021-02-21 UTC
    // - 2020-07-20+08:00 (yyyy-mm-dd-07:00)
    #[allow(deprecated)]
    fn ymd_z(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}(?P<tz>\s*[+-:a-zA-Z0-9]{3,6})$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                return match parse_timezone(matched_tz.as_str().trim()) {
                    Ok(offset) => {
                        let now = Utc::now()
                            .date()
                            .and_time(self.default_time)?
                            .with_timezone(&offset);
                        NaiveDate::parse_from_str(input, "%Y-%m-%d %Z")
                            .ok()
                            .map(|parsed| parsed.and_time(now.time()))
                            .and_then(|datetime| offset.from_local_datetime(&datetime).single())
                            .map(|at_tz| at_tz.with_timezone(&Utc))
                            .map(Ok)
                    },
                    Err(err) => Some(Err(err)),
                };
            }
        }
        None
    }

    // hh:mm:ss
    // - 01:06:06
    // - 4:00pm
    // - 6:00 AM
    #[allow(deprecated)]
    fn hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now().with_timezone(self.tz);
        NaiveTime::parse_from_str(input, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(input, "%H:%M"))
            .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M:%S %P"))
            .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M %P"))
            .ok()
            .and_then(|parsed| now.date().and_time(parsed))
            .map(|datetime| datetime.with_timezone(&Utc))
            .map(Ok)
    }

    // hh:mm:ss z
    // - 01:06:06 PST
    // - 4:00pm PST
    // - 6:00 AM PST
    // - 6:00pm UTC
    #[allow(deprecated)]
    fn hms_z(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?(?P<tz>\s+[+-:a-zA-Z0-9]{3,6})$",
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                return match parse_timezone(matched_tz.as_str().trim()) {
                    Ok(offset) => {
                        let now = Utc::now().with_timezone(&offset);
                        NaiveTime::parse_from_str(input, "%H:%M:%S %Z")
                            .or_else(|_| NaiveTime::parse_from_str(input, "%H:%M %Z"))
                            .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M:%S %P %Z"))
                            .or_else(|_| NaiveTime::parse_from_str(input, "%I:%M %P %Z"))
                            .ok()
                            .map(|parsed| now.date().naive_local().and_time(parsed))
                            .and_then(|datetime| offset.from_local_datetime(&datetime).single())
                            .map(|at_tz| at_tz.with_timezone(&Utc))
                            .map(Ok)
                    },
                    Err(err) => Some(Err(err)),
                };
            }
        }
        None
    }

    // yyyy-mon-dd
    // - 2021-Feb-21
    #[allow(deprecated)]
    fn month_ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}-[a-zA-Z]{3,9}-[0-9]{2}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now()
            .date()
            .and_time(self.default_time)?
            .with_timezone(self.tz);
        NaiveDate::parse_from_str(input, "%Y-%m-%d")
            .or_else(|_| NaiveDate::parse_from_str(input, "%Y-%b-%d"))
            .ok()
            .map(|parsed| parsed.and_time(now.time()))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // Mon dd hh:mm:ss
    // - May 6 at 9:24 PM
    // - May 27 02:45:27
    #[allow(deprecated)]
    fn month_md_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[a-zA-Z]{3}\s+[0-9]{1,2}\s*(at)?\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?$",
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now().with_timezone(self.tz);
        let with_year = format!("{} {}", now.year(), input);
        self.tz
            .datetime_from_str(&with_year, "%Y %b %d at %I:%M %P")
            .or_else(|_| self.tz.datetime_from_str(&with_year, "%Y %b %d %H:%M:%S"))
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
            .map(Ok)
    }

    // Mon dd, yyyy, hh:mm:ss
    // - May 8, 2009 5:57:51 PM
    // - September 17, 2012 10:09am
    // - September 17, 2012, 10:10:09
    fn month_mdy_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[a-zA-Z]{3,9}\.?\s+[0-9]{1,2},\s+[0-9]{2,4},?\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?$",
            ).unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let dt = input.replace(", ", " ").replace(". ", " ");
        self.tz
            .datetime_from_str(&dt, "%B %d %Y %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(&dt, "%B %d %Y %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%B %d %Y %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%B %d %Y %I:%M %P"))
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // Mon dd, yyyy hh:mm:ss z
    // - May 02, 2021 15:51:31 UTC
    // - May 02, 2021 15:51 UTC
    // - May 26, 2021, 12:49 AM PDT
    // - September 17, 2012 at 10:09am PST
    fn month_mdy_hms_z(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[a-zA-Z]{3,9}\s+[0-9]{1,2},?\s+[0-9]{4}\s*,?(at)?\s+[0-9]{2}:[0-9]{2}(:[0-9]{2})?\s*(am|pm|AM|PM)?(?P<tz>\s+[+-:a-zA-Z0-9]{3,6})$",
            ).unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        if let Some(caps) = RE.captures(input) {
            if let Some(matched_tz) = caps.name("tz") {
                let parse_from_str = NaiveDateTime::parse_from_str;
                return match parse_timezone(matched_tz.as_str().trim()) {
                    Ok(offset) => {
                        let dt = input.replace(',', "").replace("at", "");
                        parse_from_str(&dt, "%B %d %Y %H:%M:%S %Z")
                            .or_else(|_| parse_from_str(&dt, "%B %d %Y %H:%M %Z"))
                            .or_else(|_| parse_from_str(&dt, "%B %d %Y %I:%M:%S %P %Z"))
                            .or_else(|_| parse_from_str(&dt, "%B %d %Y %I:%M %P %Z"))
                            .ok()
                            .and_then(|parsed| offset.from_local_datetime(&parsed).single())
                            .map(|datetime| datetime.with_timezone(&Utc))
                            .map(Ok)
                    },
                    Err(err) => Some(Err(err)),
                };
            }
        }
        None
    }

    // Mon dd, yyyy
    // - May 25, 2021
    // - oct 7, 1970
    // - oct 7, 70
    // - oct. 7, 1970
    // - oct. 7, 70
    // - October 7, 1970
    #[allow(deprecated)]
    fn month_mdy(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[a-zA-Z]{3,9}\.?\s+[0-9]{1,2},\s+[0-9]{2,4}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now()
            .date()
            .and_time(self.default_time)?
            .with_timezone(self.tz);
        let dt = input.replace(", ", " ").replace(". ", " ");
        NaiveDate::parse_from_str(&dt, "%B %d %y")
            .or_else(|_| NaiveDate::parse_from_str(&dt, "%B %d %Y"))
            .ok()
            .map(|parsed| parsed.and_time(now.time()))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // dd Mon yyyy hh:mm:ss
    // - 12 Feb 2006, 19:17
    // - 12 Feb 2006 19:17
    // - 14 May 2019 19:11:40.164
    #[allow(deprecated)]
    fn month_dmy_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{1,2}\s+[a-zA-Z]{3,9}\s+[0-9]{2,4},?\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?$",
            ).unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let dt = input.replace(", ", " ");
        self.tz
            .datetime_from_str(&dt, "%d %B %Y %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(&dt, "%d %B %Y %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%d %B %Y %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%d %B %Y %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(&dt, "%d %B %Y %I:%M %P"))
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // dd Mon yyyy
    // - 7 oct 70
    // - 7 oct 1970
    // - 03 February 2013
    // - 1 July 2013
    #[allow(deprecated)]
    fn month_dmy(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[0-9]{1,2}\s+[a-zA-Z]{3,9}\s+[0-9]{2,4}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now()
            .date()
            .and_time(self.default_time)?
            .with_timezone(self.tz);
        NaiveDate::parse_from_str(input, "%d %B %y")
            .or_else(|_| NaiveDate::parse_from_str(input, "%d %B %Y"))
            .ok()
            .map(|parsed| parsed.and_time(now.time()))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // mm/dd/yyyy hh:mm:ss
    // - 4/8/2014 22:05
    // - 04/08/2014 22:05
    // - 4/8/14 22:05
    // - 04/2/2014 03:00:51
    // - 8/8/1965 12:00:00 AM
    // - 8/8/1965 01:00:01 PM
    // - 8/8/1965 01:00 PM
    // - 8/8/1965 1:00 PM
    // - 8/8/1965 12:00 AM
    // - 4/02/2014 03:00:51
    // - 03/19/2012 10:11:59
    // - 03/19/2012 10:11:59.3186369
    #[allow(deprecated)]
    fn slash_mdy_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{1,2}/[0-9]{1,2}/[0-9]{2,4}\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?\s*(am|pm|AM|PM)?$"
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%m/%d/%y %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%y %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%y %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%y %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%y %I:%M %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %H:%M:%S"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%m/%d/%Y %I:%M %P"))
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // mm/dd/yyyy
    // - 3/31/2014
    // - 03/31/2014
    // - 08/21/71
    // - 8/1/71
    #[allow(deprecated)]
    fn slash_mdy(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{1,2}/[0-9]{1,2}/[0-9]{2,4}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now()
            .date()
            .and_time(self.default_time)?
            .with_timezone(self.tz);
        NaiveDate::parse_from_str(input, "%m/%d/%y")
            .or_else(|_| NaiveDate::parse_from_str(input, "%m/%d/%Y"))
            .ok()
            .map(|parsed| parsed.and_time(now.time()))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy/mm/dd hh:mm:ss
    // - 2014/4/8 22:05
    // - 2014/04/08 22:05
    // - 2014/04/2 03:00:51
    // - 2014/4/02 03:00:51
    // - 2012/03/19 10:11:59
    // - 2012/03/19 10:11:59.3186369
    #[allow(deprecated)]
    fn slash_ymd_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^[0-9]{4}/[0-9]{1,2}/[0-9]{1,2}\s+[0-9]{1,2}:[0-9]{2}(:[0-9]{2})?(\.[0-9]{1,9})?\s*(am|pm|AM|PM)?$"
            )
            .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%Y/%m/%d %H:%M:%S")
            .or_else(|_| self.tz.datetime_from_str(input, "%Y/%m/%d %H:%M"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y/%m/%d %H:%M:%S%.f"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y/%m/%d %I:%M:%S %P"))
            .or_else(|_| self.tz.datetime_from_str(input, "%Y/%m/%d %I:%M %P"))
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // yyyy/mm/dd
    // - 2014/3/31
    // - 2014/03/31
    #[allow(deprecated)]
    fn slash_ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}/[0-9]{1,2}/[0-9]{1,2}$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now()
            .date()
            .and_time(self.default_time)?
            .with_timezone(self.tz);
        NaiveDate::parse_from_str(input, "%Y/%m/%d")
            .ok()
            .map(|parsed| parsed.and_time(now.time()))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // mm.dd.yyyy
    // - 3.31.2014
    // - 03.31.2014
    // - 08.21.71
    // yyyy.mm.dd
    // - 2014.03.30
    // - 2014.03
    #[allow(deprecated)]
    fn dot_mdy_or_ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[0-9]{1,4}.[0-9]{1,4}[0-9]{1,4}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now()
            .date()
            .and_time(self.default_time)?
            .with_timezone(self.tz);
        NaiveDate::parse_from_str(input, "%m.%d.%y")
            .or_else(|_| NaiveDate::parse_from_str(input, "%m.%d.%Y"))
            .or_else(|_| NaiveDate::parse_from_str(input, "%Y.%m.%d"))
            .or_else(|_| NaiveDate::parse_from_str(&format!("{}.{}", input, now.day()), "%Y.%m.%d"))
            .ok()
            .map(|parsed| parsed.and_time(now.time()))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // yymmdd hh:mm:ss mysql log
    // - 171113 14:14:20
    #[allow(deprecated)]
    fn mysql_log_timestamp(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[0-9]{6}\s+[0-9]{2}:[0-9]{2}:[0-9]{2}").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%y%m%d %H:%M:%S")
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // chinese yyyy mm dd hh mm ss
    // - 2014年04月08日11时25分18秒
    #[allow(deprecated)]
    fn chinese_ymd_hms(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^[0-9]{4}年[0-9]{2}月[0-9]{2}日[0-9]{2}时[0-9]{2}分[0-9]{2}秒$")
                    .unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        self.tz
            .datetime_from_str(input, "%Y年%m月%d日%H时%M分%S秒")
            .ok()
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }

    // chinese yyyy mm dd
    // - 2014年04月08日
    #[allow(deprecated)]
    fn chinese_ymd(&self, input: &str) -> Option<Result<DateTime<Utc>>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{4}年[0-9]{2}月[0-9]{2}日$").unwrap();
        }
        if !RE.is_match(input) {
            return None;
        }

        let now = Utc::now()
            .date()
            .and_time(self.default_time)?
            .with_timezone(self.tz);
        NaiveDate::parse_from_str(input, "%Y年%m月%d日")
            .ok()
            .map(|parsed| parsed.and_time(now.time()))
            .and_then(|datetime| self.tz.from_local_datetime(&datetime).single())
            .map(|at_tz| at_tz.with_timezone(&Utc))
            .map(Ok)
    }
}

/// Tries to parse `[-+]\d\d` continued by `\d\d`. Return FixedOffset if
/// possible. It can parse RFC 2822 legacy timezones. If offset cannot be
/// determined, -0000 will be returned.
///
/// The additional `colon` may be used to parse a mandatory or optional `:`
/// between hours and minutes, and should return a valid FixedOffset or `Err`
/// when parsing fails.
#[allow(deprecated)]
pub fn parse_timezone(s: &str) -> Result<FixedOffset> {
    let offset = if s.contains(':') {
        parse_offset_internal(s, colon_or_space, false)?
    } else {
        parse_offset_2822(s)?
    };
    Ok(FixedOffset::east(offset))
}

#[allow(deprecated)]
fn parse_offset_2822(s: &str) -> Result<i32> {
    // tries to parse legacy time zone names
    let upto = s
        .as_bytes()
        .iter()
        .position(|&c| c.is_ascii_alphabetic())
        // .position(|&c| !matches!(c, b'a'..=b'z' | b'A'..=b'Z'))
        .unwrap_or(s.len());
    if upto > 0 {
        let name = &s[..upto];
        let offset_hours = |o| Ok(o * 3600);
        if equals(name, "gmt") || equals(name, "ut") || equals(name, "utc") {
            offset_hours(0)
        } else if equals(name, "mez") {
            offset_hours(2)
        } else if equals(name, "mesz") {
            offset_hours(1)
        } else if equals(name, "edt") {
            offset_hours(-4)
        } else if equals(name, "est") || equals(name, "cdt") {
            offset_hours(-5)
        } else if equals(name, "cst") || equals(name, "mdt") {
            offset_hours(-6)
        } else if equals(name, "mst") || equals(name, "pdt") {
            offset_hours(-7)
        } else if equals(name, "pst") {
            offset_hours(-8)
        } else {
            Ok(0) // recommended by RFC 2822: consume but treat it as -0000
        }
    } else {
        let offset = parse_offset_internal(s, |s| Ok(s), false)?;
        Ok(offset)
    }
}

#[allow(deprecated)]
fn parse_offset_internal<F>(
    mut s: &str,
    mut consume_colon: F,
    allow_missing_minutes: bool,
) -> Result<i32>
where
    F: FnMut(&str) -> Result<&str>,
{
    let err_out_of_range = "input is out of range";
    let err_invalid = "input contains invalid characters";
    let err_too_short = "premature end of input";

    let digits = |s: &str| -> Result<(u8, u8)> {
        let b = s.as_bytes();
        if b.len() < 2 {
            Err(anyhow!(err_too_short))
        } else {
            Ok((b[0], b[1]))
        }
    };
    let negative = match s.as_bytes().first() {
        Some(&b'+') => false,
        Some(&b'-') => true,
        Some(_) => return Err(anyhow!(err_invalid)),
        None => return Err(anyhow!(err_too_short)),
    };
    s = &s[1..];

    // hours (00--99)
    let hours = match digits(s)? {
        (h1 @ b'0'..=b'9', h2 @ b'0'..=b'9') => i32::from((h1 - b'0') * 10 + (h2 - b'0')),
        _ => return Err(anyhow!(err_invalid)),
    };
    s = &s[2..];

    // colons (and possibly other separators)
    s = consume_colon(s)?;

    // minutes (00--59)
    // if the next two items are digits then we have to add minutes
    let minutes = if let Ok(ds) = digits(s) {
        match ds {
            (m1 @ b'0'..=b'5', m2 @ b'0'..=b'9') => i32::from((m1 - b'0') * 10 + (m2 - b'0')),
            (b'6'..=b'9', b'0'..=b'9') => return Err(anyhow!(err_out_of_range)),
            _ => return Err(anyhow!(err_invalid)),
        }
    } else if allow_missing_minutes {
        0
    } else {
        return Err(anyhow!(err_too_short));
    };

    let seconds = hours * 3600 + minutes * 60;
    Ok(if negative { -seconds } else { seconds })
}

/// Returns true when two slices are equal case-insensitively (in ASCII).
/// Assumes that the `pattern` is already converted to lower case.
fn equals(s: &str, pattern: &str) -> bool {
    let mut xs = s.as_bytes().iter().map(|&c| {
        match c {
            b'A'..=b'Z' => c + 32,
            _ => c,
        }
    });
    let mut ys = pattern.as_bytes().iter().cloned();
    loop {
        match (xs.next(), ys.next()) {
            (None, None) => return true,
            (None, _) | (_, None) => return false,
            (Some(x), Some(y)) if x != y => return false,
            _ => (),
        }
    }
}

/// Consumes any number (including zero) of colon or spaces.
fn colon_or_space(s: &str) -> Result<&str> {
    Ok(s.trim_start_matches(|c: char| c == ':' || c.is_whitespace()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn parse() {
        let test_cases = [
            ("-0800", FixedOffset::west(8 * 3600)),
            ("+10:00", FixedOffset::east(10 * 3600)),
            ("PST", FixedOffset::west(8 * 3600)),
            ("PDT", FixedOffset::west(7 * 3600)),
            ("UTC", FixedOffset::west(0)),
            ("GMT", FixedOffset::west(0)),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(parse_timezone(input).unwrap(), want, "parse/{input}")
        }
    }

    #[test]
    #[allow(deprecated)]
    fn unix_timestamp() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "0000000000",
                Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(),
            ),
            (
                "0000000000000",
                Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(),
            ),
            (
                "0000000000000000000",
                Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(),
            ),
            (
                "1511648546",
                Utc.with_ymd_and_hms(2017, 11, 25, 22, 22, 26).unwrap(),
            ),
            (
                "1620021848429",
                Utc.ymd(2021, 5, 3).and_hms_milli(6, 4, 8, 429),
            ),
            (
                "1620024872717915000",
                Utc.ymd(2021, 5, 3).and_hms_nano(6, 54, 32, 717915000),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.unix_timestamp(input).unwrap().unwrap(),
                want,
                "unix_timestamp/{input}",
            )
        }
        assert!(parse.unix_timestamp("15116").is_none());
        assert!(parse
            .unix_timestamp("16200248727179150001620024872717915000")
            .is_none());
        assert!(parse.unix_timestamp("not-a-ts").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn rfc3339() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "2021-05-01T01:17:02.604456Z",
                Utc.ymd(2021, 5, 1).and_hms_nano(1, 17, 2, 604456000),
            ),
            (
                "2017-11-25T22:34:50Z",
                Utc.with_ymd_and_hms(2017, 11, 25, 22, 34, 50).unwrap(),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.rfc3339(input).unwrap().unwrap(),
                want,
                "rfc3339/{input}",
            )
        }
        assert!(parse.rfc3339("2017-11-25 22:34:50").is_none());
        assert!(parse.rfc3339("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn rfc2822() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "Wed, 02 Jun 2021 06:31:39 GMT",
                Utc.with_ymd_and_hms(2021, 6, 2, 6, 31, 39).unwrap(),
            ),
            (
                "Wed, 02 Jun 2021 06:31:39 PDT",
                Utc.with_ymd_and_hms(2021, 6, 2, 13, 31, 39).unwrap(),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.rfc2822(input).unwrap().unwrap(),
                want,
                "rfc2822/{input}",
            )
        }
        assert!(parse.rfc2822("02 Jun 2021 06:31:39").is_none());
        assert!(parse.rfc2822("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn postgres_timestamp() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "2019-11-29 08:08-08",
                Utc.with_ymd_and_hms(2019, 11, 29, 16, 8, 0).unwrap(),
            ),
            (
                "2019-11-29 08:08:05-08",
                Utc.with_ymd_and_hms(2019, 11, 29, 16, 8, 5).unwrap(),
            ),
            (
                "2021-05-02 23:31:36.0741-07",
                Utc.ymd(2021, 5, 3).and_hms_micro(6, 31, 36, 74100),
            ),
            (
                "2021-05-02 23:31:39.12689-07",
                Utc.ymd(2021, 5, 3).and_hms_micro(6, 31, 39, 126890),
            ),
            (
                "2019-11-29 08:15:47.624504-08",
                Utc.ymd(2019, 11, 29).and_hms_micro(16, 15, 47, 624504),
            ),
            (
                "2017-07-19 03:21:51+00:00",
                Utc.with_ymd_and_hms(2017, 7, 19, 3, 21, 51).unwrap(),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.postgres_timestamp(input).unwrap().unwrap(),
                want,
                "postgres_timestamp/{input}",
            )
        }
        assert!(parse.postgres_timestamp("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn ymd_hms() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "2021-04-30 21:14",
                Utc.with_ymd_and_hms(2021, 4, 30, 21, 14, 0).unwrap(),
            ),
            (
                "2021-04-30 21:14:10",
                Utc.with_ymd_and_hms(2021, 4, 30, 21, 14, 10).unwrap(),
            ),
            (
                "2021-04-30 21:14:10.052282",
                Utc.ymd(2021, 4, 30).and_hms_micro(21, 14, 10, 52282),
            ),
            (
                "2014-04-26 05:24:37 PM",
                Utc.with_ymd_and_hms(2014, 4, 26, 17, 24, 37).unwrap(),
            ),
            (
                "2014-04-26 17:24:37.123",
                Utc.ymd(2014, 4, 26).and_hms_milli(17, 24, 37, 123),
            ),
            (
                "2014-04-26 17:24:37.3186369",
                Utc.ymd(2014, 4, 26).and_hms_nano(17, 24, 37, 318636900),
            ),
            (
                "2012-08-03 18:31:59.257000000",
                Utc.ymd(2012, 8, 3).and_hms_nano(18, 31, 59, 257000000),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.ymd_hms(input).unwrap().unwrap(),
                want,
                "ymd_hms/{input}",
            )
        }
        assert!(parse.ymd_hms("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn ymd_hms_z() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "2017-11-25 13:31:15 PST",
                Utc.with_ymd_and_hms(2017, 11, 25, 21, 31, 15).unwrap(),
            ),
            (
                "2017-11-25 13:31 PST",
                Utc.with_ymd_and_hms(2017, 11, 25, 21, 31, 0).unwrap(),
            ),
            (
                "2014-12-16 06:20:00 UTC",
                Utc.with_ymd_and_hms(2014, 12, 16, 6, 20, 0).unwrap(),
            ),
            (
                "2014-12-16 06:20:00 GMT",
                Utc.with_ymd_and_hms(2014, 12, 16, 6, 20, 0).unwrap(),
            ),
            (
                "2014-04-26 13:13:43 +0800",
                Utc.with_ymd_and_hms(2014, 4, 26, 5, 13, 43).unwrap(),
            ),
            (
                "2014-04-26 13:13:44 +09:00",
                Utc.with_ymd_and_hms(2014, 4, 26, 4, 13, 44).unwrap(),
            ),
            (
                "2012-08-03 18:31:59.257000000 +0000",
                Utc.ymd(2012, 8, 3).and_hms_nano(18, 31, 59, 257000000),
            ),
            (
                "2015-09-30 18:48:56.35272715 UTC",
                Utc.ymd(2015, 9, 30).and_hms_nano(18, 48, 56, 352727150),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.ymd_hms_z(input).unwrap().unwrap(),
                want,
                "ymd_hms_z/{input}",
            )
        }
        assert!(parse.ymd_hms_z("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn ymd() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [(
            "2021-02-21",
            Utc.ymd(2021, 2, 21).and_time(Utc::now().time()),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "ymd/{input}",
            )
        }
        assert!(parse.ymd("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn ymd_z() {
        let parse = Parse::new(&Utc, Utc::now().time());
        let now_at_pst = Utc::now().with_timezone(&FixedOffset::west(8 * 3600));
        let now_at_cst = Utc::now().with_timezone(&FixedOffset::east(8 * 3600));

        let test_cases = [
            (
                "2021-02-21 PST",
                FixedOffset::west(8 * 3600)
                    .ymd(2021, 2, 21)
                    .and_time(now_at_pst.time())
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "2021-02-21 UTC",
                FixedOffset::west(0)
                    .ymd(2021, 2, 21)
                    .and_time(Utc::now().time())
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "2020-07-20+08:00",
                FixedOffset::east(8 * 3600)
                    .ymd(2020, 7, 20)
                    .and_time(now_at_cst.time())
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .ymd_z(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "ymd_z/{input}",
            )
        }
        assert!(parse.ymd_z("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn hms() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "01:06:06",
                Utc::now().date().and_time(NaiveTime::from_hms(1, 6, 6)),
            ),
            (
                "4:00pm",
                Utc::now().date().and_time(NaiveTime::from_hms(16, 0, 0)),
            ),
            (
                "6:00 AM",
                Utc::now().date().and_time(NaiveTime::from_hms(6, 0, 0)),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.hms(input).unwrap().unwrap(),
                want.unwrap(),
                "hms/{input}",
            )
        }
        assert!(parse.hms("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn hms_z() {
        let parse = Parse::new(&Utc, Utc::now().time());
        let now_at_pst = Utc::now().with_timezone(&FixedOffset::west(8 * 3600));

        let test_cases = [
            (
                "01:06:06 PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&now_at_pst.date().naive_local())
                    .and_time(NaiveTime::from_hms(1, 6, 6))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "4:00pm PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&now_at_pst.date().naive_local())
                    .and_time(NaiveTime::from_hms(16, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "6:00 AM PST",
                FixedOffset::west(8 * 3600)
                    .from_local_date(&now_at_pst.date().naive_local())
                    .and_time(NaiveTime::from_hms(6, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
            (
                "6:00pm UTC",
                FixedOffset::west(0)
                    .from_local_date(&Utc::now().date().naive_local())
                    .and_time(NaiveTime::from_hms(18, 0, 0))
                    .map(|dt| dt.with_timezone(&Utc)),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.hms_z(input).unwrap().unwrap(),
                want.unwrap(),
                "hms_z/{input}",
            )
        }
        assert!(parse.hms_z("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn month_ymd() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [(
            "2021-Feb-21",
            Utc.ymd(2021, 2, 21).and_time(Utc::now().time()),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .month_ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "month_ymd/{input}",
            )
        }
        assert!(parse.month_ymd("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn month_md_hms() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "May 6 at 9:24 PM",
                Utc.with_ymd_and_hms(Utc::now().year(), 5, 6, 21, 24, 0)
                    .unwrap(),
            ),
            (
                "May 27 02:45:27",
                Utc.with_ymd_and_hms(Utc::now().year(), 5, 27, 2, 45, 27)
                    .unwrap(),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.month_md_hms(input).unwrap().unwrap(),
                want,
                "month_md_hms/{input}",
            )
        }
        assert!(parse.month_md_hms("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn month_mdy_hms() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "May 8, 2009 5:57:51 PM",
                Utc.with_ymd_and_hms(2009, 5, 8, 17, 57, 51).unwrap(),
            ),
            (
                "September 17, 2012 10:09am",
                Utc.with_ymd_and_hms(2012, 9, 17, 10, 9, 0).unwrap(),
            ),
            (
                "September 17, 2012, 10:10:09",
                Utc.with_ymd_and_hms(2012, 9, 17, 10, 10, 9).unwrap(),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.month_mdy_hms(input).unwrap().unwrap(),
                want,
                "month_mdy_hms/{input}",
            )
        }
        assert!(parse.month_mdy_hms("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn month_mdy_hms_z() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "May 02, 2021 15:51:31 UTC",
                Utc.with_ymd_and_hms(2021, 5, 2, 15, 51, 31).unwrap(),
            ),
            (
                "May 02, 2021 15:51 UTC",
                Utc.with_ymd_and_hms(2021, 5, 2, 15, 51, 0).unwrap(),
            ),
            (
                "May 26, 2021, 12:49 AM PDT",
                Utc.with_ymd_and_hms(2021, 5, 26, 7, 49, 0).unwrap(),
            ),
            (
                "September 17, 2012 at 10:09am PST",
                Utc.with_ymd_and_hms(2012, 9, 17, 18, 9, 0).unwrap(),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.month_mdy_hms_z(input).unwrap().unwrap(),
                want,
                "month_mdy_hms_z/{input}",
            )
        }
        assert!(parse.month_mdy_hms_z("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn month_mdy() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "May 25, 2021",
                Utc.ymd(2021, 5, 25).and_time(Utc::now().time()),
            ),
            (
                "oct 7, 1970",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "oct 7, 70",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "oct. 7, 1970",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "oct. 7, 70",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "October 7, 1970",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .month_mdy(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "month_mdy/{input}",
            )
        }
        assert!(parse.month_mdy("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn month_dmy_hms() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "12 Feb 2006, 19:17",
                Utc.with_ymd_and_hms(2006, 2, 12, 19, 17, 0).unwrap(),
            ),
            (
                "12 Feb 2006 19:17",
                Utc.with_ymd_and_hms(2006, 2, 12, 19, 17, 0).unwrap(),
            ),
            (
                "14 May 2019 19:11:40.164",
                Utc.ymd(2019, 5, 14).and_hms_milli(19, 11, 40, 164),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.month_dmy_hms(input).unwrap().unwrap(),
                want,
                "month_dmy_hms/{input}",
            )
        }
        assert!(parse.month_dmy_hms("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn month_dmy() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            ("7 oct 70", Utc.ymd(1970, 10, 7).and_time(Utc::now().time())),
            (
                "7 oct 1970",
                Utc.ymd(1970, 10, 7).and_time(Utc::now().time()),
            ),
            (
                "03 February 2013",
                Utc.ymd(2013, 2, 3).and_time(Utc::now().time()),
            ),
            (
                "1 July 2013",
                Utc.ymd(2013, 7, 1).and_time(Utc::now().time()),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .month_dmy(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "month_dmy/{input}",
            )
        }
        assert!(parse.month_dmy("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn slash_mdy_hms() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "4/8/2014 22:05",
                Utc.with_ymd_and_hms(2014, 4, 8, 22, 5, 0).unwrap(),
            ),
            (
                "04/08/2014 22:05",
                Utc.with_ymd_and_hms(2014, 4, 8, 22, 5, 0).unwrap(),
            ),
            (
                "4/8/14 22:05",
                Utc.with_ymd_and_hms(2014, 4, 8, 22, 5, 0).unwrap(),
            ),
            (
                "04/2/2014 03:00:51",
                Utc.with_ymd_and_hms(2014, 4, 2, 3, 0, 51).unwrap(),
            ),
            (
                "8/8/1965 12:00:00 AM",
                Utc.with_ymd_and_hms(1965, 8, 8, 0, 0, 0).unwrap(),
            ),
            (
                "8/8/1965 01:00:01 PM",
                Utc.with_ymd_and_hms(1965, 8, 8, 13, 0, 1).unwrap(),
            ),
            (
                "8/8/1965 01:00 PM",
                Utc.with_ymd_and_hms(1965, 8, 8, 13, 0, 0).unwrap(),
            ),
            (
                "8/8/1965 1:00 PM",
                Utc.with_ymd_and_hms(1965, 8, 8, 13, 0, 0).unwrap(),
            ),
            (
                "8/8/1965 12:00 AM",
                Utc.with_ymd_and_hms(1965, 8, 8, 0, 0, 0).unwrap(),
            ),
            (
                "4/02/2014 03:00:51",
                Utc.with_ymd_and_hms(2014, 4, 2, 3, 0, 51).unwrap(),
            ),
            (
                "03/19/2012 10:11:59",
                Utc.with_ymd_and_hms(2012, 3, 19, 10, 11, 59).unwrap(),
            ),
            (
                "03/19/2012 10:11:59.3186369",
                Utc.ymd(2012, 3, 19).and_hms_nano(10, 11, 59, 318636900),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.slash_mdy_hms(input).unwrap().unwrap(),
                want,
                "slash_mdy_hms/{input}",
            )
        }
        assert!(parse.slash_mdy_hms("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn slash_mdy() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "3/31/2014",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            (
                "03/31/2014",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            ("08/21/71", Utc.ymd(1971, 8, 21).and_time(Utc::now().time())),
            ("8/1/71", Utc.ymd(1971, 8, 1).and_time(Utc::now().time())),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .slash_mdy(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "slash_mdy/{input}",
            )
        }
        assert!(parse.slash_mdy("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn slash_ymd_hms() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "2014/4/8 22:05",
                Utc.with_ymd_and_hms(2014, 4, 8, 22, 5, 0).unwrap(),
            ),
            (
                "2014/04/08 22:05",
                Utc.with_ymd_and_hms(2014, 4, 8, 22, 5, 0).unwrap(),
            ),
            (
                "2014/04/2 03:00:51",
                Utc.with_ymd_and_hms(2014, 4, 2, 3, 0, 51).unwrap(),
            ),
            (
                "2014/4/02 03:00:51",
                Utc.with_ymd_and_hms(2014, 4, 2, 3, 0, 51).unwrap(),
            ),
            (
                "2012/03/19 10:11:59",
                Utc.with_ymd_and_hms(2012, 3, 19, 10, 11, 59).unwrap(),
            ),
            (
                "2012/03/19 10:11:59.3186369",
                Utc.ymd(2012, 3, 19).and_hms_nano(10, 11, 59, 318636900),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.slash_ymd_hms(input).unwrap().unwrap(),
                want,
                "slash_ymd_hms/{input}",
            )
        }
        assert!(parse.slash_ymd_hms("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn slash_ymd() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "2014/3/31",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            (
                "2014/03/31",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .slash_ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "slash_ymd/{input}",
            )
        }
        assert!(parse.slash_ymd("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn dot_mdy_or_ymd() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            // mm.dd.yyyy
            (
                "3.31.2014",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            (
                "03.31.2014",
                Utc.ymd(2014, 3, 31).and_time(Utc::now().time()),
            ),
            ("08.21.71", Utc.ymd(1971, 8, 21).and_time(Utc::now().time())),
            // yyyy.mm.dd
            (
                "2014.03.30",
                Utc.ymd(2014, 3, 30).and_time(Utc::now().time()),
            ),
            (
                "2014.03",
                Utc.ymd(2014, 3, Utc::now().day())
                    .and_time(Utc::now().time()),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .dot_mdy_or_ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "dot_mdy_or_ymd/{input}",
            )
        }
        assert!(parse.dot_mdy_or_ymd("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn mysql_log_timestamp() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            // yymmdd hh:mm:ss mysql log
            (
                "171113 14:14:20",
                Utc.with_ymd_and_hms(2017, 11, 13, 14, 14, 20).unwrap(),
            ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.mysql_log_timestamp(input).unwrap().unwrap(),
                want,
                "mysql_log_timestamp/{input}",
            )
        }
        assert!(parse.mysql_log_timestamp("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn chinese_ymd_hms() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [(
            "2014年04月08日11时25分18秒",
            Utc.with_ymd_and_hms(2014, 4, 8, 11, 25, 18).unwrap(),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.chinese_ymd_hms(input).unwrap().unwrap(),
                want,
                "chinese_ymd_hms/{input}",
            )
        }
        assert!(parse.chinese_ymd_hms("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn chinese_ymd() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [(
            "2014年04月08日",
            Utc.ymd(2014, 4, 8).and_time(Utc::now().time()),
        )];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse
                    .chinese_ymd(input)
                    .unwrap()
                    .unwrap()
                    .trunc_subsecs(0)
                    .with_second(0)
                    .unwrap(),
                want.unwrap().trunc_subsecs(0).with_second(0).unwrap(),
                "chinese_ymd/{input}",
            )
        }
        assert!(parse.chinese_ymd("not-date-time").is_none());
    }
    #[test]
    #[allow(deprecated)]
    fn ib_hms_z() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "20171125 13:31:15 PST",
                Utc.with_ymd_and_hms(2017, 11, 25, 21, 31, 15).unwrap(),
            ),
            (
                "20171125 13:31 PST",
                Utc.with_ymd_and_hms(2017, 11, 25, 21, 31, 0).unwrap(),
            ),
            (
                "20141216 06:20:00 UTC",
                Utc.with_ymd_and_hms(2014, 12, 16, 6, 20, 0).unwrap(),
            ),
            (
                "20141216 06:20:00 MEZ",
                Utc.with_ymd_and_hms(2014, 12, 16, 4, 20, 0).unwrap(),
            ),
            (
                "20141216-06:20:00 MEZ",
                Utc.with_ymd_and_hms(2014, 12, 16, 4, 20, 0).unwrap(),
            ),
            (
                "20141216 06:20:00 GMT",
                Utc.with_ymd_and_hms(2014, 12, 16, 6, 20, 0).unwrap(),
            ),
            (
                "20140426 13:13:43 +0800",
                Utc.with_ymd_and_hms(2014, 4, 26, 5, 13, 43).unwrap(),
            ),
            (
                "20140426 13:13:44 +09:00",
                Utc.with_ymd_and_hms(2014, 4, 26, 4, 13, 44).unwrap(),
            ),
            // (
            //     "20120803 18:31:59.257000000 +0000",
            //     Utc.ymd(2012, 8, 3).and_hms_nano(18, 31, 59, 257000000),
            // ),
            // (
            //     "20150930 18:48:56.35272715 UTC",
            //     Utc.ymd(2015, 9, 30).and_hms_nano(18, 48, 56, 352727150),
            // ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.ymd_hms_z_ib(input).unwrap().unwrap(),
                want,
                "ymd_hms_z_ib/{input}",
            )
        }
        assert!(parse.ymd_hms_z_ib("not-date-time").is_none());
    }
    #[test]
    #[allow(deprecated)]
    fn ymd_hms_timezone_ib() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "20220902 16:31:15 Europe/Berlin",
                Utc.with_ymd_and_hms(2022, 9, 2, 14, 31, 15).unwrap(),
            ),
            (
                "20220902-16:31:15 Europe/Berlin",
                Utc.with_ymd_and_hms(2022, 9, 2, 14, 31, 15).unwrap(),
            ),
            (
                "20220902-09:30:00 US/Eastern",
                Utc.with_ymd_and_hms(2022, 9, 2, 13, 30, 00).unwrap(),
            ),
            // (
            //     "20221202 16:31:15 Europe/Berlin",
            //     Utc.with_ymd_and_hms(2022, 12, 2,14, 31, 15).unwrap(),
            // ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.ymd_hms_timezone_ib(input).unwrap().unwrap(),
                want,
                "ymd_hms_timezone_ib/{input}",
            )
        }
        assert!(parse.ymd_hms_timezone_ib("not-date-time").is_none());
    }
    #[test]
    #[allow(deprecated)]
    fn hms_timezone_ib() {
        let parse = Parse::new(&Utc, Utc::now().time());

        let test_cases = [
            (
                "15:17:45 Europe/Berlin",
                Utc::now()
                    .date()
                    .and_time(NaiveTime::from_hms(13, 17, 45))
                    .unwrap(),
            ),
            // (
            //     "20221202 16:31:15 Europe/Berlin",
            //     Utc.with_ymd_and_hms(2022, 12, 2,14, 31, 15).unwrap(),
            // ),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                parse.hms_timezone_ib(input).unwrap().unwrap(),
                want,
                "hms_timezone_ib/{input}",
            )
        }
        assert!(parse.hms_timezone_ib("not-date-time").is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn timezone() {
        let test_cases = [
            ("Europe/Berlin", chrono_tz::Europe::Berlin),
            ("US/Eastern", chrono_tz::US::Eastern),
        ];

        for &(input, want) in test_cases.iter() {
            assert_eq!(
                chrono_tz::Tz::from_str(input.to_string().trim()).unwrap(),
                want,
                "timezone/{input}",
            )
        }
    }
}
