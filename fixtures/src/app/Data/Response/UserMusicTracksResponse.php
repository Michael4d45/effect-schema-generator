<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\MusicTrackData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class UserMusicTracksResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,MusicTrackData> $tracks */
        public Collection $tracks,
    ) {}
}