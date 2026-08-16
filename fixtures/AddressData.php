<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

/**
 * Address data with various field types
 */
class AddressData extends Data
{
    public function __construct(
        public string $street,
        public string $city,
        public string $state,
        public string $zipCode,
        public string $country,
        public ?string $apartment,
        public float $latitude,
        public float $longitude,
        public bool $isPrimary,
    ) {}
}