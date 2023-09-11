export * from "./index_core.js";
import rebase_client_wasm from "./pkg/rebase_client_bg.wasm";
import {setWasmInit} from "./rebase_client_wrapper.js"

// @ts-ignore
setWasmInit(() => rebase_client_wasm());
