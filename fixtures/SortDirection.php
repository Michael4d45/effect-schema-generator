<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

enum SortDirection: string
{
    case ASC = 'asc';
    case DESC = 'desc';
}