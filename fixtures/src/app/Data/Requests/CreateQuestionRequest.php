<?php

declare(strict_types=1);

namespace App\Data\Requests;

use App\Enums\QuestionType;
use Spatie\LaravelData\Attributes\Validation\Max;
use Spatie\LaravelData\Attributes\Validation\Min;
use Spatie\LaravelData\Attributes\Validation\Numeric;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Attributes\Validation\StringType;
use Spatie\LaravelData\Data;

class CreateQuestionRequest extends Data
{
    public function __construct(
        #[Required]
        public string $track_id,

        #[Required]
        public QuestionType $question_type,

        #[Required]
        #[StringType]
        #[Max(255)]
        public string $correct_answer,

        #[StringType]
        #[Max(1000)]
        public string|null $prompt_text = null,

        #[Required]
        #[Numeric]
        #[Min(1)]
        public int $base_points = 10,

        #[Numeric]
        #[Min(0)]
        public int|null $media_start_seconds = null,

        #[Numeric]
        #[Min(0)]
        public int|null $media_end_seconds = null,

        #[Required]
        #[Numeric]
        #[Min(1)]
        #[Max(5)]
        public int $difficulty_level = 1,
    ) {}
}
