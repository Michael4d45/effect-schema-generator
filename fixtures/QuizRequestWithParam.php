<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

/**
 * @param Collection<array-key,QuizModeData> $quiz_modes
 */
class QuizRequestWithParam extends Data
{
    public function __construct(
        public Collection $quiz_modes,
    ) {}
}
