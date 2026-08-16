<?php

declare(strict_types=1);

namespace App\Data\Response;

use Spatie\LaravelData\Data;

class AuthenticateBroadcastingResponse extends Data
{
    public function __construct(
        public string $auth,
    ) {}
}
