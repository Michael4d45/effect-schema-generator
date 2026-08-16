<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class PlaylistData extends Data
{
    public function __construct(
        public string $id,
        public string $user_id,
        public string $name,
        public ?string $description,
        public bool $is_public,
        public int $play_count,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var UserData|Lazy $user */
        #[AutoWhenLoadedLazy]
        public Lazy|UserData $user,
        /** @var Collection<array-key,PlaylistItemData>|Lazy $items */
        #[AutoWhenLoadedLazy]
        public Collection|Lazy $items,
        /** @var Collection<array-key,GameSessionData>|Lazy $game_sessions */
        #[AutoWhenLoadedLazy('gameSessions')]
        public Collection|Lazy $game_sessions,
    ) {}
}
