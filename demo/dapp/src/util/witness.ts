import { TwitterIcon, GlobeIcon, GitHubIcon, DiscordIcon } from "components";
import type { CredentialType } from "./claim";
export interface KeyType {
    pkh: {
        eip155: {
            address: string;
            chain_id: string;
        };
    };
}
export type Workflow = "statement" | "signature" | "witness" | "complete"

export type Instructions = {
    icon: any,
    title: string,
    subtitle: string,
    statement: string,
    statement_label: string,
    statement_placeholder: string,
    signature: string,
    signature_label: string,
    witness: string
    witness_label: string
    witness_placeholder: string
}

const ICONS = {
    twitter: TwitterIcon,
    dns: GlobeIcon,
    github: GitHubIcon,
    discord: DiscordIcon,
};

const capitalizeFirstLetter = (string) => {
    return string.charAt(0).toUpperCase() + string.slice(1);
}

export const instructions = async (t: CredentialType): Promise<Instructions> => {
    switch (t) {
        case "discord":
        case "github":
        case "twitter":
            return {
                icon: ICONS[t],
                title: `${capitalizeFirstLetter(t)} Verification Workflow`,
                subtitle: `This process is used to link your ${capitalizeFirstLetter(t)} account to your identifier by signing a 
                        message using your private key, entering your ${capitalizeFirstLetter(t)} handle, and finally, generating
                        a message.`,
                statement: `Enter your ${capitalizeFirstLetter(t)} account handle to verify
                        and include in a message signed via your wallet.`,
                statement_label: `Enter Account Handle`,
                statement_placeholder: `Enter your ${capitalizeFirstLetter(t)} handle`,
                signature: `Sign the message presented to you containing your ${capitalizeFirstLetter(t)} handle and additional 
                            information.`,
                signature_label: `Signature Prompt`,
                witness: t === "twitter" ?
                    "Tweet out your signed message to create a link between your identifier and your Twitter handle" :
                    t === "github" ?
                        "Create a Gist with this message to create a link between your identifier and your GitHub handle" :
                        "",
                witness_label: t === "twitter"
                    ? "Tweet Message" :
                    t === "github" ?
                        "Create a Gist" :
                        "",
                witness_placeholder: t === "twitter"
                    ? "Paste your tweet URL" :
                    t === "github" ?
                        "Paste your gist URL" :
                        ""
            }
        case "dns":
            return {
                icon: ICONS[t],
                title: `${capitalizeFirstLetter(t)} Verification Workflow`,
                subtitle: ``,
                statement: "Please enter the web domain you wish to prove ownership of.",
                statement_label: "domain",
                statement_placeholder: `Enter your ${capitalizeFirstLetter(t)} handle`,
                signature: `Sign the message presented to you containing your ${capitalizeFirstLetter(t)} handle and additional 
                            information.`,
                signature_label: `Signature Prompt`,
                witness: `In your DNS settings, add a new TXT record for @ and copy and put the
                        following text as the value. Keep in mind that DNS propagation can take
                        some time. This process may take a few minutes for the verification to
                        successfully complete.`,
                witness_label: "N/A",
                witness_placeholder: "N/A",
            }
    }

    throw new Error(`No instructions found for credential type: ${capitalizeFirstLetter(t)}`);
}