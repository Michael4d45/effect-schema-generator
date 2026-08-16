<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class ComplexMetadataData extends Data
{
    /**
     * @param array<string, array{name: string, value: int|string|null}> $metadata
     */
    public function __construct(
        public array $metadata,
    ) {}
}
