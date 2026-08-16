<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class RouteWaypointRequest extends Data
{
    public function __construct(
        public string $id,
        public string $name,
    ) {}
}