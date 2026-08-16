<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class SubCategoryData extends Data
{
    public function __construct(
        public string $id,
        public string $category_id,
        public string $name,
        public ?string $description,
        public int $sort_order,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var CategoryData|Lazy $category */
        #[AutoWhenLoadedLazy]
        public CategoryData|Lazy $category,
        /** @var Collection<array-key,MusicTrackData>|Lazy $music_tracks */
        #[AutoWhenLoadedLazy('musicTracks')]
        public Collection|Lazy $music_tracks,
    ) {}
}
