<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\MusicSourceData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class MusicSourcesResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,MusicSourceData> $music_sources */
        public Collection $music_sources,
    ) {}
}