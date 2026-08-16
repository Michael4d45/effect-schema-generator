<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class SessionRoundData extends Data
{
    public function __construct(
        public string $id,
        public string $session_id,
        public int $round_number,
        public string $question_id,
        public ?Carbon $started_at,
        public ?Carbon $ended_at,
        public ?string $first_buzzer_id,
        /** @var GameSessionData|Lazy $session */
        #[AutoWhenLoadedLazy]
        public GameSessionData|Lazy $session,
        /** @var QuizQuestionData|Lazy $question */
        #[AutoWhenLoadedLazy]
        public Lazy|QuizQuestionData $question,
        /** @var SessionParticipantData|null|Lazy $first_buzzer */
        #[AutoWhenLoadedLazy('firstBuzzer')]
        public Lazy|SessionParticipantData|null $first_buzzer,
        /** @var Collection<array-key,PlayerAnswerData>|Lazy $answers */
        #[AutoWhenLoadedLazy]
        public Collection|Lazy $answers,
    ) {}
}
