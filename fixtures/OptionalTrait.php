<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Attributes\Optional;

trait OptionalTrait
{
    #[Optional]
    public string $traitOptionalProperty;

    public string $traitRequiredProperty;
}