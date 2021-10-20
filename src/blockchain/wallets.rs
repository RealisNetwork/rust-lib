use runtime::AccountId;

use substrate_api_client::sp_runtime::app_crypto::sr25519;
use substrate_api_client::Pair;
use serde::Deserialize;

pub trait RealisWallet {
    fn get_public() -> AccountId;
    fn get_private() -> sr25519::Pair;
}

pub struct BridgeMaster {}

impl RealisWallet for BridgeMaster {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::to_value("5CSxbs1GPGgUZvsHNcFMyFRqu56jykBcBWBXhUBay2SXBsaA").unwrap()).unwrap()
    }

    fn get_private() -> sr25519::Pair {
        Pair::from_string(
            "fault pretty bird biology budget table symptom build option wrist time detail",
            None
        ).unwrap()
    }
}

pub struct WithdrawRealisMaster {}

impl RealisWallet for WithdrawRealisMaster {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::to_value("5GusupCwM4jaX4F7QdE9n8edtSA9b6svddqaSdm6MgnpgLUF").unwrap()).unwrap()
    }

    fn get_private() -> sr25519::Pair {
        Pair::from_string(
            "dry corn nothing bomb jelly romance enemy employ turn can penalty depth",
            None
        ).unwrap()
    }
}

pub struct WithdrawBscMaster1 {}

impl RealisWallet for WithdrawBscMaster1 {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::to_value("5H3rP9jqdhCftMAw6BngEGTH71gJQsnohHyuhBjdJW9zXPk2").unwrap()).unwrap()
    }

    fn get_private() -> sr25519::Pair {
        Pair::from_string(
            "lonely price whip repeat cricket vital change output october boost agent plunge",
            None
        ).unwrap()
    }
}

pub struct WithdrawBscMaster2 {}

impl RealisWallet for WithdrawBscMaster2 {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::to_value("5FprwZ4YNVZo1aNQugvhK6oJofVpjPqi9MfUj1fv2gqMCb3c").unwrap()).unwrap()
    }

    fn get_private() -> sr25519::Pair {
        Pair::from_string(
            "bracket history husband click custom clean industry steak believe various noodle tuna",
            None
        ).unwrap()
    }
}

pub struct AdapterMaster1 {}

impl RealisWallet for AdapterMaster1 {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::to_value("5D54XGhtRwffGsmrsaMyUdy3cZhtECnCGpxJgHto8e9csKEc").unwrap()).unwrap()
    }

    fn get_private() -> sr25519::Pair {
        Pair::from_string(
            "bless cram child coast cannon bind solid write split brisk renew tent",
            None
        ).unwrap()
    }
}

pub struct AdapterMaster2 {}

impl RealisWallet for AdapterMaster2 {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::to_value("5C7odVdth9qyssQi81XHjkF8hWeLhMpnN27U24QgMB2YNJ6T").unwrap()).unwrap()
    }

    fn get_private() -> sr25519::Pair {
        Pair::from_string(
            "cinnamon left mind answer grid clarify chuckle flash then tobacco dance truth",
            None
        ).unwrap()
    }
}

pub struct AdapterMaster3 {}

impl RealisWallet for AdapterMaster3 {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::to_value("5EU1u5MaJLfB1hneKf7oPuZUa1PoSDBqpH6wU2E2yaB3h7Vi").unwrap()).unwrap()
    }

    fn get_private() -> sr25519::Pair {
        Pair::from_string(
            "island school shell demand muscle galaxy gesture pencil napkin wrong stumble dismiss",
            None
        ).unwrap()
    }
}

pub struct AdapterMaster4 {}

impl RealisWallet for AdapterMaster4 {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::to_value("5EKqhiruvSw3etmTccRcVT3dahwhMNutAyQEkhT3NoYeVkBf").unwrap()).unwrap()
    }

    fn get_private() -> sr25519::Pair {
        Pair::from_string(
            "prevent book moment rate ride fix survey subway ostrich suggest bag differ",
            None
        ).unwrap()
    }
}