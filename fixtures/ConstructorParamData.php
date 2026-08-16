<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class ConstructorParamData extends Data
{
    /**
     * @param array<int, string> $items
     * @param string $name
     */
    public function __construct(
        public array $items,
        public string $name,
    ) {}
}
