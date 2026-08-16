<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\PlaylistData;
use Spatie\LaravelData\Data;

class PlaylistResponse extends Data
{
    public function __construct(
        public PlaylistData $playlist,
    ) {}
}
