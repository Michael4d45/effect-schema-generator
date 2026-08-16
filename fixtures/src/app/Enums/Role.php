<?php

declare(strict_types=1);

namespace App\Enums;

    enum Role: string
{
    use EnumUtil;

    case Host = 'host';
    case Player = 'player';
    case Spectator = 'spectator';
}
