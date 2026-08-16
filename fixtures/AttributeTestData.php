<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

/**
 * Custom attribute for testing different argument types
 */
#[\Attribute]
class TestAttribute
{
    public function __construct(
        public mixed $value,
    ) {}
}

/**
 * Data class for testing attribute argument formatting
 */
class AttributeTestData extends Data
{
    public function __construct(
        #[TestAttribute('string_value')]
        public string $stringArg,

        #[TestAttribute(42)]
        public int $intArg,

        #[TestAttribute(3.14)]
        public float $floatArg,

        #[TestAttribute(true)]
        public bool $trueArg,

        #[TestAttribute(false)]
        public bool $falseArg,

        #[TestAttribute(null)]
        public ?string $nullArg,

        #[TestAttribute(['a', 'b', 'c'])]
        public array $arrayArg,

        #[TestAttribute(['key1' => 'value1', 'key2' => 123])]
        public array $assocArrayArg,

        #[TestAttribute(\EffectSchemaGenerator\Tests\Fixtures\UserData::class)]
        public string $classArg,
    ) {}
}
