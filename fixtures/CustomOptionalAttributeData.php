<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use EffectSchemaGenerator\Attributes\Optional;
use Spatie\LaravelData\Data;

class CustomOptionalAttributeData extends Data
{
    public string $requiredProperty;

    #[Optional]
    public string $optionalProperty;

    public string $anotherRequired;

    #[Optional]
    public ?string $nullableOptional;
}