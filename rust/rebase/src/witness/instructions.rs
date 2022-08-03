use crate::witness::{
    dns::Claim as DnsClaim,
    github::{Claim as GitHubClaim, Opts as GitHubOpts},
    self_signed::{Claim as SelfSignedClaim, Opts as SelfSignedOpts},
    twitter::{Claim as TwitterClaim, Opts as TwitterOpts},
    witness::WitnessError,
};
use schemars::{schema::RootSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Deserialize, Serialize)]
pub struct Instructions {
    pub statement: String,
    pub signature: String,
    pub witness: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum InstructionTypes {
    #[serde(rename = "dns")]
    Dns,
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "self_signed")]
    SelfSigned,
    #[serde(rename = "twitter")]
    Twitter,
}

impl InstructionTypes {
    fn instructions(&self) -> Instructions {
        match self {
            &InstructionTypes::Dns => Instructions {
                statement: "Enter the Web Domain you wish to prove ownership of.".to_string(),
                signature: "Sign the message presented to you containing your domain and additional information.".to_string(),
                witness: "In your DNS settings, add a new TXT record for @ and copy and put the following message as the value. Keep in mind that DNS propagation can take some time. This process may take a few minutes for the verification to successfully complete.".to_string(),
            },
            &InstructionTypes::GitHub => Instructions {
                statement: "Enter your GitHub account handle to verify and include in a signed message using your wallet.".to_string(),
                signature: "Sign the message presented to you containing your GitHub handle and additional information.".to_string(),
                witness: "Create a Gist with this message to create a link between your identifier and your GitHub handle.".to_string(),
            },
            &InstructionTypes::SelfSigned => Instructions {
                statement: "Please enter both of the signers you wish to link along with what type of signer they are".to_string(),
                signature: "Please sign the presented statement with the signers entered in the previous step in the same order as provided".to_string(),
                witness: "Send the signatures and signer information to the witness".to_string(),
            },
            &InstructionTypes::Twitter =>  Instructions {
                statement: "Enter your Twitter account handle to verify and include in a signed message using your wallet.".to_string(),
                signature: "Sign the message presented to you containing your Twitter handle and additional information.".to_string(),
                witness: "Tweet out your signed message to create a link between your identifier and your Twitter handle.".to_string(),
            }
        }
    }

    fn schemas(&self) -> (RootSchema, RootSchema) {
        match &self {
            &InstructionTypes::Dns => (schema_for!(DnsClaim), schema_for!(DnsClaim)),
            &InstructionTypes::GitHub => (schema_for!(GitHubOpts), schema_for!(GitHubClaim)),
            &InstructionTypes::SelfSigned => {
                (schema_for!(SelfSignedOpts), schema_for!(SelfSignedClaim))
            }
            &InstructionTypes::Twitter => (schema_for!(TwitterOpts), schema_for!(TwitterClaim)),
        }
    }

    pub fn ui_hints(&self) -> Result<serde_json::Value, WitnessError> {
        let (statement, witness) = self.schemas();
        Ok(json!({
            "instructions": &self.instructions(),
            "statement_schema": statement,
            "witness_schema": witness
        }))
    }
}
