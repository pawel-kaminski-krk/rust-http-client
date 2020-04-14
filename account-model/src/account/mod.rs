use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq)]
pub enum Classification {
    PERSONAL,
    BUSINESS,
}

impl From<Classification> for &str {
    fn from(c: Classification) -> Self {
        match c {
            Classification::PERSONAL => "Personal",
            Classification::BUSINESS => "Business",
        }
    }
}

impl TryFrom<&str> for Classification {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Personal" => Ok(Classification::PERSONAL),
            "Business" => Ok(Classification::BUSINESS),
            _ => Err(format!("{} is not classification variant", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

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
        personal_to_value: (Classification::PERSONAL, "Personal", |c: Classification| -> &str { c.into() }),
        business_to_value: (Classification::BUSINESS, "Business", |c: Classification| -> &str { c.into() }),
    }

    parametrised! {
        personal_from_string: ("Personal", Ok(Classification::PERSONAL), |i: &str| i.try_into()),
        business_from_string: ("Business", Ok(Classification::BUSINESS), |i: &str| i.try_into()),
        unknown_string: ("unknown", Err::<Classification, String>("unknown is not classification variant".to_string()), |i: &str| i.try_into()),
    }
}