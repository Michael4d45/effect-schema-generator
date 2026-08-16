<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Attributes\Lazy;
use Illuminate\Support\Collection;

/**
 * Self-referencing tree structure
 */
class TreeNodeData extends Data
{
    public function __construct(
        public string $id,
        public string $name,
        public ?string $value,

        /** @var Collection<array-key, TreeNodeData> */
        public Collection $children,

        #[Lazy]
        public ?TreeNodeData $parent,

        // Complex nested structure
        /** @var Collection<array-key, Collection<array-key, TreeNodeData>> */
        public Collection $nestedChildren,
    ) {}
}