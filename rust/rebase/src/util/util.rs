use crate::{
    signer::signer::{DID, EIP155, PKH},
    witness::witness::{Proof, WitnessError},
};

pub const TEST_2KEY_ETH_SIG_1: &str = "0x56e48e0dbca9eebd31b23a69d56be84e8fa359d27e70e62c3999fbe2f43659845cee0d976ff83ed576e556cd8fbc377eeb4a0cb38f6949f9ac8ff6f8794b869f1b";
pub const TEST_2KEY_ETH_SIG_2: &str = "0x4f5448421f13e597f20ccfbe31ba62ab16bacc6ec93654a1131f126005ffd4cc7688c9c74b492e91cb5c795f53351ee87a05dbe32b9e11dde9d6cf3771506a101c";

pub const TEST_2KEY_ED25519_SIG_1: &str = "2660633d366856fa1da5032ddfab4e3b27e6958c99ce59b7d92dd5fbbd6fac46c916193de00cfb138ff0ba08f86669becab87d828f4673575e865ff98b165102";
pub const TEST_2KEY_ED25519_SIG_2: &str = "dcfd87b8abeadfc896056ea51ee99fe728c9b518a305c37bee7fcf3948357ef34f1b2566a2fb31c37959873a0bf076bf30dd3c45457a67b9118047580febb908";

pub struct MockGenerator {
    pub post: String,
}

pub fn test_eth_did() -> DID {
    DID::PKH(PKH::EIP155(Some(EIP155 {
        address: "0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_owned(),
        chain_id: "1".to_owned(),
    })))
}

pub fn test_eth_did_2() -> DID {
    DID::PKH(PKH::EIP155(Some(EIP155 {
        address: "0x2CfdC694c436BBb1a7f33db015d40C6AA418C3ff".to_owned(),
        chain_id: "1".to_owned(),
    })))
}

pub fn test_ed25519_did() -> DID {
    DID::Web(Some("did:web:rebasedemokey.pages.dev".to_string()))
}

pub fn test_ed25519_did_2() -> DID {
    DID::Web(Some("did:web:tzprofiles.com".to_string()))
}

pub enum TestWitness {
    DNS,
    GitHub,
    Twitter,
    Reddit,
    SoundCloud,
}

pub enum TestKey {
    Eth,
    Ed25519,
}

pub enum TestControlKey {
    Eth(TestKey),
    Ed25519(TestKey),
}

pub fn test_witness_signature(witness: TestWitness, key: TestKey) -> Result<String, WitnessError> {
    match (witness, key) {
        (TestWitness::DNS, TestKey::Ed25519) => Ok("95cff42c8a40f2ee8e0a2c37afdb8e60b7ffaf52a8378ad9288797200181a879f89872ee5b5921fb39bfea50128badd79ef041f1ba40489709984af723a1550e".to_string()),
        (TestWitness::GitHub, TestKey::Ed25519) => Ok("99d17a62134bcc7ae6e45d984c2d1472fcaa6c5fb9260d49a9107606a6a1059f4c0bc8d28b5162a1e4dad9dd9f0771968e66667c05bb5fad262d211d3a81f90a".to_string()),
        (TestWitness::Twitter, TestKey::Ed25519) => Ok("10009637910e72222e3cade3a5d5b17112f91bf93e089007f2b9b8968865bf46f73b2c6864c7adc8b19bf31f75ad4e078754d08718f1dd80941126c69f137801".to_string()),
        (TestWitness::Reddit, TestKey::Ed25519) => Err(WitnessError::BadConfig("Ed25519 + Reddit Test Not Supported".to_string())),
        (TestWitness::SoundCloud, TestKey::Ed25519) => Err(WitnessError::BadConfig("Ed25519 + SoundCloud Test Not Supported".to_string())),
        (TestWitness::DNS, TestKey::Eth) => Ok("0xabf167138efc4705a25ec7751536d3d66a4898a80aac90a9be01b6432e4a1ba261175b7b917171ed722ae24c7875cbbc0bf0c9ec318772c0d6d4335029aac3141b".to_string()),
        (TestWitness::GitHub, TestKey::Eth) => Ok("0x5cca3dbe5202e6b5389658011fa441a2c7b7424e388c4dc869e6e6d5538bcf786626b75d793c38443cb7bd7bab72d6d3eb1017fa0614d7b4770ba7d1c4f22f0e1c".to_string()),
        (TestWitness::Reddit, TestKey::Eth) => Ok("0x112ecbb1a0e597441ab5941beaaba4a49f3167411f3f4a205d862bf993fb00af2e145bc260fb0170b3b5192be2827417d5f07721ea70168d50a36ff18a95720b1b".to_string()),
        (TestWitness::SoundCloud, TestKey::Eth) => Ok("0xd71b40982871791953df2053eb1dd41186d7970d03983652a1f4f245a322e38741c775168d69aa6c050fe916227b7032103c2fa09aba6d6b1ab9742fc549163b1c".to_string()),
        (TestWitness::Twitter, TestKey::Eth) => Ok("0xaa9158d2046ca298d2afdd0c14eaf6cacd61c5560822be802ef5baad94a21d44681e9b33879a027472b621e6b6db8e2a2082ebb5ab5d3d04755b836b05fc1ca61b".to_string()),
    }
}

pub fn test_witness_statement(witness: TestWitness, key: TestKey) -> Result<String, WitnessError> {
    match (witness, key) {
        (TestWitness::DNS, TestKey::Ed25519) => Ok("example.com is linked to rebasedemokey.pages.dev".to_string()),
        (TestWitness::GitHub, TestKey::Ed25519) => Ok("I am attesting that this GitHub handle foo is linked to the Ed25519 Web Key rebasedemokey.pages.dev".to_string()),
        (TestWitness::Twitter, TestKey::Ed25519) => Ok("I am attesting that this twitter handle @foo is linked to the Ed25519 Web Key rebasedemokey.pages.dev".to_string()),
        (TestWitness::Reddit, TestKey::Ed25519) => Err(WitnessError::BadConfig("Ed25519 + Reddit Test Not Supported".to_string())),
        (TestWitness::SoundCloud, TestKey::Ed25519) => Err(WitnessError::BadConfig("Ed25519 + SoundCloud Test Not Supported".to_string())),
        (TestWitness::DNS, TestKey::Eth) => Ok("example.com is linked to 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::GitHub, TestKey::Eth) => Ok("I am attesting that this GitHub handle foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::Reddit, TestKey::Eth) => Ok("I am attesting that this Reddit handle foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::SoundCloud, TestKey::Eth) => Ok("I am attesting that this SoundCloud handle foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::Twitter, TestKey::Eth) => Ok("I am attesting that this twitter handle @foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
    }
}

impl MockGenerator {
    pub fn new<T: Proof + Sized>(sig: String, proof: fn() -> T) -> Result<Self, WitnessError> {
        Ok(MockGenerator {
            post: format!(
                "{}{}{}",
                proof().generate_statement()?,
                proof().delimitor(),
                sig
            ),
        })
    }
}
