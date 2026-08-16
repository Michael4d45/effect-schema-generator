<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Support;

/**
 * Flatten Laravel effect-schema config (discoverers + transformer FQCNs)
 * into the JSON the Rust binary expects.
 */
class ConfigPayload
{
    /**
     * @var array<string, string>
     */
    private const TRANSFORMER_TO_PLUGIN = [
        'EffectSchemaGenerator\\Plugins\\LengthAwarePaginatorPlugin' => 'length_aware_paginator',
        'EffectSchemaGenerator\\Plugins\\LazyOptionalPlugin' => 'lazy_optional',
        'EffectSchemaGenerator\\Plugins\\HiddenPlugin' => 'hidden',
        'EffectSchemaGenerator\\Plugins\\SnakeCasePlugin' => 'snake_case',
        'EffectSchemaGenerator\\Plugins\\SnakeCaseAttributePlugin' => 'snake_case_attribute',
        'EffectSchemaGenerator\\Plugins\\DatePlugin' => 'date',
        'EffectSchemaGenerator\\Plugins\\CollectionPlugin' => 'collection',
        'EffectSchemaGenerator\\Writer\\TypeEnumWriter' => 'type_enum',
        'EffectSchemaGenerator\\Writer\\EffectSchemaEnumWriter' => 'effect_schema_enum',
        'EffectSchemaGenerator\\Writer\\ConstObjectEnumWriter' => 'const_object_enum',
        'App\\EffectSchema\\Plugins\\UploadedFilePlugin' => 'uploaded_file',
    ];

    /**
     * @param  array<string, mixed>  $config
     * @return array{
     *     data_paths: list<string>,
     *     enum_paths: list<string>,
     *     output_directory: string,
     *     clear_output_directory_before_write: bool,
     *     phpdoc_overrides_types: bool,
     *     plugins: list<string>
     * }
     */
    public static function fromLaravelConfig(array $config, string $defaultOutput): array
    {
        $dataPaths = self::stringList($config['data_paths'] ?? []);
        if ($dataPaths === []) {
            $dataPaths = self::pathsFromDiscoverers($config['data_discoverers'] ?? []);
        }

        $enumPaths = self::stringList($config['enum_paths'] ?? []);
        if ($enumPaths === []) {
            $enumPaths = self::pathsFromDiscoverers($config['enum_discoverers'] ?? []);
        }

        $plugins = self::stringList($config['plugins'] ?? []);
        if ($plugins === []) {
            $plugins = self::pluginsFromTransformers($config['transformers'] ?? []);
        }

        $output = $config['output'] ?? [];
        if (!is_array($output)) {
            $output = [];
        }

        return [
            'data_paths' => $dataPaths,
            'enum_paths' => $enumPaths,
            'output_directory' => is_string($output['directory'] ?? null)
                ? $output['directory']
                : $defaultOutput,
            'clear_output_directory_before_write' => (bool) (
                $output['clear_output_directory_before_write'] ?? true
            ),
            'phpdoc_overrides_types' => (bool) ($config['phpdoc_overrides_types'] ?? true),
            'plugins' => $plugins,
        ];
    }

    /**
     * @param  mixed  $discoverers
     * @return list<string>
     */
    private static function pathsFromDiscoverers(mixed $discoverers): array
    {
        if (!is_array($discoverers)) {
            return [];
        }

        $paths = [];
        foreach ($discoverers as $entry) {
            if (!is_array($entry)) {
                continue;
            }
            foreach (self::stringList($entry['paths'] ?? []) as $path) {
                $paths[] = $path;
            }
        }

        return array_values(array_unique($paths));
    }

    /**
     * @param  mixed  $transformers
     * @return list<string>
     */
    private static function pluginsFromTransformers(mixed $transformers): array
    {
        if (!is_array($transformers)) {
            return [];
        }

        $plugins = [];
        foreach ($transformers as $transformer) {
            if (!is_string($transformer) || $transformer === '') {
                continue;
            }
            $plugins[] = self::TRANSFORMER_TO_PLUGIN[$transformer] ?? $transformer;
        }

        return $plugins;
    }

    /**
     * @param  mixed  $value
     * @return list<string>
     */
    private static function stringList(mixed $value): array
    {
        if (!is_array($value)) {
            return [];
        }

        $out = [];
        foreach ($value as $item) {
            if (is_string($item) && $item !== '') {
                $out[] = $item;
            }
        }

        return array_values($out);
    }
}
