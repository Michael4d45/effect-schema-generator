<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\GameSessionData;
use App\Data\Models\PlaylistData;
use App\Data\Models\UserStatisticData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class HomeResponse extends Data
{
    public function __construct(
        public UserStatisticData|null $statistic,
        /** @var Collection<array-key,GameSessionData> $recent_sessions */
        public Collection $recent_sessions,
        /** @var Collection<array-key,PlaylistData> $recent_playlists */
        public Collection $recent_playlists,
    ) {}
}
