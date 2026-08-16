<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\CategoryData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class CategoriesResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,CategoryData> $categories */
        public Collection $categories,
    ) {}
}
