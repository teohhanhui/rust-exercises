use err_derive::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::num::ParseFloatError;
use std::process;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TemperatureUnit {
    fn symbol(&self) -> &str {
        match self {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Kelvin => "K",
        }
    }

    fn symbol_regex(&self) -> &Regex {
        const CELSIUS_PATTERN: &str = r"°?C";
        const FAHRENHEIT_PATTERN: &str = r"°?F";
        const KELVIN_PATTERN: &str = r"K";

        static CELSIUS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(CELSIUS_PATTERN).unwrap());
        static FAHRENHEIT_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(FAHRENHEIT_PATTERN).unwrap());
        static KELVIN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(KELVIN_PATTERN).unwrap());

        match self {
            TemperatureUnit::Celsius => &CELSIUS_REGEX,
            TemperatureUnit::Fahrenheit => &FAHRENHEIT_REGEX,
            TemperatureUnit::Kelvin => &KELVIN_REGEX,
        }
    }
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Temperature {
    value: f32,
    unit: TemperatureUnit,
}

impl Temperature {
    fn convert(self, unit: TemperatureUnit) -> Result<Self, TemperatureConversionError> {
        const CELSIUS_TO_FAHRENHEIT_RATIO: f32 = 1.8;
        const CELSIUS_TO_FAHRENHEIT_OFFSET: f32 = 32.0;
        const CELSIUS_TO_KELVIN_OFFSET: f32 = 273.15;

        let value = match (self.unit, unit) {
            (from, to) if from == to => return Ok(self),
            (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => {
                self.value * CELSIUS_TO_FAHRENHEIT_RATIO + CELSIUS_TO_FAHRENHEIT_OFFSET
            }
            (TemperatureUnit::Celsius, TemperatureUnit::Kelvin) => {
                self.value + CELSIUS_TO_KELVIN_OFFSET
            }
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => {
                (self.value - CELSIUS_TO_FAHRENHEIT_OFFSET) / CELSIUS_TO_FAHRENHEIT_RATIO
            }
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin) => {
                self.convert(TemperatureUnit::Celsius)?
                    .convert(TemperatureUnit::Kelvin)?
                    .value
            }
            (TemperatureUnit::Kelvin, TemperatureUnit::Celsius) => {
                self.value - CELSIUS_TO_KELVIN_OFFSET
            }
            (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit) => {
                self.convert(TemperatureUnit::Celsius)?
                    .convert(TemperatureUnit::Fahrenheit)?
                    .value
            }
            (from, to) => return Err(TemperatureConversionError::NotSupported { from, to }),
        };

        Ok(Self { value, unit })
    }
}

impl FromStr for Temperature {
    type Err = ParseTemperatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseTemperatureError::Empty);
        }

        static TEMPERATURE_PATTERN: Lazy<String> = Lazy::new(|| {
            format!(
                r"(?i)^(?P<value>(?:-|−)?\d+(?:\.\d+)?)\s?(?P<unit>{})$",
                TemperatureUnit::iter()
                    .map(|u| u.symbol_regex().to_string())
                    .collect::<Vec<String>>()
                    .join("|")
            )
        });
        static TEMPERATURE_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(&TEMPERATURE_PATTERN).unwrap());

        let caps = TEMPERATURE_REGEX
            .captures(s)
            .ok_or_else(|| ParseTemperatureError::Invalid)?;
        let value = &caps["value"];
        let value: f32 =
            value
                .replace("−", "-")
                .parse()
                .map_err(|err| ParseTemperatureError::Parse {
                    source: err,
                    value: value.to_owned(),
                })?;
        let unit = &caps["unit"];
        let unit = TemperatureUnit::iter()
            .find(|u| u.symbol_regex().is_match(unit))
            .unwrap();

        Ok(Temperature { value, unit })
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

#[derive(Debug, Error)]
enum TemperatureConversionError {
    #[error(display = "conversion from {} to {} is not supported", from, to)]
    NotSupported {
        from: TemperatureUnit,
        to: TemperatureUnit,
    },
}

#[derive(Debug, Error)]
enum ParseTemperatureError {
    #[error(display = "cannot parse temperature from empty string")]
    Empty,
    #[error(display = "invalid temperature literal")]
    Invalid,
    #[error(display = "invalid temperature value: {}", value)]
    Parse {
        #[error(source)]
        source: ParseFloatError,
        value: String,
    },
}

fn main() {
    const DEFAULT_INPUT: &str = "36.9C";

    println!("Temperature [{}]:", DEFAULT_INPUT);

    let input = match io::stdin().lock().lines().next() {
        Some(Ok(line)) => line,
        Some(Err(err)) => panic!("Failed to read line: {:?}", err),
        None => {
            eprintln!("No input");
            process::exit(1);
        }
    };
    let input = if input.is_empty() {
        DEFAULT_INPUT
    } else {
        input.trim()
    };
    let input: Temperature = input.parse().unwrap_or_else(|err| {
        eprintln!("Invalid input: {}", err);
        process::exit(1);
    });

    println!("{}", input);

    let outputs = TemperatureUnit::iter()
        .filter(|u| *u != input.unit)
        .map(|u| input.convert(u))
        .filter(|r| match r {
            Ok(_) => true,
            Err(TemperatureConversionError::NotSupported { .. }) => false,
            // Err(err) => panic!("Temperature conversion failed: {:?}", err),
        })
        .collect::<Result<Vec<Temperature>, TemperatureConversionError>>()
        .expect("Temperature conversion failed");

    for output in outputs {
        println!("= {}", output);
    }
}
