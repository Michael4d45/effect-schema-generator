<?php

declare(strict_types=1);

namespace App\Data\Events;

use Illuminate\Support\Carbon;
use Spatie\LaravelData\Data;

class RealtimeMessageData extends Data
{
    public function __construct(
        public string $message,
        public Carbon $timestamp,
    ) {}
}
