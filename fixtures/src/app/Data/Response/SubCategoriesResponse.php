<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\SubCategoryData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class SubCategoriesResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,SubCategoryData> $sub_categories */
        public Collection $sub_categories,
    ) {}
}