use crate::signer::signer::{Signer, SignerError, SignerMethods};

pub fn crosskey_claim<T: SignerMethods, U: SignerMethods>(
    first: &Signer<T>,
    second: &Signer<U>,
    message_generator: &dyn Fn(&Signer<T>, &Signer<U>) -> String,
    delimitor: &str,
) -> Result<String, SignerError> {
    let message = message_generator(first, second);
    let sig1 = first.sign(&message)?;
    let sig2 = second.sign(&message)?;
    Ok(format!("{}{}{}{}{}", message, delimitor, sig1, delimitor, sig2))
}

const DEFAULT_DELIMITER: &str = "\n";

pub fn default_message<T: SignerMethods, U: SignerMethods>(
    first: &Signer<T>,
    second: &Signer<U>,
) -> String {
    format!(
        "{} {} is linked to {} {}",
        first.name, first.id, second.name, second.id
    )
}

pub fn default_crosskey_claim<T: SignerMethods, U: SignerMethods>(
    first: &Signer<T>,
    second: &Signer<U>
) -> Result<String, SignerError> {
    crosskey_claim(first, second,  &default_message, DEFAULT_DELIMITER)
}
