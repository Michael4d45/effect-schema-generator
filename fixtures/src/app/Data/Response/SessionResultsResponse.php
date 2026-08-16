<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\SessionFinalScoreData;
use App\Data\Models\SessionRoundData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class SessionResultsResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,SessionFinalScoreData> $final_scores */
        public Collection $final_scores,
        /** @var Collection<array-key,SessionRoundData> $rounds */
        public Collection $rounds,
    ) {}
}
