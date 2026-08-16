<?php

declare(strict_types=1);

namespace App\Features\Recipe\Responses;

use App\Data\Models\RecipeData;
use Illuminate\Pagination\LengthAwarePaginator;
use Spatie\LaravelData\Data;

class ListRecipesResponse extends Data
{
    public function __construct(
        /** @var LengthAwarePaginator<array-key, RecipeData> $recipes */
        public LengthAwarePaginator $recipes,
        public null|string $search,
        public null|bool $my_recipes,
        public null|bool $liked_recipes = null,
    ) {}
}
