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

    constructor(client: WasmClient) {
        this.client = client;
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

