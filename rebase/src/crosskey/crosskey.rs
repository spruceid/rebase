use crate::schema::{crosskey::Crosskey, schema_type::SchemaError, schema_type::SchemaType};
use crate::signer::signer::{Signer, SignerType};
use ssi::vc::Credential;

// TODO: RESTORE ONCE SIGNER TYPE FINALIZED.
pub fn crosskey_claim<T: SignerType, U: SignerType>(
    first: &dyn Signer<T>,
    second: &dyn Signer<U>,
    statement_generator: impl Fn(&dyn Signer<T>, &dyn Signer<U>) -> String,
) -> Result<(String, String), SchemaError> {
    let statement = statement_generator(first, second);
    let signature = second.sign(&statement)?;
    Ok((statement, signature))
}

pub fn crosskey_credential<T: SignerType, U: SignerType>(
    first: &dyn Signer<T>,
    second: &dyn Signer<U>,
    statement_generator: impl Fn(&dyn Signer<T>, &dyn Signer<U>) -> String,
    delimitor: &str,
) -> Result<Credential, SchemaError> {
    let (statement, signature) = crosskey_claim(first, second, statement_generator)?;
    let schema = Crosskey::new(delimitor.to_owned(), signature, statement, second, first)?;
    Ok(schema.credential(first)?)
}

const DEFAULT_DELIMITER: &str = "\n";

pub fn default_statement<T: SignerType, U: SignerType>(
    first: &dyn Signer<T>,
    second: &dyn Signer<U>,
) -> String {
    format!(
        "{} {} is linked to {} {}",
        first.signer_type().name(),
        first.id(),
        second.signer_type().name(),
        second.id()
    )
}

pub fn default_crosskey_credential<T: SignerType, U: SignerType>(
    first: &dyn Signer<T>,
    second: &dyn Signer<U>,
) -> Result<Credential, SchemaError> {
    crosskey_credential(first, second, default_statement, DEFAULT_DELIMITER)
}
