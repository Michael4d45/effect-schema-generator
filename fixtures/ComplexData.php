<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Attributes\Lazy;
use Spatie\LaravelData\Attributes\MapInputName;
use Spatie\LaravelData\Attributes\MapOutputName;
use Spatie\LaravelData\Attributes\WithCast;
use Carbon\Carbon;
use Illuminate\Support\Collection;

/**
 * Complex data class with various advanced features
 */
class ComplexData extends Data
{
    public function __construct(
        // Basic union types
        public int|string $identifier,
        public int|string|null $optionalIdentifier,

        // Complex union types
        public string|int|float|bool|null $mixedValue,

        // Collections with generics
        /** @var Collection<array-key, UserData> */
        public Collection $users,

        /** @var Collection<array-key, Lazy<UserData>> */
        #[Lazy]
        public Collection $lazyUsers,

        // Nested collections
        /** @var Collection<array-key, Collection<array-key, string>> */
        public Collection $nestedStrings,

        // Optional lazy properties
        #[Lazy]
        public ?ProfileData $optionalProfile,

        // Mapped properties
        #[MapInputName('first_name')]
        #[MapOutputName('givenName')]
        public string $firstName,

        // Date handling
        public Carbon $createdAt,
        public ?Carbon $updatedAt,

        // Complex nested data
        public AddressData $address,

        // Array types
        public array $tags,
        /** @var array<string, mixed> */
        public array $metadata,

        // Self-referencing (for testing circular reference handling)
        /** @var Collection<array-key, ComplexData> */
        public Collection $relatedItems,

        // With cast attribute
        #[WithCast('json')]
        public array $settings,
    ) {}
}