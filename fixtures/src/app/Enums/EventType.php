<?php

declare(strict_types=1);

namespace App\Enums;

enum EventType: string
{
    use EnumUtil;

    case PlayerJoin = 'player_join';
    case PlayerLeave = 'player_leave';
    case AnswerSubmitted = 'answer_submitted';
    case BuzzIn = 'buzz_in';
    case TimerExpired = 'timer_expired';
    case HostOverride = 'host_override';
    case RoundStart = 'round_start';
    case RoundEnd = 'round_end';
}
