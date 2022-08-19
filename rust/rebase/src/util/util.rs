use crate::{
    signer::signer::{Solana, DID, EIP155, PKH},
    witness::witness::{Proof, WitnessError},
};

pub const TEST_2KEY_ETH_SIG_1: &str = "0x56e48e0dbca9eebd31b23a69d56be84e8fa359d27e70e62c3999fbe2f43659845cee0d976ff83ed576e556cd8fbc377eeb4a0cb38f6949f9ac8ff6f8794b869f1b";
pub const TEST_2KEY_ETH_SIG_2: &str = "0x4f5448421f13e597f20ccfbe31ba62ab16bacc6ec93654a1131f126005ffd4cc7688c9c74b492e91cb5c795f53351ee87a05dbe32b9e11dde9d6cf3771506a101c";

pub const TEST_2KEY_ED25519_SIG_1: &str = "2660633d366856fa1da5032ddfab4e3b27e6958c99ce59b7d92dd5fbbd6fac46c916193de00cfb138ff0ba08f86669becab87d828f4673575e865ff98b165102";
pub const TEST_2KEY_ED25519_SIG_2: &str = "dcfd87b8abeadfc896056ea51ee99fe728c9b518a305c37bee7fcf3948357ef34f1b2566a2fb31c37959873a0bf076bf30dd3c45457a67b9118047580febb908";

pub const TEST_2KEY_SOLANA_SIG_1: &str = "a9da65e22dd752af74b92729fa0de6ee3f9126c8c442fc4109ae614b07fa173d6edc4099666cde2c4f9413e07f5ddd3454fcfbce3ad97f4f784a7069e8137903";
pub const TEST_2KEY_SOLANA_SIG_2: &str = "98406d0e97b8b9c7f9e4abf6069d1a11d20a6e4d1b71608862df915038ea6a60276a99aca5c306346936e101e7aef22205dc82486788578060946eac44ae6008";

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

pub fn test_solana_did() -> DID {
    DID::PKH(PKH::Solana(Some(Solana {
        address: "4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_owned(),
    })))
}

pub fn test_solana_did_2() -> DID {
    DID::PKH(PKH::Solana(Some(Solana {
        address: "5gkphffGKYKST3nfAMn7N6zKMpgH38UDRtF7tRN3tEsy".to_owned(),
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
    Solana
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
        (TestWitness::SoundCloud, TestKey::Eth) => Ok("0xe40c9867351698ac82a047587e527e833e7cec4cdd2e1bfe9c27cecf0b63e97f0edc76f972fa9e056f3ee12d2b2821ddc9a028cf5911e21fe1fb3b20a06eb3c21b".to_string()),
        (TestWitness::Twitter, TestKey::Eth) => Ok("0xaa9158d2046ca298d2afdd0c14eaf6cacd61c5560822be802ef5baad94a21d44681e9b33879a027472b621e6b6db8e2a2082ebb5ab5d3d04755b836b05fc1ca61b".to_string()),

        (TestWitness::DNS, TestKey::Solana) => Ok("a67cec3018ee111f0e86e5090f9c868c22fe51df1b8d72558e44707f9a7cd955677c8d024a15748ba285cf45b0965a970e94bcfb9e7fed1312a7525671b89e0f".to_string()),
        (TestWitness::GitHub, TestKey::Solana) => Ok("17dfd40991256c4c9e098d1321a2e13eb0cb62b7dfcd8451ca231cfa06397b2958d5252cb3627248bbfd5190d3d3a800ee77bb77afeb5ba3c232c12300ba9e0c".to_string()),
        (TestWitness::Reddit, TestKey::Solana) => Ok("e1d3992d7d3bbf10874c8ff84a6e54a4ddf16cd329f5bcd1cd63531b23833a215237e323967c1cef28b62b0a9e377c033005136d7999d9826b9b26cd030ccc0b".to_string()),
        (TestWitness::SoundCloud, TestKey::Solana) => Ok("d41004f74379970a95ada02fb7c04dc51893449aae7f92e01e38bd050da7cfa1b6c6fa6371c44be9a404e22abdab54697d5b8052ed532cf1684510f1e7556100".to_string()),
        (TestWitness::Twitter, TestKey::Solana) => Ok("7b67d87b1acb001a2b752f7af9a83b9073a16792384e19d81ea594642c4a98b8d0e948ffd7b569ee5d42f224e2d997f5381b3a83f0976c0f0d2d42713fb7230d".to_string()),
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
        (TestWitness::SoundCloud, TestKey::Eth) => Ok("I am attesting that this SoundCloud profile https://soundcloud.com/foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::Twitter, TestKey::Eth) => Ok("I am attesting that this twitter handle @foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),

        (TestWitness::DNS, TestKey::Solana) => Ok("example.com is linked to 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
        (TestWitness::GitHub, TestKey::Solana) => Ok("I am attesting that this GitHub handle foo is linked to the Solana Address 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
        (TestWitness::Reddit, TestKey::Solana) => Ok("I am attesting that this Reddit handle foo is linked to the Solana Address 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
        (TestWitness::SoundCloud, TestKey::Solana) => Ok("I am attesting that this SoundCloud profile https://soundcloud.com/foo is linked to the Solana Address 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
        (TestWitness::Twitter, TestKey::Solana) => Ok("I am attesting that this twitter handle @foo is linked to the Solana Address 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
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
