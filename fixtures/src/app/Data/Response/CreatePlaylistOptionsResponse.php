<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\MusicTrackData;
use App\Data\Models\QuizQuestionData;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

class CreatePlaylistOptionsResponse extends Data
{
    public function __construct(
        /** @var Collection<array-key,QuizQuestionData> $questions */
        public Collection $questions,
        /** @var Collection<array-key,MusicTrackData> $tracks */
        public Collection $tracks,
    ) {}
}
