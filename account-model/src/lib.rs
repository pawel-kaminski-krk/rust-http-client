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
pub struct Account<'a> {
    id: &'a Uuid,
    organisation_id: &'a Uuid,
    country: CountryCode,
    currency: Currency,
    bank_id_code: &'a str,
    bank_id: &'a str,
    bic: &'a str,
    number: &'a str,
    iban: &'a str,
    title: &'a str,
}

impl Eq for Account<'_> {}

#[derive(Debug, Eq, PartialEq)]
pub struct CopAccount<'a> {
    account: &'a Account<'a>,
    first_name: &'a str,
    bank_account_names: &'a Vec<String>,
    bank_account_classification: Classification,
    is_join_bank_account: bool,
    is_matching_opt_out_account: bool,
    secondary_identification: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SepaAccount<'a> {
    account: &'a Account<'a>,
    identification: &'a PrivateIdentification<'a>,
}