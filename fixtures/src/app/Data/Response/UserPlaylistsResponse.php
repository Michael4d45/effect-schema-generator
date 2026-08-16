<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\PlaylistData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class UserPlaylistsResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,PlaylistData> $playlists */
        public Collection $playlists,
    ) {}
}