<?php

declare(strict_types=1);

namespace App\Data\Events;

use Illuminate\Support\Carbon;
use Spatie\LaravelData\Data;

class TimerUpdatedData extends Data
{
    public function __construct(
        public int $remaining_seconds,
        public string $status,
        public Carbon $timestamp,
    ) {}
}
