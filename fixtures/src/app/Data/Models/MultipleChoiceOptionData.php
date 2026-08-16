<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class MultipleChoiceOptionData extends Data
{
    public function __construct(
        public string $id,
        public string $question_id,
        public string $option_text,
        public bool $is_correct,
        public ?int $sort_order,
        /** @var QuizQuestionData|Lazy $question */
        #[AutoWhenLoadedLazy]
        public Lazy|QuizQuestionData $question,
        /** @var Collection<array-key,PlayerAnswerData>|Lazy $player_answers */
        #[AutoWhenLoadedLazy('playerAnswers')]
        public Collection|Lazy $player_answers,
    ) {}
}
