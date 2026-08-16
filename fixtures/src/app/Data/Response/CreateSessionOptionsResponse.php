<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\PlaylistData;
use App\Data\Models\QuizModeData;
use App\Data\Models\ScoringRuleData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class CreateSessionOptionsResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,QuizModeData> $quiz_modes */
        public Collection $quiz_modes,
        /** @var Collection<array-key,ScoringRuleData> $scoring_rules */
        public Collection $scoring_rules,
        /** @var Collection<array-key,PlaylistData> $playlists */
        public Collection $playlists,
    ) {}
}
