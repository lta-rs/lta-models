//! Utilities for lta-rs
use serde::{Deserialize, Serialize};

/// Coordinate on the map
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub long: f64,
}

impl Coordinates {
    pub fn new(lat: f64, long: f64) -> Self {
        Coordinates { lat, long }
    }
}

/// Starting and ending location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub start: Coordinates,
    pub end: Coordinates,
}

impl Location {
    pub fn new(start_lat: f64, start_lang: f64, end_lat: f64, end_lang: f64) -> Self {
        Location {
            start: Coordinates::new(start_lat, start_lang),
            end: Coordinates::new(end_lat, end_lang),
        }
    }

    pub fn from_coords(start: Coordinates, end: Coordinates) -> Self {
        Location { start, end }
    }
}

/// Regex patterns
pub mod regex {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref BUS_FREQ_RE: Regex =
            Regex::new(r"^(\d{1,3})?-?(\d{1,3})?$").unwrap();

        pub static ref CARPARK_COORDS_RE: Regex =
            Regex::new(r"^([+-]?([0-9]*[.])?[0-9]+) ([+-]?([0-9]*[.])?[0-9]+)$").unwrap();

        pub static ref SPEED_BAND_RE: Regex =
            Regex::new(r"^([+-]?([0-9]*[.])?[0-9]+) ([+-]?([0-9]*[.])?[0-9]+) ([+-]?([0-9]*[.])?[0-9]+) ([+-]?([0-9]*[.])?[0-9]+)$")
                .unwrap();
    }
}

/// Utils for date types
pub mod serde_date {
    pub mod ymd_hms_option {
        use lazy_static::lazy_static;
        use serde::{Deserialize, Deserializer, Serializer};
        use time::{
            format_description::{self, FormatItem},
            OffsetDateTime,
        };

        lazy_static! {
            static ref FORMAT: Vec<FormatItem<'static>> = format_description::parse_borrowed::<2>(
                "[year]-[month]-[day] [hour]:[minute]:[second]"
            )
            .unwrap();
        }

        /// # Errors
        /// Infallible, depending on the type of `date` is provided
        pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match date {
                Some(dt) => {
                    let s = dt.format(&FORMAT).unwrap();
                    serializer.serialize_str(&s)
                }
                None => serializer.serialize_none(),
            }
        }

        /// # Errors
        /// Fails if invalid UTC datetime is provided
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            OffsetDateTime::parse(&s, &FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }

    pub mod str_time_option {
        use serde::{Deserialize, Deserializer, Serializer};
        use time::Time;

        /// # Errors
        /// Fails when data cant be deserialized to String
        pub fn ser_str_time_opt<S>(
            opt_time: &Option<Time>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match opt_time {
                Some(time) => {
                    let hr = time.hour();
                    let min = time.minute();
                    let mut sec_str = String::with_capacity(1);
                    sec_str.push('0');

                    let s = [hr.to_string(), min.to_string(), sec_str].join(":");

                    serializer.serialize_str(&s)
                }
                None => serializer.serialize_none(),
            }
        }

        /// # Errors
        /// Fails if date cannot be parsed in this format HHMM
        pub fn de_str_time_opt_erp<'de, D>(deserializer: D) -> Result<Option<Time>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            if s.eq("-") {
                return Ok(None);
            }
            let hr = &mut s[0..1].parse().map_err(serde::de::Error::custom)?;
            let min = &s[3..4].parse().map_err(serde::de::Error::custom)?;
            if *hr == 24 {
                *hr = 0
            }

            Time::from_hms(*hr, *min, 0)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }

        /// # Errors
        /// Fails if date cannot be parsed in this format HH:MM
        pub fn de_str_time_opt_br<'de, D>(deserializer: D) -> Result<Option<Time>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            if s.eq("-") {
                return Ok(None);
            }
            let hr = &mut s[0..1].parse().map_err(serde::de::Error::custom)?;
            let min = &s[2..3].parse().map_err(serde::de::Error::custom)?;
            if *hr == 24 {
                *hr = 0
            }

            Time::from_hms(*hr, *min, 0)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }

    pub mod str_date {
        use lazy_static::lazy_static;
        use serde::{Deserialize, Deserializer, Serializer};
        use time::{
            format_description::{self, FormatItem},
            Date,
        };

        lazy_static! {
            static ref FORMAT: Vec<FormatItem<'static>> =
                format_description::parse_borrowed::<2>("[year]-[month]-[day]").unwrap();
        }

        /// # Errors
        /// Fails when data cant be deserialized to String
        pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = date.format(&FORMAT).unwrap();
            serializer.serialize_str(&s)
        }

        /// # Errors
        /// Fails when date isn't the same format as [`crate::utils::serde_date::str_date::FORMAT`]
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Date::parse(&s, &FORMAT).map_err(serde::de::Error::custom)
        }
    }
}

/// Deserialisation utils
pub mod de {
    use std::fmt;
    use std::fmt::Display;
    use std::iter::FromIterator;
    use std::marker::PhantomData as Phantom;
    use std::str::FromStr;

    use crate::utils::{regex::*, Coordinates, Location};
    use serde::de::{self, Visitor};
    use serde::{Deserialize, Deserializer};
    use std::fmt::Formatter;

    /// Error for wrapped data
    pub struct WrapErr;

    /// Separator trait
    pub trait Sep {
        fn delimiter() -> &'static str;
    }

    impl Display for WrapErr {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "StringWrapErr")
        }
    }

    /// # Errors
    /// Fails when data cant be deserialized to String. Returns None if data is invalid
    pub fn treat_error_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        // let value: Value = Deserialize::deserialize(deserializer)?;
        Ok(T::deserialize(deserializer).ok())
    }

    /// Simple conversion of `Y`,`Yes` and `N`, `No` to boolean
    /// # Errors
    /// Fails when data cant be deserialized to String. Returns `false` if data is invalid
    pub fn from_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_ref() {
            "Y" | "Yes" => Ok(true),
            _ => Ok(false),
        }
    }

    /// To be used when coordinates are space separated
    /// in a string and you would like to convert them to a Coordinates
    /// structure.
    /// # Errors
    /// Fails when data cant be deserialized to String. Returns None if data is invalid
    pub fn from_str_to_coords<'de, D>(deserializer: D) -> Result<Option<Coordinates>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() || !CARPARK_COORDS_RE.is_match(s.as_str()) {
            return Ok(None);
        }

        let caps = CARPARK_COORDS_RE.captures(&s).unwrap();
        let lat = caps.get(1).map_or(0.0, |m| m.as_str().parse().unwrap());
        let long = caps.get(3).map_or(0.0, |m| m.as_str().parse().unwrap());

        Ok(Some(Coordinates::new(lat, long)))
    }

    /// # Errors
    /// Fails when data cant be deserialized to String. Returns None if data is invalid
    #[cfg(not(feature = "fastfloat"))]
    pub fn from_str_loc_to_loc<'de, D>(deserializer: D) -> Result<Option<Location>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() || !SPEED_BAND_RE.is_match(s.as_str()) {
            return Ok(None);
        }

        let caps = SPEED_BAND_RE.captures(&s).unwrap();
        let lat_start = caps.get(1).map_or(0.0, |m| m.as_str().parse().unwrap());
        let long_start = caps.get(3).map_or(0.0, |m| m.as_str().parse().unwrap());
        let lat_end = caps.get(5).map_or(0.0, |m| m.as_str().parse().unwrap());
        let long_end = caps.get(7).map_or(0.0, |m| m.as_str().parse().unwrap());

        Ok(Some(Location::new(
            lat_start, long_start, lat_end, long_end,
        )))
    }

    /// Generic implementation of `FromStr`
    /// # Errors
    /// Fails when data cant be deserialized to String
    pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str(&s).map_err(de::Error::custom)
    }

    /// Uses fast-float crate to deserialise float string instead of using
    /// the standard library's `FromStr`
    /// # Errors
    /// Fails when data cant be deserialized to String and when data is an invalid float string
    #[cfg(feature = "fastfloat")]
    pub fn from_str_fast_float<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: fast_float::FastFloat,
    {
        let s = String::deserialize(deserializer)?;
        fast_float::parse(s).map_err(|_| de::Error::custom("Invalid float string!"))
    }

    /// # Errors
    /// Fails when data cant be deserialized to String. Returns None on error
    pub fn from_str_error_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(T::from_str(&s).ok())
    }

    /// # Errors
    /// Fails when data cant be deserialized to String
    pub fn delimited<'de, V, T, D>(deserializer: D) -> Result<V, D::Error>
    where
        V: FromIterator<T>,
        T: FromStr + Sep,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        struct DelimitedBy<V, T>(Phantom<V>, Phantom<T>);

        impl<'de, V, T> Visitor<'de> for DelimitedBy<V, T>
        where
            V: FromIterator<T>,
            T: FromStr + Sep,
            T::Err: Display,
        {
            type Value = V;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("string containing / separated elements")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                s.split(T::delimiter())
                    .map(FromStr::from_str)
                    .collect::<Result<_, _>>()
                    .map_err(de::Error::custom)
            }
        }

        let visitor = DelimitedBy(Phantom, Phantom);
        deserializer.deserialize_str(visitor)
    }
}
