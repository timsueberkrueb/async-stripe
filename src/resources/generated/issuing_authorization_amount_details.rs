// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingAuthorizationAmountDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorizationAmountDetails {

    /// The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Option<i64>,
}
