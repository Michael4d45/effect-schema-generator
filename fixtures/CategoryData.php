<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

/**
 * Category data for products
 */
class CategoryData extends Data
{
    public function __construct(
        public string $id,
        public string $name,
        public string $slug,
        public ?string $description,
        public ?CategoryData $parent,
        public int $sortOrder,
        public bool $isActive,
    ) {}
}