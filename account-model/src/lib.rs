use iso_currency::Currency;
use isocountry::CountryCode;
use uuid::Uuid;

use crate::account::account_builder::Builder;
use crate::account::Classification;

mod account;

#[derive(Debug, Eq, PartialEq)]
pub struct PrivateIdentification<'a> {
    name: &'a str,
    surname: &'a str,
    birth_date: &'a str,
    birth_country: CountryCode,
    document_number: &'a str,
    address_line: &'a str,
    city: &'a str,
    country: CountryCode,
}

#[derive(Debug, PartialEq)]
pub struct Account {
    id: Uuid,
    organisation_id: Uuid,
    country: CountryCode,
    currency: Currency,
    bank_id_code: String,
    bank_id: String,
    bic: String,
    number: Option<String>,
    iban: Option<String>,
    title: Option<String>,
    classification: Classification,
}

impl Account {
    pub fn cop() -> Builder {
        return Builder::new_account();
    }

    pub fn sepa() -> Builder {
        return Builder::new_account();
    }


    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn organisation_id(&self) -> Uuid {
        self.organisation_id
    }
    pub fn country(&self) -> CountryCode {
        self.country
    }
    pub fn currency(&self) -> Currency {
        self.currency
    }
    pub fn bank_id_code(&self) -> &str {
        &self.bank_id_code
    }
    pub fn bank_id(&self) -> &str {
        &self.bank_id
    }
    pub fn bic(&self) -> &str {
        &self.bic
    }
    pub fn number(&self) -> &Option<String> {
        &self.number
    }
    pub fn iban(&self) -> &Option<String> {
        &self.iban
    }
    pub fn title(&self) -> &Option<String> {
        &self.title
    }
    pub fn classification(&self) -> Classification {
        self.classification
    }
}

impl Eq for Account {}

#[derive(Debug, Eq, PartialEq)]
pub struct CopAccount<'a> {
    account: Account,
    first_name: &'a str,
    bank_account_names: &'a Vec<String>,
    bank_account_classification: Classification,
    is_join_bank_account: bool,
    is_matching_opt_out_account: bool,
    secondary_identification: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SepaAccount<'a> {
    account: Account,
    identification: &'a PrivateIdentification<'a>,
}