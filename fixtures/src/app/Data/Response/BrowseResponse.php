<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\CategoryData;
use App\Data\Models\MusicTrackData;
use App\Data\Models\PlaylistData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class BrowseResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,CategoryData> $categories */
        public Collection $categories,
        /** @var Collection<array-key,PlaylistData> $featured_playlists */
        public Collection $featured_playlists,
        /** @var Collection<array-key,MusicTrackData> $recent_tracks */
        public Collection $recent_tracks,
    ) {}
}
