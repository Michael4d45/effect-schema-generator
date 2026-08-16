<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\ContentData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class ContentItems extends Data
{
    public function __construct(
        /** @var Collection<array-key,ContentData> $content */
        public Collection $content,
    ) {}
}
