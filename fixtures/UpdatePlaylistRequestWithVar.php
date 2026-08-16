<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class UpdatePlaylistRequestWithVar extends Data
{
    public function __construct(
        /** @var array<string> $question_ids */
        public array $question_ids = [],
    ) {}
}
