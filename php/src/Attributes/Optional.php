<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Attributes;

use Attribute;

#[Attribute(Attribute::TARGET_PROPERTY | Attribute::TARGET_CLASS)]
class Optional
{
    public function __construct() {}
}
