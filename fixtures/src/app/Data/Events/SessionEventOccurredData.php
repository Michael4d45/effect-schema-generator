<?php

declare(strict_types=1);

namespace App\Data\Events;

use Illuminate\Support\Carbon;
use Spatie\LaravelData\Data;

class SessionEventOccurredData extends Data
{
    public function __construct(
        public int|string|null $user_id,
        public Carbon $timestamp,
    ) {}
}
