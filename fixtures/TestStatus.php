<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

enum TestStatus: string
{
    case ACTIVE = 'active';
    case INACTIVE = 'inactive';
    case PENDING = 'pending';
    case SUSPENDED = 'suspended';
}