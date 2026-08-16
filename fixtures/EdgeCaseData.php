<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Attributes\Lazy;

/**
 * Data class with edge cases and potential problem types
 */
class EdgeCaseData extends Data
{
    public function __construct(
        // Mixed type (should be unknown)
        public mixed $anything,

        // Complex union with many types
        public string|int|float|bool|null|TestStatus $complexUnion,

        // Nested nullable unions
        public string|int|null $nullableUnion,

        // Deeply nested collections
        /** @var Collection<array-key, Collection<array-key, Collection<array-key, string>>> */
        public \Illuminate\Support\Collection $deeplyNested,

        // Lazy with complex type
        #[Lazy]
        /** @var Collection<array-key, ?UserData> */
        public \Illuminate\Support\Collection $lazyNullableUsers,

        // Empty array type
        public array $emptyArray,

        // Array with mixed types
        /** @var array<mixed> */
        public array $mixedArray,

        public mixed $mixedProperty,

        /** @var array<mixed> */
        public mixed $mixedPropertyButArray,

        // PHPStan array shapes
        /** @var array{key: string, value: int} */
        public array $arrayShape,

        /** @var array{name: string, age: int, active?: bool} */
        public array $arrayShapeWithOptional,

        /** @var array{0: string, 1: int, 2: bool} */
        public array $arrayShapeWithNumericKeys,

        // PHPStan list types
        /** @var list<string> */
        public array $listType,

        /** @var list<int|string> */
        public array $listWithUnion,

        // PHPStan non-empty-array
        /** @var non-empty-array<string> */
        public array $nonEmptyArray,

        // PHPStan iterable
        /** @var iterable<string> */
        public iterable $iterableType,

        /** @var iterable<int, UserData> */
        public iterable $iterableWithKeyValue,

        // PHPStan callable (in PHPDoc only)
        /** @var callable(string, int): bool */
        public mixed $callableType,

        // PHPStan class-string
        /** @var class-string<UserData> */
        public string $classString,

        /** @var class-string */
        public string $classStringGeneric,

        // PHPStan literal types
        /** @var 'active'|'inactive'|'pending' */
        public string $literalStringUnion,

        /** @var 1|2|3|4|5 */
        public int $literalIntUnion,

        /** @var true|false */
        public bool $literalBoolUnion,

        // PHPStan closure
        /** @var \Closure(string, int): bool */
        public mixed $closureType,

        // PHPStan array with spread
        /** @var array{...array<string>, extra: int} */
        public array $arrayWithSpread,

        // PHPStan const types
        /** @var 'constant-value' */
        public string $constStringType,

        // PHPStan intersection types (PHPDoc only, not valid PHP syntax)
        /** @var UserData&ProfileData */
        public mixed $intersectionType,

        // PHPStan key-of and value-of (if supported)
        /** @var array{key1: string, key2: int} */
        public array $complexArrayShape,
    ) {}
}