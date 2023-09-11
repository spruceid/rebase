import init, {InitInput, WasmClient} from "./pkg/rebase_client.js";

// There is some weirdness around re-exporting types using rollup, see:
// https://github.com/rollup/plugins/issues/71
// This was the cleanest way to re-export a type that I have found:
export type AttestationStatement = import("./pkg/rebase_client.js").AttestationStatement;
export type AttestationTypes = import("./pkg/rebase_client.js").AttestationTypes;
export type BasicProfileAttestationStatement = import("./pkg/rebase_client.js").BasicProfileAttestationStatement;
export type ClientConfig = import("./pkg/rebase_client.js").Client;
export type CredentialWrapper = import("./pkg/rebase_client.js").CredentialWrapper;
export type DelegatedAttestationConfig = import("./pkg/rebase_client.js").DelegatedAttestationConfig;
export type DelegatedAttestationPreConfig = import("./pkg/rebase_client.js").DelegatedAttestationPreConfig;
export type FlowType = import("./pkg/rebase_client.js").FlowType;
export type Instructions = import("./pkg/rebase_client.js").Instructions;
export type InstructionsReq = import("./pkg/rebase_client.js").InstructionsReq;
export type JWTWrapper = import("./pkg/rebase_client.js").JWTWrapper;
export type Proofs = import("./pkg/rebase_client.js").Proofs;
export type SessionConfig = import("./pkg/rebase_client.js").SessionConfig;
export type StatementResponse  = import("./pkg/rebase_client.js").StatementResponse;
export type Statements  = import("./pkg/rebase_client.js").Statements;
export type Subjects = import("./pkg/rebase_client.js").Subjects;
export type VCWrapper = import("./pkg/rebase_client.js").VCWrapper;
export type VerifyRes = import("./pkg/rebase_client.js").VerifyRes;

export type LoadOpts =  {
	wasm?: InitInput,
	config?: ClientConfig,
};

const DEFAULT_WITNESS_URL = "https://rebasedemo.spruceid.workers.dev"

// This returns a client config pointed to the Spruce ID witness.
export const defaultClientConfig = (): ClientConfig => {
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

let wasmInit: (() => InitInput) | undefined = undefined;
export const setWasmInit = (arg: () => InitInput) => {
  wasmInit = arg;
};

let initialized: Promise<void> | undefined = undefined;

export class Client {
    private delegated_attestation_config: DelegatedAttestationConfig | null = null;
	readonly client: WasmClient;

	private constructor(conf: ClientConfig) {
		this.client = new WasmClient(JSON.stringify(conf));
	}

	public static initialize = async (options?: LoadOpts): Promise<Client> => {
		if (initialized === undefined) {
			//@ts-ignore
			const loadModule = options?.wasm ?? wasmInit();
			initialized = init(loadModule).then(() => void 0);
		}

		await initialized;
		let c = options?.config ?? defaultClientConfig();
		return new Client(c)
	}

	set_delegated_attestation_config(config: DelegatedAttestationPreConfig, signature: string) {
        let dac: DelegatedAttestationConfig = {
            service_key: config.service_key,            
            session_config: config.session_config,
            siwe_recap_message: config.siwe_recap_message,
            siwe_recap_signature: signature 
        };

        this.delegated_attestation_config = dac;
    }

	public async delegated_attestation_jwt(statement: AttestationStatement): Promise<JWTWrapper> {
        if (!this.delegated_attestation_config) {
            throw new Error("Must supply a DelegatedAttestationConfig before calling delegated_attestation_jwt");
        }

        let dac = JSON.stringify(this.delegated_attestation_config);
        let stmt = JSON.stringify(statement);

        let res = await this.client.delegated_attestation_jwt(dac, stmt);
        return JSON.parse(res) as JWTWrapper;
    }

	public async new_jwk(): Promise<string> {
        return await this.client.new_jwk();
    }

	public async siwe_message(session_config: SessionConfig, witness: string, delegated_capabilities: Array<AttestationTypes>): Promise<DelegatedAttestationConfig> {
        let sc = JSON.stringify(session_config);
        let dc = JSON.stringify(delegated_capabilities);
        let res = await this.client.siwe_message(sc, witness, dc);
        return JSON.parse(res) as DelegatedAttestationConfig;
    }

	public async instructions(type: FlowType): Promise<Instructions> {
        let req: InstructionsReq = {
            type,
        };
        let res = await this.client.instructions(JSON.stringify(req));
        res = JSON.parse(res);

        return res as Instructions;
    }

	public async statement(req: Statements): Promise<StatementResponse> {
        let res = await this.client.statement(JSON.stringify(req));
        return JSON.parse(res) as StatementResponse;
    }

    public async witness_jwt(req: Proofs): Promise<JWTWrapper> {
        let res = await this.client.witness_jwt(JSON.stringify(req));
        return JSON.parse(res) as JWTWrapper;
    }

    public async witness_ld(req: Proofs): Promise<CredentialWrapper> {
        let res = await this.client.witness_ld(JSON.stringify(req));
        return JSON.parse(res) as CredentialWrapper;
    }

    public async verify(req: VCWrapper): Promise<VerifyRes> {
        let res = await this.client.verify(JSON.stringify(req));
        return JSON.parse(res) as VerifyRes;
    }
}
