<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\GameSessionData;
use Spatie\LaravelData\Data;

class SessionLobbyResponse extends Data
{
    public function __construct(
        public GameSessionData $session,
    ) {}
}
