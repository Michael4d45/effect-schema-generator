<?php

declare(strict_types=1);

namespace App\Data\Models;

use App\Enums\CredentialType;
use Illuminate\Support\Carbon;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class SourceApiCredentialData extends Data
{
    public function __construct(
        public string $id,
        public string $source_id,
        public ?CredentialType $credential_type,
        public string $encrypted_value,
        public ?Carbon $expires_at,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var MusicSourceData|Lazy $source */
        #[AutoWhenLoadedLazy]
        public Lazy|MusicSourceData $source,
    ) {}
}
