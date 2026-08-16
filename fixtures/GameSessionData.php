<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Carbon\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Optional;

class GameSessionData extends Data
{
    public function __construct(
        public string $id,
        public string $host_id,
        public string $room_code,
        public SessionStatus $status,
        public string $quiz_mode_id,
        public string $scoring_rule_id,
        public ?string $playlist_id,
        public int $max_players,
        public ?Carbon $started_at,
        public ?Carbon $ended_at,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        public Optional|UserData $host,
        public Optional|QuizModeData $quiz_mode,
        public Optional|ScoringRuleData $scoring_rule,
        public Optional|PlaylistData|null $playlist,
        /** @var Collection<array-key,SessionParticipantData>|Optional */
        public Optional|Collection $participants,
        /** @var Collection<array-key,SessionRoundData>|Optional */
        public Optional|Collection $rounds,
        /** @var Collection<array-key,SessionEventData>|Optional */
        public Optional|Collection $events,
        /** @var Collection<array-key,SessionFinalScoreData>|Optional */
        public Optional|Collection $final_scores,
    ) {}
}
