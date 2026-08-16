<?php

declare(strict_types=1);

namespace App\Data\Models;

use Spatie\LaravelData\Data;

class ActivityData extends Data
{
    public function __construct(
        public string $id,
        public ?string $name,
    ) {}
}
