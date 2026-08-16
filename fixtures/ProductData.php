<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Attributes\Lazy;
use Spatie\LaravelData\Attributes\MapInputName;
use Spatie\LaravelData\Attributes\MapOutputName;
use Spatie\LaravelData\Attributes\WithCast;
use Carbon\Carbon;
use Illuminate\Support\Collection;

/**
 * Product data with various attributes and transformations
 */
class ProductData extends Data
{
    public function __construct(
        public string $id,
        public string $sku,

        #[MapInputName('product_name')]
        #[MapOutputName('name')]
        public string $productName,

        public string $description,
        public float $price,
        public int $stockQuantity,

        // Currency as union type
        public string|int $currencyCode,

        // Categories with complex types
        /** @var Collection<array-key, CategoryData> */
        public Collection $categories,

        // Images as transformed data
        /** @var Collection<array-key, ProductImageData> */
        #[WithCast('json')]
        public Collection $images,

        // Dates with different formats
        public Carbon $createdAt,
        public Carbon $updatedAt,
        public ?Carbon $publishedAt,

        // Status with enum
        public TestStatus $status,

        // Flexible attributes
        /** @var array<string, mixed> */
        public array $attributes,

        // SEO data
        public ?string $metaTitle,
        public ?string $metaDescription,

        // Related products (lazy loaded)
        #[Lazy]
        /** @var Collection<array-key, ProductData> */
        public Collection $relatedProducts,

        // Reviews
        /** @var Collection<array-key, ProductReviewData> */
        public Collection $reviews,
    ) {}
}