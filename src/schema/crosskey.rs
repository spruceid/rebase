use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{Signer, SignerType};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

pub struct Crosskey {
    pub delimitor: String,
    pub signature: String,
    pub statement: String,
    pub statement_id: String,
    pub vc_id: String,
}

impl Crosskey {
    pub async fn new<T: SignerType, U: SignerType>(
        delimitor: String,
        signature: String,
        statement: String,
        statement_signer: &dyn Signer<T>,
        vc_signer: &dyn Signer<U>,
    ) -> Result<Self, SchemaError> {
        statement_signer.valid_signature(&statement, &signature).await?;
        Ok(Crosskey {
            delimitor,
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
            // NOTE: THIS URL CONTAINES INFO FOR SAME AS:
            // TODO: Change to better URL
            "https://tzprofiles.com/2021/ethereum-address-control-v1.jsonld",
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        // TODO: Verify this is correct:
        Ok(None)
    }

    fn subject(&self, _signer_did: &str) -> Result<serde_json::Value, SchemaError> {
        Ok(json!({
            "address": self.vc_id,
            "sameAs": self.statement_id
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(serde_json::from_value(json!([ 
            "VerifiableCredential",
            "CrossKeyControl",
         ]))?)
    }
}
