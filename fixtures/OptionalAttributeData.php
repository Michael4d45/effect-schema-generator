<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Attributes\Optional;
use Spatie\LaravelData\Data;

#[Optional]
class OptionalAttributeData extends Data
{
    public string $requiredProperty;

    #[Optional]
    public string $optionalProperty;

    public string $anotherRequired;

    #[Optional]
    public ?string $nullableOptional;
}