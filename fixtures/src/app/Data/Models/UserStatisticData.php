<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Carbon;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class UserStatisticData extends Data
{
    public function __construct(
        public string $id,
        public string $user_id,
        public int $total_games_played,
        public int $total_wins,
        public int $total_points,
        public ?float $average_score,
        public int $best_streak,
        public ?string $favorite_category_id,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var UserData|Lazy $user */
        #[AutoWhenLoadedLazy]
        public Lazy|UserData $user,
        /** @var CategoryData|null|Lazy $favorite_category */
        #[AutoWhenLoadedLazy('favoriteCategory')]
        public CategoryData|Lazy|null $favorite_category,
    ) {}
}
