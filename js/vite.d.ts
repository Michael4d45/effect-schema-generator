import type { Plugin } from 'vite';

export interface LaravelDataTypesOptions {
    /** Project-relative path to effect-schema.toml / .json. */
    config?: string;
    /** Override binary path (else EFFECT_SCHEMA_BIN, then the npm platform package). */
    binary?: string;
    /** Override watch globs. Defaults to `{data,enum}_paths` from the config file. */
    patterns?: string[];
}

/** Vite plugin: spawn `effect-schema transform` on build and PHP HMR. */
export function laravelDataTypes(options?: LaravelDataTypesOptions): Plugin;
