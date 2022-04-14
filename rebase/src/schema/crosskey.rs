use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{Signer, SignerMethods, SignerType};
use ssi::{
    one_or_many::OneOrMany,
    vc::{Evidence},
};

pub struct Crosskey {
    pub statement: String,
    pub delimitor: String,
    pub signature: String,
}

impl Crosskey {
    pub fn new<T: SignerMethods, U: SignerType>(
        statement: String,
        delimitor: String,
        signature: String,
        signer: &Signer<T, U>,
        
    ) -> Result<Self, SchemaError> {
        signer.valid_signature(&statement, &signature)?;
        Ok(Crosskey {
            statement,
            delimitor,
            signature,
        })
    }
}

impl SchemaType for Crosskey {
    fn context(&self) -> Result<String, SchemaError> {
        Err(SchemaError::Context("Unimplemented".to_string()))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        Err(SchemaError::Context("Unimplemented".to_string()))
    }

    fn subject(&self, singer_did: &str) -> Result<String, SchemaError> {
        Err(SchemaError::Context("Unimplemented".to_string()))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Err(SchemaError::Context("Unimplemented".to_string()))
    }
}