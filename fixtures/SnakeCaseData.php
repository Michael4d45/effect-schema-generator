<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use EffectSchemaGenerator\Attributes\SnakeCase;
use Spatie\LaravelData\Data;

class SnakeCaseData extends Data
{
    public string $simpleProperty;

    #[SnakeCase]
    public string $firstName;

    public string $lastName;

    #[SnakeCase]
    public ?string $phoneNumber;
}
