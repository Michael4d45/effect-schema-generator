<?php

declare(strict_types=1);

return [
    'binary' => env('EFFECT_SCHEMA_BIN', base_path('vendor/bin/effect-schema')),

    /*
     | Direct paths (Rust binary). If empty, paths are taken from
     | data_discoverers / enum_discoverers so existing Laravel configs still work.
     */
    'data_paths' => [
        app_path('Data'),
    ],
    'enum_paths' => [
        app_path('Enums'),
    ],
    'data_discoverers' => [],
    'enum_discoverers' => [],
    'phpdoc_overrides_types' => true,
    'plugins' => [
        'length_aware_paginator',
        'lazy_optional',
        'hidden',
        'snake_case_attribute',
        'date',
        'collection',
        'type_enum',
        'effect_schema_enum',
        'const_object_enum',
    ],
    'output' => [
        'directory' => resource_path('js/schemas'),
        'clear_output_directory_before_write' => true,
    ],
];
