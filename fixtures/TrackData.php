<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class TrackData extends Data
{
    public function __construct(
        public string $name,
        /** @var MarkerData[] */
        public array $markers,
    ) {}
}