<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\QuizQuestionData;
use Illuminate\Pagination\LengthAwarePaginator;
use Spatie\LaravelData\Data;

class QuizQuestionsResponse extends Data
{
    public function __construct(
        /** @var LengthAwarePaginator<int, QuizQuestionData> $quiz_questions */
        public LengthAwarePaginator $quiz_questions,
    ) {}
}
