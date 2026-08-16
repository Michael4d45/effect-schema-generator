<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class ScoringRuleData extends Data
{
    public function __construct(
        public string $id,
        public string|null $name,
        public int $base_points,
        public float|null $decay_factor,
        public int|null $max_time_ms,
        public bool $streak_bonus_enabled,
        public float $streak_multiplier,
        public Carbon|null $created_at,
        public Carbon|null $updated_at,
        /** @var Collection<array-key,GameSessionData> $game_sessions */
        #[AutoWhenLoadedLazy('gameSessions')]
        public Collection|Lazy $game_sessions,
    ) {}
}
