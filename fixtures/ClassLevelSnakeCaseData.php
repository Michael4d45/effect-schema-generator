<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use EffectSchemaGenerator\Attributes\SnakeCase;
use Spatie\LaravelData\Data;

#[SnakeCase]
class ClassLevelSnakeCaseData extends Data
{
    public string $firstName;

    public string $lastName;

    public ?string $phoneNumber;
}
