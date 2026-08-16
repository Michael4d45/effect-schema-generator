<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\CategoryData;
use App\Data\Models\MusicTrackData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class CategoryResponse extends Data
{
    public function __construct(
        public CategoryData $category,
        /** @var Collection<array-key,MusicTrackData> $tracks */
        public Collection $tracks,
        public int $questions_count,
    ) {}
}
