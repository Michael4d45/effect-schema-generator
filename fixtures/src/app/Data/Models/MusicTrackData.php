<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class MusicTrackData extends Data
{
    public function __construct(
        public string $id,
        public ?string $user_id,
        public string $title,
        public string $artist_name,
        public ?string $album_name,
        public ?int $release_year,
        public ?string $genre,
        public ?int $duration_ms,
        public string $sub_category_id,
        public string $primary_source_id,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var UserData|null|Lazy $user */
        #[AutoWhenLoadedLazy]
        public Lazy|UserData|null $user,
        /** @var SubCategoryData|Lazy $sub_category */
        #[AutoWhenLoadedLazy('subCategory')]
        public Lazy|SubCategoryData $sub_category,
        /** @var MusicSourceData|Lazy $primary_source */
        #[AutoWhenLoadedLazy('primarySource')]
        public Lazy|MusicSourceData $primary_source,
        /** @var Collection<array-key,TrackSourceLinkData>|Lazy $source_links */
        #[AutoWhenLoadedLazy('sourceLinks')]
        public Collection|Lazy $source_links,
        /** @var Collection<array-key,QuizQuestionData>|Lazy $quiz_questions */
        #[AutoWhenLoadedLazy('quizQuestions')]
        public Collection|Lazy $quiz_questions,
    ) {}
}
