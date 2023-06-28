import type { WasmClient } from "./wasm";
import * as Types from "./bindings";

export type {
    Types
}

const DEFAULT_WITNESS_URL = "https://rebasedemo.spruceid.workers.dev"

// This returns a client config pointed to the Spruce ID witness.
export const defaultClientConfig = (): Types.ClientConfig => {
    return {
        endpoints: { 
            instructions: `${DEFAULT_WITNESS_URL}/instructions`,
            statement: `${DEFAULT_WITNESS_URL}/statement`,
            witness_jwt: `${DEFAULT_WITNESS_URL}/witness_jwt`,
            witness_ld: `${DEFAULT_WITNESS_URL}/witness_ld`,
            verify: `${DEFAULT_WITNESS_URL}/verify`
        }
    }
};

export class Client {
    readonly client: WasmClient; 
    private delegated_attestation_config: Types.DelegatedAttestationConfig | null = null;

    constructor(client: WasmClient) {
        this.client = client;
    }

    set_delegated_attestation_config(config: Types.DelegatedAttestationPreConfig, signature: string) {
        let dac: Types.DelegatedAttestationConfig = {
            service_key: config.service_key,            
            session_config: config.session_config,
            siwe_recap_message: config.siwe_recap_message,
            siwe_recap_signature: signature 
        };

        this.delegated_attestation_config = dac;
    }

    async delegated_attestation_jwt(statement: Types.AttestationStatement) {
        if (!this.delegated_attestation_config) {
            throw new Error("Must supply a DelegatedAttestationConfig before calling delegated_attestation_jwt");
        }

        let dac = JSON.stringify(this.delegated_attestation_config);
        let stmt = JSON.stringify(statement);

        let res = await this.client.delegated_attestation_jwt(dac, stmt);
        return JSON.parse(res) as Types.JWTWrapper;
    }

    async new_jwk(): Promise<string> {
        return await this.client.new_jwk();
    }

    async siwe_message(session_config: Types.SessionConfig, witness: string, delegated_capabilities: Array<Types.AttestationTypes>): Promise<Types.DelegatedAttestationConfig> {
        let sc = JSON.stringify(session_config);
        let dc = JSON.stringify(delegated_capabilities);
        let res = await this.client.siwe_message(sc, witness, dc);
        return JSON.parse(res) as Types.DelegatedAttestationConfig;
    }

    async instructions(type: Types.FlowType): Promise<Types.Instructions> {
        let req: Types.InstructionsReq = {
            type,
        };
        let res = await this.client.instructions(JSON.stringify(req));
        res = JSON.parse(res);

        return res as Types.Instructions;
    }

    async statement(req: Types.Statements): Promise<Types.StatementResponse> {
        let res = await this.client.statement(JSON.stringify(req));
        return JSON.parse(res) as Types.StatementResponse;
    }

    async witness_jwt(req: Types.Proofs): Promise<Types.JWTWrapper> {
        let res = await this.client.witness_jwt(JSON.stringify(req));
        return JSON.parse(res) as Types.JWTWrapper;
    }

    async witness_ld(req: Types.Proofs): Promise<Types.CredentialWrapper> {
        let res = await this.client.witness_ld(JSON.stringify(req));
        return JSON.parse(res) as Types.CredentialWrapper;
    }

    async verify(req: Types.VCWrapper): Promise<Types.VerifyRes> {
        let res = await this.client.verify(JSON.stringify(req));
        return JSON.parse(res) as Types.VerifyRes;
    }
}

