<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class AnswerVariantData extends Data
{
    public function __construct(
        public string $id,
        public string $question_id,
        public null|string $accepted_text,
        /** @var QuizQuestionData|Lazy $question */
        #[AutoWhenLoadedLazy]
        public Lazy|QuizQuestionData $question,
        /** @var Collection<array-key,PlayerAnswerData>|Lazy $player_answers */
        #[AutoWhenLoadedLazy('playerAnswers')]
        public Collection|Lazy $player_answers,
    ) {}
}
