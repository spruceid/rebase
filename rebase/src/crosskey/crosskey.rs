use crate::schema::{crosskey::Crosskey, schema_type::SchemaError, schema_type::SchemaType};
use crate::signer::signer::{Signer, SignerMethods};
use ssi::vc::Credential;

// TODO: RESTORE ONCE SIGNER TYPE FINALIZED.
/*
pub fn crosskey_claim<T: SignerMethods, U: SignerMethods>(
    first: &Signer<T>,
    second: &Signer<U>,
    statement_generator: impl Fn(&Signer<T>, &Signer<U>) -> String,
) -> Result<(String, String), SchemaError> {
    let statement = statement_generator(first, second);
    let signature = second.sign(&statement)?;
    Ok((statement, signature))
}

pub fn crosskey_credential<T: SignerMethods, U: SignerMethods>(
    first: &Signer<T>,
    second: &Signer<U>,
    statement_generator: impl Fn(&Signer<T>, &Signer<U>) -> String,
    delimitor: &str,
) -> Result<Credential, SchemaError> {
    let (statement, signature) = crosskey_claim(first, second, statement_generator)?;
    let schema = Crosskey::new(statement, delimitor.to_owned(), signature, second)?;
    Ok(schema.credential(first)?)
}

const DEFAULT_DELIMITER: &str = "\n";

pub fn default_statement<T: SignerMethods, U: SignerMethods>(
    first: &Signer<T>,
    second: &Signer<U>,
) -> String {
    format!(
        "{} {} is linked to {} {}",
        first.name, first.id, second.name, second.id
    )
}

pub fn default_crosskey_credential<T: SignerMethods, U: SignerMethods>(
    first: &Signer<T>,
    second: &Signer<U>,
) -> Result<Credential, SchemaError> {
    crosskey_credential(first, second, default_statement, DEFAULT_DELIMITER)
}
*/