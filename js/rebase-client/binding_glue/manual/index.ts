import type { WasmClient } from "./wasm";
import * as Types from "./bindings";

export type {
    Types
}

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

    async statement(req: Types.StatementReq): Promise<Types.FlowResponse> {
        let res = await this.client.statement(JSON.stringify(req));
        return JSON.parse(res) as Types.FlowResponse;
    }

    async witness_jwt(req: Types.WitnessReq): Promise<Types.JWTWrapper> {
        let res = await this.client.witness_jwt(JSON.stringify(req));
        return JSON.parse(res) as Types.JWTWrapper;
    }

    async witness_ld(req: Types.WitnessReq): Promise<Types.CredentialWrapper> {
        let res = await this.client.witness_ld(JSON.stringify(req));
        return JSON.parse(res) as Types.CredentialWrapper;
    }

    async verify(req: Types.VCWrapper): Promise<Types.VerifyRes> {
        let res = await this.client.verify(JSON.stringify(req));
        return JSON.parse(res) as Types.VerifyRes;
    }
}

