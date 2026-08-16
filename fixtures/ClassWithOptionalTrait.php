<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class ClassWithOptionalTrait extends Data
{
    use OptionalTrait;

    public string $classProperty;
}