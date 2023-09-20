use crate::{
    issuer::ed25519::Ed25519Jwk,
    subject::{ed25519::Ed25519Jwk as JwkSubj, ethereum::Eip155, solana::Solana},
    types::{
        defs::{get_verification_method, DIDKey, DIDMethod, Issuer, Source, Subject, JWK},
        enums::subject::{Key, Pkh, Subjects, Web},
        error::{FlowError, IssuerError, SubjectError},
    },
};
use async_trait::async_trait;
use ssi::{ldp::Proof as LDProof, one_or_many::OneOrMany, vc::Credential};

pub const TEST_2KEY_ETH_SIG_1: &str = "0x56e48e0dbca9eebd31b23a69d56be84e8fa359d27e70e62c3999fbe2f43659845cee0d976ff83ed576e556cd8fbc377eeb4a0cb38f6949f9ac8ff6f8794b869f1b";
pub const TEST_2KEY_ETH_SIG_2: &str = "0x4f5448421f13e597f20ccfbe31ba62ab16bacc6ec93654a1131f126005ffd4cc7688c9c74b492e91cb5c795f53351ee87a05dbe32b9e11dde9d6cf3771506a101c";

pub const TEST_2KEY_ED25519_SIG_1: &str = "2660633d366856fa1da5032ddfab4e3b27e6958c99ce59b7d92dd5fbbd6fac46c916193de00cfb138ff0ba08f86669becab87d828f4673575e865ff98b165102";
pub const TEST_2KEY_ED25519_SIG_2: &str = "dcfd87b8abeadfc896056ea51ee99fe728c9b518a305c37bee7fcf3948357ef34f1b2566a2fb31c37959873a0bf076bf30dd3c45457a67b9118047580febb908";

pub const TEST_2KEY_SOLANA_SIG_1: &str = "a9da65e22dd752af74b92729fa0de6ee3f9126c8c442fc4109ae614b07fa173d6edc4099666cde2c4f9413e07f5ddd3454fcfbce3ad97f4f784a7069e8137903";
pub const TEST_2KEY_SOLANA_SIG_2: &str = "98406d0e97b8b9c7f9e4abf6069d1a11d20a6e4d1b71608862df915038ea6a60276a99aca5c306346936e101e7aef22205dc82486788578060946eac44ae6008";

pub fn test_eth_did() -> Subjects {
    Subjects::Pkh(Pkh::Eip155(Eip155 {
        address: "0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_owned(),
        chain_id: "1".to_owned(),
    }))
}

pub fn test_eth_did_2() -> Subjects {
    Subjects::Pkh(Pkh::Eip155(Eip155 {
        address: "0x2CfdC694c436BBb1a7f33db015d40C6AA418C3ff".to_owned(),
        chain_id: "1".to_owned(),
    }))
}

pub fn test_solana_did() -> Subjects {
    Subjects::Pkh(Pkh::Solana(Solana {
        address: "4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_owned(),
    }))
}

pub fn test_solana_did_2() -> Subjects {
    Subjects::Pkh(Pkh::Solana(Solana {
        address: "5gkphffGKYKST3nfAMn7N6zKMpgH38UDRtF7tRN3tEsy".to_owned(),
    }))
}

pub async fn test_did_keypair() -> Result<(Subjects, Ed25519Jwk), String> {
    let jwk = JWK::generate_ed25519().map_err(|e| e.to_string())?;
    let dk = DIDKey {};

    let d = dk
        .generate(&Source::Key(&jwk))
        .ok_or("DID Generation returned None".to_string())?;

    let vm = get_verification_method(&d, &dk)
        .await
        .ok_or("Failed to generated verification method from DID".to_string())?;

    let split_did: Vec<String> = vm.split('#').map(|s| s.to_string()).collect();
    if split_did.len() != 2 {
        return Err("Delegate DID was not in expected format".to_string());
    }

    let json_jwk = serde_json::to_string(&jwk)
        .map_err(|e| format!("Could not serailize JWK from did resolver: {}", e))?;

    Ok((
        Subjects::Key(Key::Ed25519(
            JwkSubj::new(&split_did[0], &split_did[1]).map_err(|e| e.to_string())?,
        )),
        Ed25519Jwk::new(&split_did[0], &json_jwk, &split_did[1]).map_err(|e| e.to_string())?,
    ))
}

pub fn test_ed25519_did() -> Subjects {
    Subjects::Web(Web::Ed25519(JwkSubj {
        did: "did:web:rebasedemokey.pages.dev".to_string(),
        key_name: "controller".to_string(),
    }))
}

pub fn test_ed25519_did_2() -> Subjects {
    Subjects::Web(Web::Ed25519(JwkSubj {
        did: "did:web:tzprofiles.com".to_string(),
        key_name: "controller".to_string(),
    }))
}

pub enum TestWitness {
    DNS,
    GitHub,
    Twitter,
    Reddit,
    SoundCloud,
    NftOwnership,
    PoapOwnership,
}

pub enum TestKey {
    Eth,
    Ed25519,
    Solana,
}

pub fn test_witness_signature(witness: TestWitness, key: TestKey) -> Result<String, FlowError> {
    match (witness, key) {
        (TestWitness::DNS, TestKey::Ed25519) => Ok("95cff42c8a40f2ee8e0a2c37afdb8e60b7ffaf52a8378ad9288797200181a879f89872ee5b5921fb39bfea50128badd79ef041f1ba40489709984af723a1550e".to_string()),
        (TestWitness::GitHub, TestKey::Ed25519) => Ok("99d17a62134bcc7ae6e45d984c2d1472fcaa6c5fb9260d49a9107606a6a1059f4c0bc8d28b5162a1e4dad9dd9f0771968e66667c05bb5fad262d211d3a81f90a".to_string()),
        (TestWitness::Twitter, TestKey::Ed25519) => Ok("10009637910e72222e3cade3a5d5b17112f91bf93e089007f2b9b8968865bf46f73b2c6864c7adc8b19bf31f75ad4e078754d08718f1dd80941126c69f137801".to_string()),
        (TestWitness::Reddit, TestKey::Ed25519) => Err(FlowError::Validation("Ed25519 + Reddit Test Not Supported".to_string())),
        (TestWitness::SoundCloud, TestKey::Ed25519) => Err(FlowError::Validation("Ed25519 + SoundCloud Test Not Supported".to_string())),
        (TestWitness::NftOwnership, TestKey::Ed25519) => Err(FlowError::Validation("Ed25519 + NFT Test Not Supported".to_string())),
        (TestWitness::PoapOwnership, TestKey::Ed25519) => Err(FlowError::Validation("Ed25519 + POAP Test Not Supported".to_string())),

        (TestWitness::DNS, TestKey::Eth) => Ok("0xabf167138efc4705a25ec7751536d3d66a4898a80aac90a9be01b6432e4a1ba261175b7b917171ed722ae24c7875cbbc0bf0c9ec318772c0d6d4335029aac3141b".to_string()),
        (TestWitness::GitHub, TestKey::Eth) => Ok("0x5cca3dbe5202e6b5389658011fa441a2c7b7424e388c4dc869e6e6d5538bcf786626b75d793c38443cb7bd7bab72d6d3eb1017fa0614d7b4770ba7d1c4f22f0e1c".to_string()),
        (TestWitness::Reddit, TestKey::Eth) => Ok("0x112ecbb1a0e597441ab5941beaaba4a49f3167411f3f4a205d862bf993fb00af2e145bc260fb0170b3b5192be2827417d5f07721ea70168d50a36ff18a95720b1b".to_string()),
        (TestWitness::SoundCloud, TestKey::Eth) => Ok("0xe40c9867351698ac82a047587e527e833e7cec4cdd2e1bfe9c27cecf0b63e97f0edc76f972fa9e056f3ee12d2b2821ddc9a028cf5911e21fe1fb3b20a06eb3c21b".to_string()),
        (TestWitness::Twitter, TestKey::Eth) => Ok("0xaa9158d2046ca298d2afdd0c14eaf6cacd61c5560822be802ef5baad94a21d44681e9b33879a027472b621e6b6db8e2a2082ebb5ab5d3d04755b836b05fc1ca61b".to_string()),
        (TestWitness::NftOwnership, TestKey::Eth) => Ok("0x301a5e55d5e49bebf9704dbaa2f9341393cfd559f6a85e2d3f8e74a2ec5c63087b1015bbc44253d2323c41657c23f196a510ce730e18fcf706c7e425d77b89c91b".to_string()),
        (TestWitness::PoapOwnership, TestKey::Eth) => Ok("0xec2889af4f2c3008bc689312fd004d1832f345363ce508f98859ffe151b68e820e72ba816fdc35709db305cc0dee8f2571a16de6a24beeabee49d007e0cd44811b".to_string()),

        (TestWitness::DNS, TestKey::Solana) => Ok("a67cec3018ee111f0e86e5090f9c868c22fe51df1b8d72558e44707f9a7cd955677c8d024a15748ba285cf45b0965a970e94bcfb9e7fed1312a7525671b89e0f".to_string()),
        (TestWitness::GitHub, TestKey::Solana) => Ok("17dfd40991256c4c9e098d1321a2e13eb0cb62b7dfcd8451ca231cfa06397b2958d5252cb3627248bbfd5190d3d3a800ee77bb77afeb5ba3c232c12300ba9e0c".to_string()),
        (TestWitness::Reddit, TestKey::Solana) => Ok("e1d3992d7d3bbf10874c8ff84a6e54a4ddf16cd329f5bcd1cd63531b23833a215237e323967c1cef28b62b0a9e377c033005136d7999d9826b9b26cd030ccc0b".to_string()),
        (TestWitness::SoundCloud, TestKey::Solana) => Ok("d41004f74379970a95ada02fb7c04dc51893449aae7f92e01e38bd050da7cfa1b6c6fa6371c44be9a404e22abdab54697d5b8052ed532cf1684510f1e7556100".to_string()),
        (TestWitness::Twitter, TestKey::Solana) => Ok("7b67d87b1acb001a2b752f7af9a83b9073a16792384e19d81ea594642c4a98b8d0e948ffd7b569ee5d42f224e2d997f5381b3a83f0976c0f0d2d42713fb7230d".to_string()),
        (TestWitness::NftOwnership, TestKey::Solana) => Err(FlowError::Validation("Solana + NFT Test Not Supported".to_string())),
        (TestWitness::PoapOwnership, TestKey::Solana) => Err(FlowError::Validation("Solana + POAP Test Not Supported".to_string())),
    }
}

pub fn test_witness_statement(witness: TestWitness, key: TestKey) -> Result<String, FlowError> {
    match (witness, key) {
        (TestWitness::DNS, TestKey::Ed25519) => Ok("example.com is linked to rebasedemokey.pages.dev".to_string()),
        (TestWitness::GitHub, TestKey::Ed25519) => Ok("I am attesting that this GitHub handle foo is linked to the Ed25519 Web Key rebasedemokey.pages.dev".to_string()),
        (TestWitness::Twitter, TestKey::Ed25519) => Ok("I am attesting that this twitter handle @foo is linked to the Ed25519 Web Key rebasedemokey.pages.dev".to_string()),
        (TestWitness::Reddit, TestKey::Ed25519) => Err(FlowError::Validation("Ed25519 + Reddit Test Not Supported".to_string())),
        (TestWitness::SoundCloud, TestKey::Ed25519) => Err(FlowError::Validation("Ed25519 + SoundCloud Test Not Supported".to_string())),
        (TestWitness::NftOwnership, TestKey::Ed25519) => Err(FlowError::Validation("Ed25519 + NFT Test Not Supported".to_string())),
        (TestWitness::PoapOwnership, TestKey::Ed25519) => Err(FlowError::Validation("Ed25519 + POAP Test Not Supported".to_string())),

        (TestWitness::DNS, TestKey::Eth) => Ok("example.com is linked to 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::GitHub, TestKey::Eth) => Ok("I am attesting that this GitHub handle foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::Reddit, TestKey::Eth) => Ok("I am attesting that this Reddit handle foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::SoundCloud, TestKey::Eth) => Ok("I am attesting that this SoundCloud profile https://soundcloud.com/foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::Twitter, TestKey::Eth) => Ok("I am attesting that this twitter handle @foo is linked to the Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016".to_string()),
        (TestWitness::NftOwnership, TestKey::Eth) => Ok(r#"The Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016 owns an asset from the contract 0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85 on the network eth-mainnet at time of 2023-09-27T16:23:00.447Z

cd05dc1c800cbde0ed902af53b486771981df54caef7139a09a5f9653c7d925e48671236f480d3187ae65521f2f7e365ab25175102fbec185fa2e3e8e11c800b"#.to_string()),
        (TestWitness::PoapOwnership, TestKey::Eth) => Ok(r#"The Ethereum Address 0xdA3176d77c04632F2862B14E35bc6B4717FB5016 has a POAP for event id 102213 at time of 2023-09-27T16:36:33.696Z

59cabccace1631a0b0f7b7a38304e551b4cd847623f0cf6ad20296a1d63636b9259fbb3015500f4065168138a3ca90c2f992ed0de702f94fefff07f7fecae804"#.to_string()),

        (TestWitness::DNS, TestKey::Solana) => Ok("example.com is linked to 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
        (TestWitness::GitHub, TestKey::Solana) => Ok("I am attesting that this GitHub handle foo is linked to the Solana Address 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
        (TestWitness::Reddit, TestKey::Solana) => Ok("I am attesting that this Reddit handle foo is linked to the Solana Address 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
        (TestWitness::SoundCloud, TestKey::Solana) => Ok("I am attesting that this SoundCloud profile https://soundcloud.com/foo is linked to the Solana Address 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
        (TestWitness::Twitter, TestKey::Solana) => Ok("I am attesting that this twitter handle @foo is linked to the Solana Address 4uTjzi5QCmE1qpB7TBnDk5tyzUBvSBWKBUpWheVBuMBN".to_string()),
        (TestWitness::NftOwnership, TestKey::Solana) => Err(FlowError::Validation("Solana + NFT Test Not Supported".to_string())),
        (TestWitness::PoapOwnership, TestKey::Solana) => Err(FlowError::Validation("Solana + POAP Test Not Supported".to_string())),
    }
}

pub struct MockFlow {
    pub statement: String,
    pub signature: String,
}

pub struct MockIssuer {}

#[async_trait(?Send)]
impl Subject for MockIssuer {
    fn did(&self) -> Result<String, SubjectError> {
        Err(SubjectError::Validation("unimplemented".to_string()))
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        Err(SubjectError::Validation("unimplemented".to_string()))
    }

    fn verification_method(&self) -> Result<String, SubjectError> {
        Err(SubjectError::Validation("unimplemented".to_string()))
    }

    async fn valid_signature(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<(), SubjectError> {
        Err(SubjectError::Validation("unimplemented".to_string()))
    }
}

#[async_trait(?Send)]
impl Issuer for MockIssuer {
    async fn sign(&self, _plain_text: &str) -> Result<String, IssuerError> {
        Err(IssuerError::Internal("unimplemented".to_string()))
    }

    async fn sign_vc(&self, _vc: &mut Credential) -> Result<(), IssuerError> {
        Err(IssuerError::Internal("unimplemented".to_string()))
    }

    async fn generate_jwt(&self, _vc: &Credential) -> Result<String, IssuerError> {
        Err(IssuerError::Internal("unimplemented".to_string()))
    }

    async fn proof(
        &self,
        _credential: &Credential,
    ) -> Result<Option<OneOrMany<LDProof>>, IssuerError> {
        Err(IssuerError::Internal("unimplemented".to_string()))
    }
}
