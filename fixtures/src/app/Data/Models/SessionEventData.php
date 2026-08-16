<?php

declare(strict_types=1);

namespace App\Data\Models;

use App\Enums\EventType;
use Illuminate\Support\Carbon;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class SessionEventData extends Data
{
    public function __construct(
        public string $id,
        public string $session_id,
        public ?EventType $event_type,
        public ?string $participant_id,
        /** @var array<string, mixed>|null $payload */
        public ?array $payload,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var GameSessionData|Lazy $session */
        #[AutoWhenLoadedLazy]
        public GameSessionData|Lazy $session,
        /** @var SessionParticipantData|null|Lazy $participant */
        #[AutoWhenLoadedLazy]
        public Lazy|SessionParticipantData|null $participant,
    ) {}
}
