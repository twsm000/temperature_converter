use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum TemperatureErrorKind {
    NotNumeric,
    ScaleUnknown,
    Inconvertible,
}

#[derive(Debug, Clone)]
pub struct ParseTemperatureError {
    kind: TemperatureErrorKind,
}

impl ParseTemperatureError {
    #[doc(hidden)]
    pub fn __description(&self) -> &str {
        match self.kind {
            TemperatureErrorKind::NotNumeric => "not a numeric value",
            TemperatureErrorKind::ScaleUnknown => "scale unknown",
            TemperatureErrorKind::Inconvertible => "string is empty"
        }
    }
}

impl std::fmt::Display for ParseTemperatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(self.__description(), f)
    }
}

impl std::error::Error for ParseTemperatureError {}

#[derive(Copy, Clone)]
enum Scale {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Scale::Celsius => 'C',
            Scale::Fahrenheit => 'F',
            Scale::Kelvin => 'K'
        };

        write!(f, "{}", output)
    }
}

struct Temperature {
    value: f64,
    scale: Scale,
    convert_to: Scale,
}

impl Temperature {
    const K: f64 = 273.15;

    fn f_c(f: f64) -> f64 {
        (f - 32.0) * 5.0 / 9.0
    }

    fn c_f(c: f64) -> f64 {
        (c * 9.0 / 5.0) + 32.0
    }

    fn c_k(c: f64, k: f64) -> f64 {
        c + k
    }

    pub fn convert(&self) -> f64 {
        return match (self.scale, self.convert_to) {
            (Scale::Celsius, Scale::Kelvin) => Temperature::c_k(self.value, Temperature::K),
            (Scale::Celsius, Scale::Fahrenheit) => Temperature::c_f(self.value),
            (Scale::Fahrenheit, Scale::Celsius) => Temperature::f_c(self.value),
            (Scale::Fahrenheit, Scale::Kelvin) => Temperature::c_k(Temperature::f_c(self.value), Temperature::K),
            (Scale::Kelvin, Scale::Celsius) => Temperature::c_k(self.value, -Temperature::K),
            (Scale::Kelvin, Scale::Fahrenheit) => Temperature::c_f(Temperature::c_k(self.value, -Temperature::K)),
            _ => self.value
        };
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} => {}{}", self.value, self.scale, self.convert(), self.convert_to)
    }
}

impl FromStr for Temperature {
    type Err = ParseTemperatureError;

    fn from_str(temp: &str) -> Result<Self, Self::Err> {
        let temp = temp.trim();
        if temp.is_empty() {
            return Err(ParseTemperatureError { kind: TemperatureErrorKind::Inconvertible });
        }
        if temp.len() == 1 {
            return match f64::from_str(temp) {
                Ok(_) => Err(ParseTemperatureError { kind: TemperatureErrorKind::ScaleUnknown }),
                _ => Err(ParseTemperatureError { kind: TemperatureErrorKind::NotNumeric })
            };
        }


        let scales_index = temp.len() - 2;
        let temp = temp.to_uppercase();
        let (value, scales) = temp.split_at(scales_index);

        let scales = match scales {
            "CF" => (Scale::Celsius, Scale::Fahrenheit),
            "CK" => (Scale::Celsius, Scale::Kelvin),
            "FC" => (Scale::Fahrenheit, Scale::Celsius),
            "FK" => (Scale::Fahrenheit, Scale::Kelvin),
            "KC" => (Scale::Kelvin, Scale::Celsius),
            "KF" => (Scale::Kelvin, Scale::Fahrenheit),
            _ => return Err(ParseTemperatureError { kind: TemperatureErrorKind::ScaleUnknown })
        };

        let value = match f64::from_str(value) {
            Ok(value_parsed) => value_parsed,
            _ => return Err(ParseTemperatureError { kind: TemperatureErrorKind::NotNumeric })
        };

        Ok(Temperature { value, scale: scales.0, convert_to: scales.1 })
    }
}

fn main() {
    let app_args = std::env::args().skip(1);

    if app_args.len() <= 0 {
        eprintln!("Usage exemple: {} 32FC 45FK 36CK 32CF", get_exec_name());
        std::process::exit(1);
    }

    let mut temperature_list: Vec<Temperature> = Vec::new();
    for elem in app_args {
        match Temperature::from_str(&elem) {
            Ok(temp) => temperature_list.push(temp),
            Err(err) => println!("ParseError: {}, {}", elem, err)
        }
    }

    for temp in temperature_list {
        println!("{}", temp)
    }
}

fn get_exec_name() -> String {
    // let exec_cow = std::env::current_exe()
    //     .ok()
    //     .expect("error ...")
    //     .file_name()
    //     .expect("error ...")
    //     .to_string_lossy()
    //     .into_owned();

    std::env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}
