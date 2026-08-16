<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;
use Carbon\Carbon;

/**
 * Product review data
 */
class ProductReviewData extends Data
{
    public function __construct(
        public string $id,
        public string $userId,
        public string $userName,
        public int $rating, // 1-5 stars
        public string $title,
        public string $comment,
        public bool $isVerifiedPurchase,
        public bool $isRecommended,
        public Carbon $createdAt,
        public ?Carbon $updatedAt,

        // Nested user data
        public ?UserData $user,
    ) {}
}