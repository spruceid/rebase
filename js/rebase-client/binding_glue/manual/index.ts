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

    // async instructions(type: Types.InstructionsType): Promise<Types.Instructions> {
    async instructions(type: Types.InstructionsType): Promise<Types.Instructions> {
        let req: Types.InstructionsReq = {
            type,
        };
        let res = await this.client.instructions(JSON.stringify(req));
        res = JSON.parse(res);

        return res as Types.Instructions;
    }

    // async statement(req: Types.StatementReq): Promise<Types.FlowResponse> {
    async statement(req: Types.StatementReq): Promise<Types.FlowResponse> {
        let res = await this.client.statement(JSON.stringify(req));
        return JSON.parse(res) as Types.FlowResponse;
    }

    async jwt(req: Types.WitnessReq): Promise<Types.WitnessJWTRes> {
        let res = await this.client.jwt(JSON.stringify(req));
        return JSON.parse(res) as Types.WitnessJWTRes;
    }

    // TODO: Regen the bindings, the use Types.VerificationReq
    async verify(req: Types.WitnessJWTRes): Promise<Types.VerifyRes> {
        let res = await this.client.verify_jwt(JSON.stringify(req));
        return JSON.parse(res) as Types.VerifyRes;
    }
}

