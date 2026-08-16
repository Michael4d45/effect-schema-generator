<?php

declare(strict_types=1);

namespace App\Data\Models;

use App\Enums\SessionStatus;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

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
        /** @var UserData|Lazy $host */
        #[AutoWhenLoadedLazy]
        public Lazy|UserData $host,
        /** @var QuizModeData|Lazy $quiz_mode */
        #[AutoWhenLoadedLazy('quizMode')]
        public Lazy|QuizModeData $quiz_mode,
        /** @var ScoringRuleData|Lazy $scoring_rule */
        #[AutoWhenLoadedLazy('scoringRule')]
        public Lazy|ScoringRuleData $scoring_rule,
        /** @var PlaylistData|null|Lazy $playlist */
        #[AutoWhenLoadedLazy]
        public Lazy|PlaylistData|null $playlist,
        /** @var Collection<array-key,SessionParticipantData>|Lazy $participants */
        #[AutoWhenLoadedLazy]
        public Collection|Lazy $participants,
        /** @var Collection<array-key,SessionRoundData>|Lazy $rounds */
        #[AutoWhenLoadedLazy]
        public Collection|Lazy $rounds,
        /** @var Collection<array-key,SessionEventData>|Lazy $events */
        #[AutoWhenLoadedLazy]
        public Collection|Lazy $events,
        /** @var Collection<array-key,SessionFinalScoreData>|Lazy $final_scores */
        #[AutoWhenLoadedLazy('finalScores')]
        public Collection|Lazy $final_scores,
    ) {}
}
