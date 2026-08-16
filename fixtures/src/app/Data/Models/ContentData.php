<?php

declare(strict_types=1);

namespace App\Data\Models;

use Spatie\LaravelData\Data;

class ContentData extends Data
{
    public function __construct(
        public int $id,
        public string $title,
        public string $body,
    ) {}
}
