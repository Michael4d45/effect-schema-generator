<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class CategoryData extends Data
{
    public function __construct(
        public string $id,
        public string $name,
        public ?string $description,
        public ?string $icon_url,
        public int $sort_order,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var Collection<array-key,SubCategoryData>|Lazy $sub_categories */
        #[AutoWhenLoadedLazy('subCategories')]
        public Collection|Lazy $sub_categories,
        /** @var Collection<array-key,UserStatisticData>|Lazy $user_statistics */
        #[AutoWhenLoadedLazy('userStatistics')]
        public Collection|Lazy $user_statistics,
    ) {}
}
