import { execFile } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import osPath from 'node:path';
import { promisify } from 'node:util';
import { minimatch } from 'minimatch';
import { resolveBinary } from './resolve-binary.js';

const execFileAsync = promisify(execFile);

/** @type {import('rollup').PluginContext | undefined} */
let pluginContext;

const watchPatternsFromConfig = (root, configPath) => {
    if (!existsSync(configPath)) {
        return [];
    }
    const text = readFileSync(configPath, 'utf8');
    const paths = [];
    for (const key of ['data_paths', 'enum_paths']) {
        const block = text.match(new RegExp(`${key}\\s*=\\s*\\[([^\\]]*)\\]`, 's'));
        if (!block) {
            continue;
        }
        for (const match of block[1].matchAll(/"([^"]+)"/g)) {
            paths.push(match[1]);
        }
    }
    return paths.map((dir) => {
        const rel = osPath.isAbsolute(dir) ? osPath.relative(root, dir) : dir;
        return `${rel.replaceAll('\\', '/').replace(/\/$/, '')}/**/*.php`;
    });
};

const shouldRun = (patterns, opts) => {
    const file = opts.file.replaceAll('\\', '/');
    return patterns.some((pattern) => {
        const resolved = osPath
            .resolve(opts.server.config.root, pattern)
            .replaceAll('\\', '/');
        return minimatch(file, resolved);
    });
};

/**
 * Vite plugin: spawn `effect-schema transform --config <toml>` on build and PHP HMR.
 * Does not boot Laravel.
 *
 * @param {{ config?: string, binary?: string, patterns?: string[] }} [options]
 * @returns {import('vite').Plugin}
 */
export const laravelDataTypes = ({
    config = 'effect-schema.toml',
    binary,
    patterns,
} = {}) => {
    let root = process.cwd();
    let configPath = osPath.resolve(root, config);
    let watchPatterns = patterns ?? [];
    let bin = binary ?? '';

    const runCommand = async () => {
        try {
            await execFileAsync(bin, ['transform', '--config', configPath], {
                cwd: root,
            });
            pluginContext?.info('[effect-schema] TypeScript types generated.');
        } catch (error) {
            const e = error;
            const details = [e.message || '', e.stdout || '', e.stderr || '']
                .filter(Boolean)
                .join('\n');
            pluginContext?.error(
                '[effect-schema] Failed to generate types:\n' + details,
            );
        }
    };

    return {
        name: 'laravel-data-types',
        enforce: 'pre',
        configResolved(resolved) {
            root = resolved.root;
            configPath = osPath.resolve(root, config);
            bin = binary ?? resolveBinary(root);
            watchPatterns = (patterns ?? watchPatternsFromConfig(root, configPath)).map(
                (pattern) => pattern.replaceAll('\\', '/'),
            );
        },
        buildStart() {
            pluginContext = this;
            return runCommand();
        },
        async handleHotUpdate({ file, server }) {
            if (shouldRun(watchPatterns, { file, server })) {
                await runCommand();
            }
        },
    };
};
