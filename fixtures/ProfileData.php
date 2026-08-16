<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class ProfileData extends Data
{
    public function __construct(
        public string $bio,
        public ?string $avatar,
        public array $preferences,
    ) {}
}