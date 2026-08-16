<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\QuizQuestionData;
use Spatie\LaravelData\Data;

class QuizQuestionResponse extends Data
{
    public function __construct(
        public QuizQuestionData $quiz_question,
    ) {}
}
