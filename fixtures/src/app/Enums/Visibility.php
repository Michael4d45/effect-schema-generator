<?php

declare(strict_types=1);

namespace App\Enums;

enum Visibility: string
{
    use EnumUtil;

    case Public = 'public';
    case Private = 'private';
}
