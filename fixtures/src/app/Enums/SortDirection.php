<?php

declare(strict_types=1);

namespace App\Enums;

enum SortDirection: string
{
    use EnumUtil;

    case Asc = 'asc';
    case Desc = 'desc';
}
