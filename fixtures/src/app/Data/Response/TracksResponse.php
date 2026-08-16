<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\MusicTrackData;
use Illuminate\Pagination\LengthAwarePaginator;
use Spatie\LaravelData\Data;

class TracksResponse extends Data
{
    public function __construct(
        /** @var LengthAwarePaginator<array-key,MusicTrackData> $tracks */
        public LengthAwarePaginator $tracks,
    ) {}
}
