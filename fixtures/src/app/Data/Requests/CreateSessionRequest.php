<?php

declare(strict_types=1);

namespace App\Data\Requests;

use Spatie\LaravelData\Attributes\Validation\Max;
use Spatie\LaravelData\Attributes\Validation\Min;
use Spatie\LaravelData\Attributes\Validation\Numeric;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Data;

class CreateSessionRequest extends Data
{
    public function __construct(
        #[Required]
        public string $quiz_mode_id,

        #[Required]
        public string $scoring_rule_id,

        public string|null $playlist_id = null,

        #[Required]
        #[Numeric]
        #[Min(1)]
        #[Max(50)]
        public int $max_players = 10,
    ) {}
}
