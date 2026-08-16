<?php

declare(strict_types=1);

namespace App\Enums;

enum SessionStatus: string
{
    use EnumUtil;

    case Lobby = 'lobby';
    case InProgress = 'in_progress';
    case RoundTransition = 'round_transition';
    case Paused = 'paused';
    case Completed = 'completed';
}
