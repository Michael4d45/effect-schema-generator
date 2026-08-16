<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

enum Priority: int
{
    case LOW = 1;
    case MEDIUM = 2;
    case HIGH = 3;
    case URGENT = 4;
}