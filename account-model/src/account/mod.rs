use std::convert::TryFrom;
use std::str::FromStr;

pub mod account_builder;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

impl TryFrom<&str> for Classification {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "Personal" => Ok(Classification::PERSONAL),
            "Business" => Ok(Classification::BUSINESS),
            _ => Err(format!("{} is not classification variant", input))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum KnownBankIdCode {
    GbBankIdCode,
    AuBankIdCode,
    BeBankIdCode,
    CaBankIdCode,
    FrBankIdCode,
    DeBankIdCode,
    GrBankIdCode,
    HkBankIdCode,
    ItBankIdCode,
    LuBankIdCode,
    PlBankIdCode,
    PtBankIdCode,
    EsBankIdCode,
    ChBankIdCode,
    UsBankIdCode,
}

impl From<KnownBankIdCode> for &str {
    fn from(input: KnownBankIdCode) -> Self {
        match input {
            KnownBankIdCode::GbBankIdCode => "GBDSC",
            KnownBankIdCode::AuBankIdCode => "AUBSB",
            KnownBankIdCode::BeBankIdCode => "BE",
            KnownBankIdCode::CaBankIdCode => "CACPA",
            KnownBankIdCode::FrBankIdCode => "FR",
            KnownBankIdCode::DeBankIdCode => "DEBLZ",
            KnownBankIdCode::GrBankIdCode => "GRBIC",
            KnownBankIdCode::HkBankIdCode => "HKNCC",
            KnownBankIdCode::ItBankIdCode => "ITNCC",
            KnownBankIdCode::LuBankIdCode => "LULUX",
            KnownBankIdCode::PlBankIdCode => "PLKNR",
            KnownBankIdCode::PtBankIdCode => "PTNCC",
            KnownBankIdCode::EsBankIdCode => "ESNCC",
            KnownBankIdCode::ChBankIdCode => "CHBCC",
            KnownBankIdCode::UsBankIdCode => "USABA",
        }
    }
}

impl FromStr for KnownBankIdCode {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "GBDSC" => Ok(KnownBankIdCode::GbBankIdCode),
            "AUBSB" => Ok(KnownBankIdCode::AuBankIdCode),
            "BE" => Ok(KnownBankIdCode::BeBankIdCode),
            "CACPA" => Ok(KnownBankIdCode::CaBankIdCode),
            "FR" => Ok(KnownBankIdCode::FrBankIdCode),
            "DEBLZ" => Ok(KnownBankIdCode::DeBankIdCode),
            "GRBIC" => Ok(KnownBankIdCode::GrBankIdCode),
            "HKNCC" => Ok(KnownBankIdCode::HkBankIdCode),
            "ITNCC" => Ok(KnownBankIdCode::ItBankIdCode),
            "LULUX" => Ok(KnownBankIdCode::LuBankIdCode),
            "PLKNR" => Ok(KnownBankIdCode::PlBankIdCode),
            "PTNCC" => Ok(KnownBankIdCode::PtBankIdCode),
            "ESNCC" => Ok(KnownBankIdCode::EsBankIdCode),
            "CHBCC" => Ok(KnownBankIdCode::ChBankIdCode),
            "USABA" => Ok(KnownBankIdCode::UsBankIdCode),
            _ => Err(format!("{} is not known bank id variant", input))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::Classification;
    use super::KnownBankIdCode;

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
        personal_from_string: ("Personal", Ok(Classification::PERSONAL), |i: &str| i.parse()),
        business_from_string: ("Business", Ok(Classification::BUSINESS), |i: &str| i.parse()),
        unknown_classification_string: ("unknown", Err::<Classification, String>("unknown is not classification variant".to_string()), |i: &str| i.parse()),
        personal_try_from_string: ("Personal", Ok(Classification::PERSONAL), |i: &str| i.try_into()),
        business_try_from_string: ("Business", Ok(Classification::BUSINESS), |i: &str| i.try_into()),
        unknown_classification_try_from_string: ("unknown", Err::<Classification, String>("unknown is not classification variant".to_string()), |i: &str| i.try_into()),
    }

    parametrised! {
        GB_bank_id_code_to_value: (KnownBankIdCode::GbBankIdCode, "GBDSC", |c: KnownBankIdCode| -> &str { c.into() }),
        AU_bank_id_code_to_value: (KnownBankIdCode::AuBankIdCode, "AUBSB", |c: KnownBankIdCode| -> &str { c.into() }),
        BE_bank_id_code_to_value: (KnownBankIdCode::BeBankIdCode, "BE",    |c: KnownBankIdCode| -> &str { c.into() }),
        CA_bank_id_code_to_value: (KnownBankIdCode::CaBankIdCode, "CACPA", |c: KnownBankIdCode| -> &str { c.into() }),
        FR_bank_id_code_to_value: (KnownBankIdCode::FrBankIdCode, "FR",    |c: KnownBankIdCode| -> &str { c.into() }),
        DE_bank_id_code_to_value: (KnownBankIdCode::DeBankIdCode, "DEBLZ", |c: KnownBankIdCode| -> &str { c.into() }),
        GR_bank_id_code_to_value: (KnownBankIdCode::GrBankIdCode, "GRBIC", |c: KnownBankIdCode| -> &str { c.into() }),
        HK_bank_id_code_to_value: (KnownBankIdCode::HkBankIdCode, "HKNCC", |c: KnownBankIdCode| -> &str { c.into() }),
        IT_bank_id_code_to_value: (KnownBankIdCode::ItBankIdCode, "ITNCC", |c: KnownBankIdCode| -> &str { c.into() }),
        LU_bank_id_code_to_value: (KnownBankIdCode::LuBankIdCode, "LULUX", |c: KnownBankIdCode| -> &str { c.into() }),
        PL_bank_id_code_to_value: (KnownBankIdCode::PlBankIdCode, "PLKNR", |c: KnownBankIdCode| -> &str { c.into() }),
        PT_bank_id_code_to_value: (KnownBankIdCode::PtBankIdCode, "PTNCC", |c: KnownBankIdCode| -> &str { c.into() }),
        ES_bank_id_code_to_value: (KnownBankIdCode::EsBankIdCode, "ESNCC", |c: KnownBankIdCode| -> &str { c.into() }),
        CH_bank_id_code_to_value: (KnownBankIdCode::ChBankIdCode, "CHBCC", |c: KnownBankIdCode| -> &str { c.into() }),
        US_bank_id_code_to_value: (KnownBankIdCode::UsBankIdCode, "USABA", |c: KnownBankIdCode| -> &str { c.into() }),
    }

    parametrised! {
        GB_bank_id_code_from_string: ("GBDSC", Ok(KnownBankIdCode::GbBankIdCode), |c: &str| c.parse()),
        AU_bank_id_code_from_string: ("AUBSB", Ok(KnownBankIdCode::AuBankIdCode), |c: &str| c.parse()),
        BE_bank_id_code_from_string: ("BE", Ok(KnownBankIdCode::BeBankIdCode), |c: &str| c.parse()),
        CA_bank_id_code_from_string: ("CACPA", Ok(KnownBankIdCode::CaBankIdCode), |c: &str| c.parse()),
        FR_bank_id_code_from_string: ("FR", Ok(KnownBankIdCode::FrBankIdCode), |c: &str| c.parse()),
        DE_bank_id_code_from_string: ("DEBLZ", Ok(KnownBankIdCode::DeBankIdCode), |c: &str| c.parse()),
        GR_bank_id_code_from_string: ("GRBIC", Ok(KnownBankIdCode::GrBankIdCode), |c: &str| c.parse()),
        HK_bank_id_code_from_string: ("HKNCC", Ok(KnownBankIdCode::HkBankIdCode), |c: &str| c.parse()),
        IT_bank_id_code_from_string: ("ITNCC", Ok(KnownBankIdCode::ItBankIdCode), |c: &str| c.parse()),
        LU_bank_id_code_from_string: ("LULUX", Ok(KnownBankIdCode::LuBankIdCode), |c: &str| c.parse()),
        PL_bank_id_code_from_string: ("PLKNR", Ok(KnownBankIdCode::PlBankIdCode), |c: &str| c.parse()),
        PT_bank_id_code_from_string: ("PTNCC", Ok(KnownBankIdCode::PtBankIdCode), |c: &str| c.parse()),
        ES_bank_id_code_from_string: ("ESNCC", Ok(KnownBankIdCode::EsBankIdCode), |c: &str| c.parse()),
        CH_bank_id_code_from_string: ("CHBCC", Ok(KnownBankIdCode::ChBankIdCode), |c: &str| c.parse()),
        US_bank_id_code_from_string: ("USABA", Ok(KnownBankIdCode::UsBankIdCode), |c: &str| c.parse()),
        unknown_bank_id_string: ("unknown", Err::<KnownBankIdCode, String>("unknown is not known bank id variant".to_string()), |i: &str| i.parse()),
    }
}