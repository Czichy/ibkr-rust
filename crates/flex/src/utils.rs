pub mod de {

    use std::{fmt::Display, str::FromStr};

    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{de, Deserialize, Deserializer};

    // You can use this deserializer for any type that implements FromStr
    // and the FromStr::Err implements Display
    pub fn deserialize_from_str<'de, S, D>(deserializer: D) -> core::result::Result<S, D::Error>
    where
        S: FromStr,      // Required for S::from_str...
        S::Err: Display, // Required for .map_err(de::Error::custom)
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        S::from_str(&s).map_err(de::Error::custom)
    }

    pub fn deserialize_option<'de, T, D>(
        deserializer: D,
    ) -> core::result::Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        if let Ok(v) = T::deserialize(deserializer) {
            return Ok(Some(v));
        }
        Ok(None)
    }

    pub fn deserialize_option_from_str<'de, S, D>(
        deserializer: D,
    ) -> core::result::Result<Option<S>, D::Error>
    where
        S: FromStr,      // Required for S::from_str...
        S::Err: Display, // Required for .map_err(de::Error::custom)
        // T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(S::from_str(&s).map_err(de::Error::custom)?))
        }
    }

    pub fn deserialize_empty_string_is_none<'de, D>(
        deserializer: D,
    ) -> core::result::Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }

    pub fn naive_date_time_from_str<'de, D>(
        deserializer: D,
    ) -> core::result::Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_fmt = &vec![
            "%Y-%m-%d", "%Y%m%d", "%m/%d/%Y", "%m/%d/%y", "%d/%m/%Y", "%d/%m/%y", "%d-%m-%y",
        ];
        let delim_fmt = &vec![",", ";", " ", ""];
        let time_fmt = &vec!["%H:%M:%S", "%H%M%S"];
        let date_time_fmt = date_fmt
            .iter()
            .flat_map(move |&a| {
                delim_fmt
                    .iter()
                    .flat_map(move |&b| time_fmt.iter().map(move |&c| format!("{}{}{}", a, b, c)))
            })
            .collect::<Vec<_>>();

        let s: String = Deserialize::deserialize(deserializer)?;

        for fmt in date_time_fmt.iter() {
            let parsed_date = NaiveDateTime::parse_from_str(&s, fmt);
            if let Ok(parsed_date) = parsed_date {
                return Ok(parsed_date);
            }
        }
        // NaiveDateTime::parse_from_str(&s,
        // "%Y-%m-%d;%H:%M:%S").map_err(de::Error::custom)
        Err(de::Error::custom(format!(
            "unknown date time format: {}",
            s
        )))
    }
    pub fn some_naive_date_time_from_str<'de, D>(
        deserializer: D,
    ) -> core::result::Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }
        let date_fmt = &vec![
            "%Y-%m-%d", "%Y%m%d", "%m/%d/%Y", "%m/%d/%y", "%d/%m/%Y", "%d/%m/%y", "%d-%m-%y",
        ];
        let delim_fmt = &vec![",", ";", " ", ""];
        let time_fmt = &vec!["%H:%M:%S", "%H%M%S"];
        let date_time_fmt = date_fmt
            .iter()
            .flat_map(move |&a| {
                delim_fmt
                    .iter()
                    .flat_map(move |&b| time_fmt.iter().map(move |&c| format!("{}{}{}", a, b, c)))
            })
            .collect::<Vec<_>>();

        for fmt in date_time_fmt.iter() {
            let parsed_date = NaiveDateTime::parse_from_str(&s, fmt);
            if let Ok(parsed_date) = parsed_date {
                return Ok(Some(parsed_date));
            }
        }
        // NaiveDateTime::parse_from_str(&s,
        // "%Y-%m-%d;%H:%M:%S").map_err(de::Error::custom)
        Err(de::Error::custom(format!(
            "unknown date time format: {}",
            s
        )))
    }
    pub fn naive_date_from_str<'de, D>(deserializer: D) -> core::result::Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_fmt = &vec![
            "%Y-%m-%d", "%Y%m%d", "%m/%d/%Y", "%m/%d/%y", "%d/%m/%Y", "%d/%m/%y", "%d-%m-%y",
        ];

        let s: String = Deserialize::deserialize(deserializer)?;

        for fmt in date_fmt.iter() {
            let parsed_date = NaiveDate::parse_from_str(&s, fmt);
            if let Ok(parsed_date) = parsed_date {
                return Ok(parsed_date);
            }
        }
        Err(de::Error::custom(format!(
            "unknown date time format: {}",
            s
        )))
    }
    pub fn some_naive_date_from_str<'de, D>(
        deserializer: D,
    ) -> core::result::Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }
        let date_fmt = &vec![
            "%Y-%m-%d", "%Y%m%d", "%m/%d/%Y", "%m/%d/%y", "%d/%m/%Y", "%d/%m/%y", "%d-%m-%y",
        ];

        for fmt in date_fmt.iter() {
            let parsed_date = NaiveDate::parse_from_str(&s, fmt);
            if let Ok(parsed_date) = parsed_date {
                return Ok(Some(parsed_date));
            }
        }
        Err(de::Error::custom(format!(
            "unknown date time format: {}",
            s
        )))
    }
}
