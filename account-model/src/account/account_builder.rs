use iso_currency::Currency;
use isocountry::CountryCode;
use log::debug;
use uuid::Uuid;

use crate::Account;
use crate::account::{Classification, KnownBankIdCode};
use crate::account::Classification::{BUSINESS, PERSONAL};

#[derive(Debug)]
pub struct Builder {
    id: Option<Uuid>,
    organisation_id: Option<Uuid>,
    country: Option<CountryCode>,
    currency: Option<Currency>,
    bank_id_code: Option<String>,
    bank_id: Option<String>,
    bic: Option<String>,
    number: Option<String>,
    iban: Option<String>,
    title: Option<String>,
    classification: Classification,
}

impl Builder {
    pub fn new_account() -> Builder {
        Builder {
            id: None,
            organisation_id: None,
            country: None,
            currency: None,
            bank_id_code: None,
            bank_id: None,
            bic: None,
            number: None,
            iban: None,
            title: None,
            classification: Classification::PERSONAL,
        }
    }

    pub fn with_account_id(&mut self, id: Uuid) -> &mut Builder {
        self.id = Some(id);
        self
    }

    pub fn with_organisation_id(&mut self, id: Uuid) -> &mut Builder {
        self.organisation_id = Some(id);
        self
    }

    pub fn with_country(&mut self, country: CountryCode) -> &mut Builder {
        self.country = Some(country);
        self
    }

    pub fn with_bank_id(&mut self, id: &str) -> &mut Builder {
        self.bank_id = Some(id.to_string());
        self
    }

    pub fn with_bank_id_code(&mut self, code: &str) -> &mut Builder {
        self.bank_id_code = Some(code.to_string());
        self
    }

    pub fn with_number(&mut self, number: &str) -> &mut Builder {
        self.number = Some(number.to_string());
        self
    }

    pub fn with_iban(&mut self, iban: &str) -> &mut Builder {
        self.iban = Some(iban.to_string());
        self
    }

    pub fn with_bic(&mut self, bic: &str) -> &mut Builder {
        self.bic = Some(bic.to_string());
        self
    }

    pub fn mark_personal(&mut self) -> &mut Builder {
        self.classification = PERSONAL;
        self
    }

    pub fn mark_business(&mut self) -> &mut Builder {
        self.classification = BUSINESS;
        self
    }

    pub fn build(&mut self) -> Result<Account, String> {
        if self.country.is_none() {
            return Err("Country is required".to_string());
        }

        if self.country.unwrap() == CountryCode::GBR {
            self.create_account_with_gb_country_restrictions()

            // } else if self.account.country == CountryCode::AUS {
            //     self.account.currency = Currency::AUD;
            //     return validateAuAccountRestrictions(&self.account);
            // } else if self.account.country == CountryCode::BEL {
            //     self.account.currency = Currency::EUR;
            //     return validateBeAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::CAN {
            //     self.account.currency = Currency::EUR;
            //     return validateCaAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::FRA {
            //     self.account.currency = Currency::EUR;
            //     return validateFrAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::DEU {
            //     self.account.currency = Currency::EUR;
            //     return validateDeAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::GRC {
            //     self.account.currency = Currency::EUR;
            //     return validateGrAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::HKG {
            //     self.account.currency = Currency::HKD;
            //     return validateHkAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::ITA {
            //     self.account.currency = Currency::EUR;
            //     return validateItAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::LUX {
            //     self.account.currency = Currency::LUF;
            //     return validateLuAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::NLD {
            //     self.account.currency = Currency::EUR;
            //     return validateNlAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::POL {
            //     self.account.currency = Currency::PLN;
            //     return validatePlAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::PRT {
            //     self.account.currency = Currency::EUR;
            //     return validatePtAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::ESP {
            //     self.account.currency = Currency::EUR;
            //     return validateEsAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::CHE {
            //     self.account.currency = Currency::CHF;
            //     return validateChAccountRestriction(&self.account);
            // } else if self.account.country == CountryCode::USA {
            //     self.account.currency = Currency::USD;
            //     return validateUsAccountRestriction(&self.account);
        } else {
            Err(format!("Unsupported country {}", self.country.unwrap()))
        }
    }

    fn create_account_with_gb_country_restrictions(&mut self) -> Result<Account, String> {
        if self.bic.is_none() {
            return Err(format!("{} requires Bic", CountryCode::GBR.alpha3()));
        }
        self.validate_account_restriction_by(CountryCode::GBR,
                                             8,
                                             6,
                                             KnownBankIdCode::GbBankIdCode)?;
        Ok(Account {
            id: get_or_create_id(self.id),
            organisation_id: get_or_create_id(self.organisation_id),
            country: CountryCode::GBR,
            currency: Currency::GBP,
            bank_id: self.bank_id.as_ref().expect("bank id is required").to_string(),
            bank_id_code: self.bank_id_code.as_ref().expect("bank id code is required").to_string(),
            bic: self.bic.as_ref().unwrap().to_string(),
            number: self.number.clone(),
            iban: self.iban.clone(),
            title: self.title.clone(),
            classification: self.classification,
        })
    }

    fn validate_account_restriction_by(&mut self,
                                       country: CountryCode,
                                       expected_number_length: u8,
                                       expected_bank_id_length: u8,
                                       expected_code: KnownBankIdCode) -> Result<(), String> {
        if self.number.is_some() {
            if self.number.as_ref().unwrap().len() != expected_number_length as usize {
                return Err(format!("{} requires {}-character long Account Number",
                                   country.alpha3(), expected_number_length));
            }
        }
        if self.bank_id.is_none() || self.bank_id.as_ref().unwrap().len() != expected_bank_id_length as usize {
            return Err(format!("{} requires {}-character BankId, got '{:?}'",
                               country.alpha3(), expected_bank_id_length, self.bank_id));
        }

        let bank_id_code: &str = expected_code.into();
        let bank_id_code_s: &String = &bank_id_code.to_string();
        if self.bank_id_code.is_none() || self.bank_id_code.as_ref().unwrap().ne(bank_id_code_s) {
            debug!(target: "account_builder", "{} requires BankIdCode, got invalid '{:?}'",
                   country.alpha3(), self.bank_id_code);
            self.bank_id_code = Some(bank_id_code_s.to_string());
        }
        return Ok(());
    }
}

fn get_or_create_id(given: Option<Uuid>) -> Uuid {
    if given.is_none() {
        Uuid::new_v4()
    } else {
        given.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use iso_currency::Currency::GBP;
    use isocountry::CountryCode::GBR;
    use uuid::Uuid;

    use crate::account::KnownBankIdCode;
    use crate::account::Classification::{BUSINESS, PERSONAL};

    use super::Builder;

    #[test]
    fn should_fail_as_no_country() {
        let actual = Builder::new_account()
            .build()
            .unwrap_err();

        // then
        assert_eq!("Country is required", actual);
    }

    macro_rules! invalid_account_parametrised {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (country, bic, number, bank_id, expected) = $value;
                    let mut builder: Builder = Builder::new_account();
                    let builder: &mut Builder = &mut builder;

                    builder
                        .with_country(country)
                        .mark_business();

                    if bank_id.is_some() {
                        builder.with_bank_id(bank_id.unwrap());
                    }
                    if bic.is_some() {
                        builder.with_bic(bic.unwrap());
                    }
                    if number.is_some() {
                        builder.with_number(number.unwrap());
                    }

                    let actual = builder.build().unwrap_err();
                    assert_eq!(expected, actual);
                }
            )*
        }
    }

    invalid_account_parametrised! {
        GBR_invalid_bic: (GBR, None, None, None, "GBR requires Bic"),
        GBR_no_bank_id: (GBR, Some("NWBKGB22"), None, None, "GBR requires 6-character BankId, got 'None'"),
        GBR_too_short_bank_id: (GBR, Some("NWBKGB22"), None, Some("lt-6c"), "GBR requires 6-character BankId, got 'Some(\"lt-6c\")'"),
        GBR_too_long_bank_id: (GBR, Some("NWBKGB22"), None, Some("gt6char"), "GBR requires 6-character BankId, got 'Some(\"gt6char\")'"),
        GBR_too_short_number: (GBR, Some("NWBKGB22"), Some("lt-8chr"), Some("400300"), "GBR requires 8-character long Account Number"),
        GBR_too_long_number: (GBR, Some("NWBKGB22"), Some("gt-8chars"), Some("400300"), "GBR requires 8-character long Account Number"),
    }

    #[test]
    fn should_build_account_in_gbr() {
        let account = Builder::new_account()
            .with_country(GBR)
            .with_bank_id("400300")
            .with_bic("NWBKGB22")
            .mark_business()
            .build().unwrap();

        // then
        assert!(!account.id.is_nil());
        assert!(!account.organisation_id.is_nil());
        assert_eq!("NWBKGB22", account.bic);
        assert_eq!("400300", account.bank_id);
        assert_eq!(as_string(KnownBankIdCode::GbBankIdCode), account.bank_id_code);
        assert_eq!(GBR, account.country);
        assert_eq!(GBP, account.currency);
        assert_eq!(BUSINESS, account.classification);
    }

    #[test]
    fn should_build_full_account_in_gbr() {
        let id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let account = Builder::new_account()
            .with_account_id(id)
            .with_organisation_id(org_id)
            .with_country(GBR)
            .with_bank_id("400300")
            .with_bic("NWBKGB22")
            .mark_personal()
            .with_number("12345678")
            .with_bank_id_code("invalid")
            .with_iban("iban-code")
            .build().unwrap();

        // then
        assert_eq!(id, account.id);
        assert_eq!(org_id, account.organisation_id);
        assert_eq!("NWBKGB22", account.bic);
        assert_eq!("400300", account.bank_id);
        assert_eq!(as_string(KnownBankIdCode::GbBankIdCode), account.bank_id_code);
        assert_eq!(GBR, account.country);
        assert_eq!(GBP, account.currency);
        assert_eq!(PERSONAL, account.classification);
        assert_eq!("12345678", account.number.unwrap().to_string());
        assert_eq!("iban-code", account.iban.unwrap().to_string());
    }

    fn as_string(code: KnownBankIdCode) -> String {
        let value: &str = code.into();
        value.to_string()
    }
}
