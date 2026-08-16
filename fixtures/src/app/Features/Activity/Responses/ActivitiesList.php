<?php

declare(strict_types=1);

namespace App\Features\Activity\Responses;

use App\Data\Models\ActivityData;
use Illuminate\Pagination\LengthAwarePaginator;
use Spatie\LaravelData\Data;

class ActivitiesList extends Data
{
    public function __construct(
        /** @var LengthAwarePaginator<array-key,ActivityData> $activities */
        public LengthAwarePaginator $activities,
    ) {}
}
