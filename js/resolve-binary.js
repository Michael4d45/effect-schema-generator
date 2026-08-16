import { createRequire } from 'node:module';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);
const packageRoot = path.dirname(fileURLToPath(import.meta.url));

export const isMusl = () => {
    if (process.platform !== 'linux') {
        return false;
    }
    try {
        const glibcVersion = process.report?.getReport?.()?.header?.glibcVersionRuntime;
        if (typeof glibcVersion === 'string' && glibcVersion.length > 0) {
            return false;
        }
        if (glibcVersion === undefined && process.report?.getReport) {
            return true;
        }
    } catch {
        // fall through to linker checks
    }
    return (
        existsSync('/lib/ld-musl-x86_64.so.1') ||
        existsSync('/lib/ld-musl-aarch64.so.1')
    );
};

const bundledBinaryDirs = () => {
    const { platform, arch } = process;
    if (platform === 'linux') {
        const musl = `linux-${arch}-musl`;
        const gnu = `linux-${arch}-gnu`;
        return isMusl() ? [musl] : [gnu, musl];
    }
    if (platform === 'darwin') {
        return [`darwin-${arch}`];
    }
    if (platform === 'win32') {
        return [`win32-${arch}`];
    }
    return [];
};

export const platformPackageNames = () => {
    const { platform, arch } = process;
    if (platform === 'linux') {
        const musl = `@michael4d45/effect-schema-generator-linux-${arch}-musl`;
        const gnu = `@michael4d45/effect-schema-generator-linux-${arch}-gnu`;
        return isMusl() ? [musl] : [gnu, musl];
    }
    if (platform === 'darwin') {
        return [`@michael4d45/effect-schema-generator-darwin-${arch}`];
    }
    if (platform === 'win32') {
        return [`@michael4d45/effect-schema-generator-win32-${arch}`];
    }
    return [];
};

const binaryFromPackage = (pkgName) => {
    try {
        const pkgJson = require.resolve(`${pkgName}/package.json`);
        const binName =
            process.platform === 'win32' ? 'effect-schema.exe' : 'effect-schema';
        const bin = path.join(path.dirname(pkgJson), binName);
        return existsSync(bin) ? bin : null;
    } catch {
        return null;
    }
};

/**
 * Resolve the native effect-schema binary for this platform.
 * @param {string} [projectRoot]
 * @returns {string}
 */
export const resolveBinary = (projectRoot) => {
    const fromEnv = process.env.EFFECT_SCHEMA_BIN;
    if (fromEnv && existsSync(fromEnv)) {
        return fromEnv;
    }

    for (const pkgName of platformPackageNames()) {
        const fromPkg = binaryFromPackage(pkgName);
        if (fromPkg) {
            return fromPkg;
        }
    }

    for (const dir of bundledBinaryDirs()) {
        const binName =
            process.platform === 'win32' ? 'effect-schema.exe' : 'effect-schema';
        const bundled = path.join(packageRoot, 'binaries', dir, binName);
        if (existsSync(bundled)) {
            return bundled;
        }
    }

    const crateRoot = path.resolve(packageRoot, '..');
    for (const candidate of [
        path.join(crateRoot, 'target/release/effect-schema'),
        path.join(crateRoot, 'target/debug/effect-schema'),
    ]) {
        if (existsSync(candidate)) {
            return candidate;
        }
    }

    if (projectRoot) {
        const vendorBin = path.join(projectRoot, 'vendor/bin/effect-schema');
        if (existsSync(vendorBin)) {
            return vendorBin;
        }
    }

    const expected = platformPackageNames()[0] ?? 'this platform';
    throw new Error(
        `Native effect-schema binary not found for ${process.platform}-${process.arch}` +
            `${process.platform === 'linux' ? ` (${isMusl() ? 'musl' : 'gnu'})` : ''}. ` +
            `npm install should pull @michael4d45/effect-schema-generator (binary for ${expected}). ` +
            `Set EFFECT_SCHEMA_BIN to override.`,
    );
};
