use iso_currency::Currency;
use isocountry::CountryCode;
use log::debug;
use uuid::Uuid;

use crate::Account;
use crate::account::{Classification, KnownBankIdCode};
use crate::account::Classification::{BUSINESS, PERSONAL};

#[derive(Debug)]
pub struct Builder<'a> {
    account: Account<'a>,
}

impl<'a> Builder<'a> {
    pub fn new_account() -> Builder<'a> {
        Builder {
            account: Account {
                id: Uuid::new_v4(),
                organisation_id: Uuid::new_v4(),
                country: CountryCode::AFG,
                currency: Currency::AED,
                bank_id_code: "",
                bank_id: "",
                bic: "",
                number: "",
                iban: "",
                title: "",
                classification: Classification::PERSONAL,
            }
        }
    }

    pub fn with_account_id(&mut self, id: Uuid) -> &mut Builder<'a> {
        self.account.id = id;
        self
    }

    pub fn with_organisation_id(&mut self, id: Uuid) -> &mut Builder<'a> {
        self.account.id = id;
        self
    }

    pub fn with_country(&mut self, country: CountryCode) -> &mut Builder<'a> {
        self.account.country = country;
        self
    }

    pub fn with_bank_id(&mut self, id: &'a str) -> &mut Builder<'a> {
        self.account.bank_id = id;
        self
    }

    pub fn with_bank_id_code(&mut self, code: &'a str) -> &mut Builder<'a> {
        self.account.bank_id_code = code;
        self
    }

    pub fn with_number(&mut self, number: &'a str) -> &mut Builder<'a> {
        self.account.number = number;
        self
    }

    pub fn with_iban(&mut self, iban: &'a str) -> &mut Builder<'a> {
        self.account.iban = iban;
        self
    }

    pub fn with_bic(&mut self, bic: &'a str) -> &mut Builder<'a> {
        self.account.bic = bic;
        self
    }

    pub fn mark_personal(&mut self) -> &mut Builder<'a> {
        self.account.classification = PERSONAL;
        self
    }

    pub fn mark_business(&mut self) -> &mut Builder<'a> {
        self.account.classification = BUSINESS;
        self
    }

    pub fn build(&mut self) -> Result<Account, String> {
        if self.account.country == CountryCode::GBR {
            self.account.currency = Currency::GBP;
            return validate_gb_country_account_restrictions(&mut self.account);
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
            return Err(format!("Unsupported country {}", self.account.country))
        }
    }
}

fn validate_gb_country_account_restrictions<'a>(account: &'a mut Account<'a>) -> Result<&'a mut Account<'a>, String> {
    if account.bic.is_empty() {
        return Err(format!("{} requires Bic", CountryCode::GBR))
    }
    return validate_account_restriction_by(account,
                                           CountryCode::GBR,
                                           8,
                                           6,
                                           KnownBankIdCode::GbBankIdCode);
}

fn validate_account_restriction_by<'a>(account: &'a mut Account<'a>,
                                       country: CountryCode,
                                       expected_number_length: u8,
                                       expected_bank_id_length: u8,
                                       expected_code: KnownBankIdCode) -> Result<&'a mut Account<'a>, String> {
    let number_value_length = account.number.len();
    if number_value_length > 0 {
        if number_value_length != expected_number_length as usize {
            return Err(format!("{} requires {}-character long Account Number",
                               country, expected_number_length));
        }
    }
    if account.bank_id.len() != expected_bank_id_length as usize {
        return Err(format!("{} requires {}-character BankId, got '{}'",
                           country, expected_bank_id_length, account.bank_id));
    }
    if account.bank_id_code != expected_code.value() {
        debug!(target: "account_builder", "{} requires BankIdCode, got invalid '{}'",
               country, account.bank_id_code);
        account.bank_id_code = expected_code.value()
    }
    return Ok(account);
}

#[cfg(test)]
mod tests {
    use iso_currency::Currency::GBP;
    use isocountry::CountryCode::GBR;

    use crate::account::Classification::BUSINESS;

    use super::Builder;

    #[test]
    fn should_build_account_in_gbr() {
        let account = Builder::new_account()
            .with_country(GBR)
            .with_bank_id("400300")
            .with_bic("NWBKGB22")
            .mark_business()
            .build().unwrap();

        // then
        assert_eq!("NWBKGB22", account.bic);
        assert_eq!("400300", account.bank_id);
        assert_eq!(GBR, account.country);
        assert_eq!(GBP, account.currency);
        assert_eq!(BUSINESS, account.classification);
    }
}
