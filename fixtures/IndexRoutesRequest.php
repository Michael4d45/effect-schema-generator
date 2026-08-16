<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

class IndexRoutesRequest extends PaginationRequest
{
    /** @var null|string */
    public null|string $search = null;
    /** @var bool */
    public bool $my_routes = false;
    /** @var null|Visibility */
    public null|Visibility $visibility = null;
    /** @var null|string */
    public null|string $activity_type_id = null;
    /** @var null|float */
    public null|float $min_distance = null;
    /** @var null|float */
    public null|float $max_distance = null;
    /** @var null|string */
    public null|string $difficulty = null;
}