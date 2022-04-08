pub enum Ethereum {
    // EIP712,
    PlainText,
}

pub enum Tezos {
    PlainText,
}

pub enum SignerType {
    Ethereum(Ethereum),
    Tezos(Tezos),
}

impl SignerType {
    pub fn name(&self) -> String {
        match self {
            SignerType::Ethereum(_) => "Ethereum Address".to_string(),
            SignerType::Tezos(_) => "Tezos Address".to_string(),
        }
    }

    // TODO: Replace with this error
    pub fn valid_id(&self, _id: &str) -> Result<(), String> {
        match self {
            SignerType::Ethereum(_) => {
                // TODO: Something with id.
                Ok(())
            }
            SignerType::Tezos(_) => {
                // TODO: Something with id.
                Ok(())
            }
        }
    }

    pub fn as_did(&self, id: &str) -> Result<String, String> {
        self.valid_id(id)?;
        match self {
            // TODO: Support EIP712
            SignerType::Ethereum(Ethereum::PlainText) => {
                Err("Plain text signing of VCs in Ethereum is not implemented".to_string())
            }
            SignerType::Tezos(Tezos::PlainText) => {
                Ok(format!("did:pkh:tz:{}", id))
            }
        }
    }
}

pub trait SignerMethods {
    // TODO: Add async-trait and make these async.
    // sign takes plain text and returns the corresponding signature
    fn sign(&self, plain_text: &str) -> String;
    // id returns the identifier for the given signer, such as a public key hash.
    fn id(&self) -> String;
}

pub struct Signer<T: SignerMethods> {
    pub id: String,
    pub name: String,
    pub signer_type: SignerType,
    opts: T,
}

impl<T> Signer<T>
where
    T: SignerMethods,
{
    // TODO: Replace with this error
    pub fn new(opts: T, signer_type: SignerType) -> Result<Self, String> {
        let id = opts.id();
        signer_type.valid_id(&id)?;
        Ok(Signer {
            id,
            name: signer_type.name(),
            signer_type,
            opts,
        })
    }

    pub fn sign(&self, text: &str) -> String {
        self.opts.sign(text)
    }

    pub fn as_did(&self) -> Result<String, String> {
        self.signer_type.as_did(&self.id)
    }
}
