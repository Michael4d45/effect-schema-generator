<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

/**
 * Product image data
 */
class ProductImageData extends Data
{
    public function __construct(
        public string $id,
        public string $url,
        public string $altText,
        public int $sortOrder,
        public bool $isPrimary,
        public ?string $thumbnailUrl,
        public int $width,
        public int $height,
    ) {}
}