<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

enum SessionStatus
{
    case WAITING;
    case ACTIVE;
    case FINISHED;
    case CANCELLED;
}