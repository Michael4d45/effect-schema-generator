<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

/**
 * @param array<string> $question_ids
 */
class UpdatePlaylistRequestWithParam extends Data
{
    public function __construct(
        public array $question_ids = [],
    ) {}
}
