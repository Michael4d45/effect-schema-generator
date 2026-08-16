<?php

declare(strict_types=1);

namespace App\Data\Requests;

use Spatie\LaravelData\Attributes\Validation\Max;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Attributes\Validation\StringType;
use Spatie\LaravelData\Data;

class CreatePlaylistRequest extends Data
{
    public function __construct(
        #[Required]
        #[StringType]
        #[Max(255)]
        public string $name,

        #[StringType]
        #[Max(1000)]
        public string|null $description = null,

        public bool $is_public = false,

        /** @var array<string> $question_ids */
        public array $question_ids = [],

        /** @var array<CreateQuestionRequest> $new_questions */
        public array $new_questions = [],
    ) {}
}
