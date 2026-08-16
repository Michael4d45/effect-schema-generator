<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\ScoringRuleData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class ScoringRulesResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,ScoringRuleData> $scoring_rules */
        public Collection $scoring_rules,
    ) {}
}