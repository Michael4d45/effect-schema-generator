<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures\Enums;

enum RouteMarkerCollectionMode: string
{
    case Sequential = 'sequential';
    case Random = 'random';
}