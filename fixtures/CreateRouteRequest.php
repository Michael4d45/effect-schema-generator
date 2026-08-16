<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class CreateRouteRequest extends Data
{
    public function __construct(
        public string $name,
        public null|string $description = null,
        public null|string $activity_type_id = null,
        public null|float $distance = null,
        public null|float $elevation_gain = null,
        public null|float $elevation_loss = null,
        public null|string $difficulty = null,
        public Enums\Visibility $visibility = Enums\Visibility::Public,
        public Enums\RouteMarkerCollectionMode $marker_collection_mode = Enums\RouteMarkerCollectionMode::Sequential,
        /** @var list<RouteWaypointRequest> */
        public array $waypoints = [],
    ) {}
}