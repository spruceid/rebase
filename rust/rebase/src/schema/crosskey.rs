use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{Signer, SignerType};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

pub struct Crosskey {
    pub signature: String,
    // The statement signer as a DID
    pub statement: String,
    pub statement_id: String,
    pub vc_id: String,
}

impl Crosskey {
    pub async fn new<T: SignerType, U: SignerType>(
        signature: String,
        statement: String,
        statement_generator: &impl Fn(&dyn Signer<T>, &dyn Signer<U>) -> String,
        statement_signer: &dyn Signer<T>,
        vc_signer: &dyn Signer<U>,
    ) -> Result<Self, SchemaError> {
        let s = statement_generator(statement_signer, vc_signer);
        if statement != s {
            return Err(SchemaError::MismatchedStatement(format!(
                "credential statement: '{}' generated from arguments: '{}'",
                statement, s
            )));
        }

        statement_signer
            .valid_signature(&statement, &signature)
            .await?;
        Ok(Crosskey {
            signature,
            statement,
            statement_id: statement_signer.id(),
            vc_id: vc_signer.id(),
        })
    }
}

impl SchemaType for Crosskey {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "CrosskeyControl": "https://example.com/CrosskeyControl",
                "controller": "https://example.com/controller",
                "sameAs": "https://example.com/sameAs",
                "statement": "https://example.com/statement",
                "signature": "https://example.com/signature",
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        Ok(None)
    }

    fn subject(&self) -> Result<serde_json::Value, SchemaError> {
        Ok(json!({
            "controller": self.vc_id,
            "sameAs": self.statement_id,
            "statement": self.statement,
            "signature": self.signature,
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(serde_json::from_value(json!([
            "VerifiableCredential",
            "CrosskeyControl",
        ]))?)
    }
}
