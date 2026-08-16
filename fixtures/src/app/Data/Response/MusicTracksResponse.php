<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\MusicTrackData;
use Illuminate\Pagination\LengthAwarePaginator;
use Spatie\LaravelData\Data;

class MusicTracksResponse extends Data
{
    public function __construct(
        /** @var LengthAwarePaginator<int, MusicTrackData> $music_tracks */
        public LengthAwarePaginator $music_tracks,
    ) {}
}
