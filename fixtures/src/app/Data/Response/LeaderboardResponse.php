<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\UserStatisticData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class LeaderboardResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,UserStatisticData> $players */
        public Collection $players,
    ) {}
}
