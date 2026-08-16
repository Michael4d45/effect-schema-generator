<?php

declare(strict_types=1);

namespace App\Data\Requests;

use App\Enums\QuestionType;
use App\Enums\Visibility;
use Spatie\LaravelData\Attributes\Validation\Exists;
use Spatie\LaravelData\Attributes\Validation\IntegerType;
use Spatie\LaravelData\Attributes\Validation\Max;
use Spatie\LaravelData\Attributes\Validation\Min;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Attributes\Validation\StringType;
use Spatie\LaravelData\Data;

class CreateQuizQuestionRequest extends Data
{
    public function __construct(
        #[Required]
        public QuestionType $question_type,

        #[Required]
        #[StringType]
        #[Max(500)]
        public string $correct_answer,

        #[StringType]
        #[Exists('music_tracks', 'id')]
        public string|null $track_id = null,

        #[StringType]
        #[Max(1000)]
        public string|null $prompt_text = null,

        #[IntegerType]
        #[Min(1)]
        #[Max(10000)]
        public int $base_points = 1000,

        #[IntegerType]
        #[Min(0)]
        public int|null $media_start_seconds = null,

        #[IntegerType]
        #[Min(0)]
        public int|null $media_end_seconds = null,

        #[IntegerType]
        #[Min(1)]
        #[Max(5)]
        public int $difficulty_level = 1,

        public Visibility $visibility = Visibility::Public,
    ) {}
}
