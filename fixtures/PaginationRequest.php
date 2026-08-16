<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

class PaginationRequest extends Data
{
    /** @var null|int */
    public null|int $per_page = 15;
    /** @var null|int */
    public null|int $page = 1;
    /** @var null|SortDirection */
    public null|SortDirection $sort_direction = null;
    /** @var null|string */
    public null|string $sort_by = null;
}