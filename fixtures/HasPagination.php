<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;

/**
 * @mixin Data
 */
trait HasPagination
{
    public null|int $per_page = 15;
    /** @var null|list<string> */
    public null|array $columns = ['*'];
    public null|string $page_name = 'page';
    public null|int $page = null;

    /** @var null|list<\App\Enums\SortDirection> */
    public null|array $sort_directions = null;
    /** @var null|list<string> */
    public null|array $sort_by = null;
}
