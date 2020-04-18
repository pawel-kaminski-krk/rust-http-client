use iso_currency::Currency;
use isocountry::CountryCode;
use uuid::Uuid;

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