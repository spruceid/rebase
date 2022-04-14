use ssi::vc::{Credential, LinkedDataProofOptions};
use thiserror::Error;

// TODO: Reformat to Trait rather than Enum

#[derive(Error, Debug)]
pub enum SignerError {
    #[error("invalid id for {signer_type:?}, {reason:?}")]
    InvalidId {
        signer_type: String,
        reason: String
    },
    #[error("failed to sign bytes, {0}")]
    Sign(String),

    #[error("failed to sign credential, {0}")]
    SignCredential(String),

    #[error("given message and signature did not correspond to given key")]
    InvalidSignature,

    // TODO: Remove!
    #[error("this feature is unimplemented")]
    Unimplemented
}

pub trait SignerType {
    fn name(&self) -> String;

    fn valid_id(&self, _id: &str) -> Result<(), SignerError>; 

    fn as_did(&self, id: &str) -> Result<String, SignerError>;

    // proof returns the linked data proof options for a given signer type
    fn proof(&self, id: &str) -> Result<Option<LinkedDataProofOptions>, SignerError>;

    // {
    //     self.valid_id(id)?;
    //     match self {
    //         SignerType::Ed25519(signer_type) => match signer_type {
    //              Ed25519::DIDWebJWK => Ok(Some(LinkedDataProofOptions {
    //                 verification_method: Some(URI::String(format!(
    //                     "{}#controller",
    //                     self.as_did(&id)?
    //                 ))),
    //                 ..Default::default()
    //             })),
    //         }
    //         SignerType::Ethereum(_) => {
    //             // TODO: impl.
    //             Err(SignerError::Unimplemented)
    //         }
    //         SignerType::Tezos(signer_type) => match signer_type {
    //             Tezos::PlainText => Ok(Some(LinkedDataProofOptions {
    //                 verification_method: Some(URI::String(format!(
    //                     "{}#TezosMethod2021",
    //                     self.as_did(&id)?
    //                 ))),
    //                 ..Default::default()
    //             })),
    //             // _ => Err("impl".to_string()),
    //         },
    //     }
    // }

    fn valid_signature(&self, statement: &str, signature: &str, id: &str) -> Result<(), SignerError>;
}

pub trait SignerMethods {
    // TODO: Add async-trait and make these async.
    // sign takes plain text and returns the corresponding signature
    fn sign(&self, plain_text: &str) -> Result<String, SignerError>;
    // sign_vc takes a mutable reference to an incomplete VC and signs it.
    fn sign_vc(
        &self,
        vc: &mut Credential,
        proof: Option<LinkedDataProofOptions>,
    ) -> Result<(), SignerError>;
    // id returns the identifier for the given signer, such as a public key hash.
    fn id(&self) -> String;
}

pub struct Signer<T: SignerMethods, U: SignerType> {
    pub id: String,
    pub name: String,
    pub signer_type: U,
    opts: T,
}

impl<T, U> Signer<T, U>
where
    T: SignerMethods,
    U: SignerType
{
    pub fn new(opts: T, signer_type: U) -> Result<Self, SignerError> {
        let id = opts.id();
        signer_type.valid_id(&id)?;
        Ok(Signer {
            id,
            name: signer_type.name(),
            signer_type,
            opts,
        })
    }

    pub fn sign(&self, text: &str) -> Result<String, SignerError> {
        self.opts.sign(text)
    }

    pub fn sign_vc(&self, vc: &mut Credential) -> Result<(), SignerError> {
        self.opts.sign_vc(vc, self.signer_type.proof(&self.id)?)
    }

    pub fn as_did(&self) -> Result<String, SignerError> {
        self.signer_type.as_did(&self.id)
    }

    pub fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError> {
        self.signer_type.valid_signature(statement, signature, &self.id)
    }
}
