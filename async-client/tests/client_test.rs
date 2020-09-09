use iso_currency::Currency::GBP;
use isocountry::CountryCode::GBR;
use uuid::Uuid;

use account_model::Account;
use async_http_client::Client;

#[test]
fn sends_create_cop_account_request() {
    let account = Account::cop()
        .with_country(GBR)
        .with_bank_id("400300")
        .with_bic("NWBKGB22")
        .mark_business()
        .build()
        .unwrap();

    assert!(!account.id().is_nil());
}