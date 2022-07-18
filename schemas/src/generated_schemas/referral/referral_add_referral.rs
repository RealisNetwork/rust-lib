// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralReferralAddReferralParams {
    #[serde(rename = "referralId")]
    pub referral_id: String,
    #[serde(rename = "referrerId")]
    pub referrer_id: String,
}
pub type ReferralReferralAddReferralReturns = bool;
