use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Classification {
    PERSONAL,
    BUSINESS,
}

impl Classification {
    pub fn value(&self) -> &'static str {
        match *self {
            Classification::PERSONAL => "Personal",
            Classification::BUSINESS => "Business",
        }
    }
}

impl FromStr for Classification {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Personal" => Ok(Classification::PERSONAL),
            "Business" => Ok(Classification::BUSINESS),
            _ => Err(format!("{} is not classification variant", input))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::account::Classification::PERSONAL;

    use super::Classification;

    macro_rules! parametrised {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected, runnable) = $value;
                assert_eq!(expected, runnable(input));
            }
        )*
        }
    }

    parametrised! {
        personal_to_value: (Classification::PERSONAL, "Personal", |c: Classification| c.value()),
        business_to_value: (Classification::BUSINESS, "Business", |c: Classification| c.value()),
    }

    parametrised! {
        personal_from_string: ("Personal", Ok(Classification::PERSONAL), |i: &str| i.parse()),
        business_from_string: ("Business", Ok(Classification::BUSINESS), |i: &str| i.parse()),
        unknown_string: ("unknown", Err::<Classification, String>("unknown is not classification variant".to_string()), |i: &str| i.parse()),
    }
}