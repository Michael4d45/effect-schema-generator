<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class IndexRouteEventsRequest extends Data
{
    use HasPagination;
    use HasVisibility;
}
