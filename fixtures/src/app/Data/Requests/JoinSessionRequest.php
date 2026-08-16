<?php

declare(strict_types=1);

namespace App\Data\Requests;

use App\Models\GameSession;
use Spatie\LaravelData\Attributes\Validation\Exists;
use Spatie\LaravelData\Attributes\Validation\Max;
use Spatie\LaravelData\Attributes\Validation\Nullable;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Attributes\Validation\StringType;
use Spatie\LaravelData\Data;

class JoinSessionRequest extends Data
{
    public function __construct(
        #[Required]
        #[StringType]
        #[Max(6)]
        #[Exists(GameSession::class, 'room_code')]
        public string $room_code,

        #[StringType]
        #[Max(255)]
        #[Nullable]
        public string|null $guest_name,
    ) {}

    public function gameSession(): GameSession
    {
        return GameSession::where(
            'room_code',
            $this->room_code,
        )->firstOrCreate();
    }
}
