import type { CredentialType } from "./claim";

export type Workflow = "statement" | "signature" | "witness" | "complete"

export type Instructions = {
    statement: string,
    statement_label: string,
    signature: string,
    witness: string
    witness_label: string
}

// TODO: Support passing full components!
export const instructions = async (t: CredentialType): Promise<Instructions> => {
    switch (t) {
        case "discord":
        case "github":
        case "twitter":
            return {
                statement: `Please input your ${t} handle`,
                statement_label: `handle`,
                signature: `Sign the generated statement`,
                witness: "Please post the generated text, then enter the need information",
                witness_label: t === "twitter" ? "tweet_url" : t === "github" ? "gist_id" : ""
            }
        case "dns":
            return {
                statement: "Please enter the web domain you wish to prove ownership of.",
                statement_label: "domain",
                signature: "Sign the generated statement",
                witness: `In your DNS settings, add a new TXT record for @ and copy and put the
          following text as the value. Keep in mind that DNS propagation can take
          some time. This process may take a few minutes for the verification to
          successfully complete.`,
                witness_label: "N/A"
            }
    }

    throw new Error(`No instructions found for credential type: ${t}`);
}