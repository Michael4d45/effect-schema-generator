<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class QuizRequestWithVar extends Data
{
    public function __construct(
        /** @var Collection<array-key,QuizModeData> $quiz_modes */
        public Collection $quiz_modes,
    ) {}
}
