<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\GameSessionData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class ActiveGamesResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,GameSessionData> $sessions */
        public Collection $sessions,
    ) {}
}
