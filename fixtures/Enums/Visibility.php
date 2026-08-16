<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures\Enums;

enum Visibility: string
{
    case Public = 'public';
    case Private = 'private';
}