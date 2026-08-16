<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Support;

class BinaryResolver
{
    /**
     * @param  array<string, mixed>  $config
     */
    public static function resolve(array $config, string $basePath): string
    {
        $fromEnv = getenv('EFFECT_SCHEMA_BIN');
        if (is_string($fromEnv) && $fromEnv !== '' && is_file($fromEnv) && is_executable($fromEnv)) {
            return $fromEnv;
        }

        if (isset($config['binary']) && is_string($config['binary']) && $config['binary'] !== '') {
            $configured = $config['binary'];
            if (is_file($configured) && is_executable($configured)) {
                return $configured;
            }
        }

        $fromBundled = self::npmBundledBinary($basePath);
        if ($fromBundled !== null) {
            return $fromBundled;
        }

        $fromNpm = self::npmNativeBinary($basePath);
        if ($fromNpm !== null) {
            return $fromNpm;
        }

        $packageRoot = dirname(__DIR__, 2);
        $crateRoot = dirname($packageRoot);

        $candidates = [
            $basePath . '/vendor/bin/effect-schema',
            $packageRoot . '/bin/effect-schema',
            $crateRoot . '/target/release/effect-schema',
            $crateRoot . '/target/debug/effect-schema',
        ];

        foreach ($candidates as $path) {
            if (is_file($path) && is_executable($path)) {
                return $path;
            }
        }

        return $fromEnv !== false && $fromEnv !== '' ? $fromEnv : 'effect-schema';
    }

    private static function npmNativeBinary(string $basePath): ?string
    {
        $arch = match (php_uname('m')) {
            'x86_64', 'amd64' => 'x64',
            'aarch64', 'arm64' => 'arm64',
            default => null,
        };
        if ($arch === null) {
            return null;
        }

        $names = [];
        if (PHP_OS_FAMILY === 'Linux') {
            $musl = is_file('/lib/ld-musl-x86_64.so.1') || is_file('/lib/ld-musl-aarch64.so.1');
            if ($musl) {
                $names[] = "effect-schema-generator-linux-{$arch}-musl";
            } else {
                $names[] = "effect-schema-generator-linux-{$arch}-gnu";
                $names[] = "effect-schema-generator-linux-{$arch}-musl";
            }
        } elseif (PHP_OS_FAMILY === 'Darwin') {
            $names[] = "effect-schema-generator-darwin-{$arch}";
        } elseif (PHP_OS_FAMILY === 'Windows') {
            $names[] = "effect-schema-generator-win32-{$arch}";
        }

        foreach ($names as $name) {
            $binName = PHP_OS_FAMILY === 'Windows' ? 'effect-schema.exe' : 'effect-schema';
            $path = $basePath . '/node_modules/@michael4d45/' . $name . '/' . $binName;
            if (is_file($path) && (PHP_OS_FAMILY === 'Windows' || is_executable($path))) {
                return $path;
            }
        }

        return null;
    }

    private static function npmBundledBinary(string $basePath): ?string
    {
        $arch = match (php_uname('m')) {
            'x86_64', 'amd64' => 'x64',
            'aarch64', 'arm64' => 'arm64',
            default => null,
        };
        if ($arch === null) {
            return null;
        }

        $dirs = [];
        if (PHP_OS_FAMILY === 'Linux') {
            $musl = is_file('/lib/ld-musl-x86_64.so.1') || is_file('/lib/ld-musl-aarch64.so.1');
            $dirs = $musl
                ? ["linux-{$arch}-musl"]
                : ["linux-{$arch}-gnu", "linux-{$arch}-musl"];
        } elseif (PHP_OS_FAMILY === 'Darwin') {
            $dirs = ["darwin-{$arch}"];
        } elseif (PHP_OS_FAMILY === 'Windows') {
            $dirs = ["win32-{$arch}"];
        }

        $binName = PHP_OS_FAMILY === 'Windows' ? 'effect-schema.exe' : 'effect-schema';
        $root = $basePath . '/node_modules/@michael4d45/effect-schema-generator/binaries/';
        foreach ($dirs as $dir) {
            $path = $root . $dir . '/' . $binName;
            if (is_file($path) && (PHP_OS_FAMILY === 'Windows' || is_executable($path))) {
                return $path;
            }
        }

        return null;
    }
}
