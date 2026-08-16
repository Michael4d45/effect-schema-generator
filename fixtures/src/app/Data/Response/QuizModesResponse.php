<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\QuizModeData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class QuizModesResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,QuizModeData> $quiz_modes */
        public Collection $quiz_modes,
    ) {}
}
