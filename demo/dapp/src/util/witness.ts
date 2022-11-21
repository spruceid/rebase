import { TwitterIcon, GlobeIcon, GitHubIcon, DiscordIcon, EmailIcon, RedditIcon, SoundCloudIcon } from "src/components/icons";
import type { CredentialType } from "./claim";
import { Client } from "@rebase-xyz/rebase-client";

const witnessUrl = process.env.WITNESS_URL;
const statementUrl = `${witnessUrl}/statement`;
const instructionsUrl = `${witnessUrl}/instructions`;
const jwtUrl = `${witnessUrl}/witness`;

export const client = new Client(instructionsUrl, statementUrl, jwtUrl);

export function needsDelimitor(c: CredentialType): boolean {
    switch (c) {
        case "github": 
        case "twitter":
            return true;
        default:
            return false;
    }
}

export type PKHType = EIP155 | Solana;
export interface EIP155 {
    eip155: {
        address: string;
        chain_id: string;
    }
}

export interface Solana {
    solana: {
        address: string;
    }
}

export interface Subject {
    pkh?: PKHType,
    web?: string;
}
export type Workflow = "signer" | "statement" | "signature" | "witness" | "complete"

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
    email: EmailIcon,
    github: GitHubIcon,
    discord: DiscordIcon,
    reddit: RedditIcon,
    soundcloud: SoundCloudIcon,
};

export const titleCase = (s) => {
    if(s === 'github'){
        return 'GitHub';
    } else if (s == 'soundcloud'){
        return 'SoundCloud';
    } else {
        return s.charAt(0).toUpperCase() + s.slice(1);
    }
}

interface WitnessInfo {
    statement: string,
    statement_label: string,
    statement_placeholder: string,
    witness: string,
    witness_label: string,
    witness_placeholder: string,
}

function witness_info(t: CredentialType): WitnessInfo {
    let statement = `Enter your ${titleCase(t)} account handle to verify and include it in a message signed via your wallet.`;
    let statement_label = "Enter Account Handle";
    let statement_placeholder =  `Enter your ${titleCase(t)} handle`;
    switch (t) {
        case "github":
            return {
                statement,
                statement_label,
                statement_placeholder,
                witness: "Create a Gist with this message to create a link between your identifier and your GitHub handle.",
                witness_label: "Create a Gist",
                witness_placeholder: "Paste your gist URL"
            }
        case "twitter":
            return {
                statement,
                statement_label,
                statement_placeholder,
                witness: "Tweet out your signed message to create a link between your identifier and your Twitter handle.", 
                witness_label: "Tweet Message",
                witness_placeholder: "Paste your tweet URL"
            }
        case "reddit":
            return {
                statement,
                statement_label,
                statement_placeholder,
                witness: "Update your Reddit account's About section to only include this signature:",
                witness_label: "Update your profile's About section",
                witness_placeholder: "N/A"
            }
        case "soundcloud":
            return {
                statement: "Enter the link to your SoundCloud profile",
                statement_label: "Enter your SoundCloud profile url",
                statement_placeholder: "Enter your SoundCloud profile link",
                witness: "Update your SoundCloud profile's Bio section to only include this signature:", 
                witness_label: "Update your profile's Bio section",
                witness_placeholder: "N/A"
            }
        // case "discord":
        default:
            return {
                statement,
                statement_label,
                statement_placeholder,
                witness: "",
                witness_label: "",
                witness_placeholder: "N/A"
            }
    }
}

export const instructions = async (t: CredentialType): Promise<Instructions> => {
    let {statement, statement_label, statement_placeholder, witness, witness_label, witness_placeholder} = witness_info(t);
    switch (t) {
        // case "discord":
        case "github":
        case "twitter":
        case "reddit":
        case "soundcloud":
            return {
                icon: ICONS[t],
                title: `${titleCase(t)} Verification Workflow`,
                subtitle: `This process is used to link your ${titleCase(t)} account to your identifier by signing a 
                        message using your private key, entering your ${titleCase(t)} handle, and finally, generating
                        a message to post.`,
                statement,
                statement_label,
                statement_placeholder,
                signature: `Sign the message presented to you containing your ${titleCase(t)} handle and additional 
                            information.`,
                signature_label: `Signature Prompt`,
                witness,
                witness_label,
                witness_placeholder            
            }
        case "email":
            return {
                icon: ICONS[t],
                title: `${titleCase(t)} Verification Workflow`,
                subtitle: `This process is used to link your ${titleCase(t)} address to your identifier by signing a 
                        message using your private key, entering your ${titleCase(t)} address, and finally, presenting 
                        a challenge generated by the witness sent to your e-mail`,
                statement: "Enter your e-mail address to verify and include it in a message signed via your wallet.",
                statement_label: "Enter your e-mail address.",
                statement_placeholder: "Enter you e-mail address",
                signature: `Sign the message presented to you containing your ${titleCase(t)} address and additional 
                            information.`,
                signature_label: `Signature Prompt`,
                witness: "Find the verification e-mail in your inbox and copy the sent challenge to the input below.",
                witness_label: "Paste the Challenge found in your email",
                witness_placeholder: "Paste the challenge"
            }
        case "dns":
            return {
                icon: ICONS[t],
                title: `${titleCase(t)} Verification Workflow`,
                subtitle: `This process is used to link your Web Domain to your identifier by entering your domain, 
                            signing a message using your private key, and finally, generating a message to verify.`,
                statement: "Enter the Web Domain you wish to prove ownership of.",
                statement_label: "Enter the Domain",
                statement_placeholder: `Enter your domain (example.com)`,
                signature: `Sign the message presented to you containing your domain and additional 
                            information.`,
                signature_label: `Signature Prompt`,
                witness: `In your DNS settings, add a new TXT record for @ and copy and put the
                        following message as the value. Keep in mind that DNS propagation can take
                        some time. This process may take a few minutes for the verification to
                        successfully complete.`,
                witness_label: "Add a TXT record with the Message",
                witness_placeholder: "N/A",
            }
    }

    throw new Error(`No instructions found for credential type: ${t}`);
}
