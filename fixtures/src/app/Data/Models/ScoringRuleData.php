<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class ScoringRuleData extends Data
{
    public function __construct(
        public string $id,
        public ?string $name,
        public int $base_points,
        public ?float $decay_factor,
        public ?int $max_time_ms,
        public bool $streak_bonus_enabled,
        public float $streak_multiplier,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var Collection<array-key,GameSessionData>|Lazy $game_sessions */
        #[AutoWhenLoadedLazy('gameSessions')]
        public Collection|Lazy $game_sessions,
    ) {}
}
