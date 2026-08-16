<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class SessionEventOccurredData extends Data
{
    public function __construct(
        /** @var int|null|string|null */
        public $user_id,
        public \DateTime $timestamp,
    ) {}
}
