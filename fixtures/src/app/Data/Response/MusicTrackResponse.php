<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\MusicTrackData;
use Spatie\LaravelData\Data;

class MusicTrackResponse extends Data
{
    public function __construct(
        public MusicTrackData $music_track,
    ) {}
}
