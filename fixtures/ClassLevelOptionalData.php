<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use EffectSchemaGenerator\Attributes\Optional;
use Spatie\LaravelData\Data;

#[Optional]
class ClassLevelOptionalData extends Data
{
    public string $name;

    public string $email;

    public ?string $nickname;
}