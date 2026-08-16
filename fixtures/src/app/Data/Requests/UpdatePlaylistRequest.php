<?php

declare(strict_types=1);

namespace App\Data\Requests;

use App\Data\Models\UserData;
use Spatie\LaravelData\Attributes\Validation\Max;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Attributes\Validation\StringType;
use Spatie\LaravelData\Data;

class UpdatePlaylistRequest extends Data
{
    public function __construct(
        #[Required]
        #[StringType]
        #[Max(255)]
        public string $name,

        #[StringType]
        #[Max(1000)]
        public ?string $description,

        public bool $is_public,

        /** @var array<string> $question_ids */
        public array $question_ids,

        public UserData $user,
    ) {}
}
