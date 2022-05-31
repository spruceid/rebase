import type { CredentialType } from "./claim";

export type Workflow = "statement" | "signature" | "witness" | "complete"

export type Instructions = {
    statement: string,
    statement_label: string,
    signature: string,
    witness: string
    witness_label: string
}

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
    }

    throw new Error(`No instructions found for credential type: ${t}`);
}