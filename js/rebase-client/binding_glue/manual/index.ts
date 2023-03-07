// TODO: This file will be used to replace rebase_client.js
import * as wasm from "./rebase_client_bg.wasm";
export * from "./rebase_client_bg.js";
import { Client } from "./rebase_client_bg.js";
import * as Bindings from "./bindings/index";

export class RebaseClient {
    readonly client: Client

    constructor(endpoints: Bindings.Endpoints) {
        let s = JSON.stringify(endpoints);
        this.client = new Client(s);
    }

    async instructions(type: Bindings.InstructionsType): Promise<Instructions> {
        let req: Bindings.InstructionsReq = {
            type,
        };
        let res = await this.client.instructions(JSON.stringify(req));
        res = JSON.parse(res);

        return res as Bindings.Instructions;
    }

    async statement(req: Bindings.StatementReq): Promise<Bindings.FlowResponse> {
        let res = await this.client.statement(JSON.stringify(req));
        return JSON.parse(res) as Bindings.FlowResponse;
    }

    async jwt(req: Bindings.WitnessReq): Promise<Bindings.WitnessJWTRes> {
        let res = await this.client.jwt(JSON.stringify(req));
        return JSON.parse(res) as Bindings.WitnessJWTRes;
    }

    // TODO: Regen the bindings, the use Bindings.VerificationReq
    async verify(req: Bindings.WitnessJWTRes): Promise<Bindings.VerifyRes> {
        let res = await this.client.verify_jwt(JSON.stringify(req));
        return JSON.parse(res) as Bindings.VerifyRes;
    }
}