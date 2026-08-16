<?php

declare(strict_types=1);

namespace App\Data\Models;

use App\Enums\QuestionType;
use App\Enums\Visibility;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class QuizQuestionData extends Data
{
    public function __construct(
        public string $id,
        public ?string $user_id,
        public ?string $track_id,
        public QuestionType $question_type,
        public ?string $prompt_text,
        public string $correct_answer,
        public int $base_points,
        public ?int $media_start_seconds,
        public ?int $media_end_seconds,
        public int $difficulty_level,
        public Visibility $visibility,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var UserData|null|Lazy $user */
        #[AutoWhenLoadedLazy]
        public Lazy|UserData|null $user,
        /** @var MusicTrackData|null|Lazy $track */
        #[AutoWhenLoadedLazy]
        public Lazy|MusicTrackData|null $track,
        /** @var Collection<array-key,AnswerVariantData>|Lazy $answer_variants */
        #[AutoWhenLoadedLazy('answerVariants')]
        public Collection|Lazy $answer_variants,
        /** @var Collection<array-key,MultipleChoiceOptionData>|Lazy $multiple_choice_options */
        #[AutoWhenLoadedLazy('multipleChoiceOptions')]
        public Collection|Lazy $multiple_choice_options,
        /** @var Collection<array-key,SessionRoundData>|Lazy $session_rounds */
        #[AutoWhenLoadedLazy('sessionRounds')]
        public Collection|Lazy $session_rounds,
        /** @var Collection<array-key,PlaylistItemData>|Lazy $playlist_items */
        #[AutoWhenLoadedLazy('playlistItems')]
        public Collection|Lazy $playlist_items,
    ) {}
}
