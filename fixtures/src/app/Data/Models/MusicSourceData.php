<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class MusicSourceData extends Data
{
    public function __construct(
        public string $id,
        public string $name,
        public string $display_name,
        public ?string $icon_url,
        public ?string $api_base_url,
        public bool $requires_authentication,
        public bool $is_active,
        public int $priority,
        public ?Carbon $created_at,
        public ?Carbon $updated_at,
        /** @var Collection<array-key,SourceApiCredentialData>|Lazy $api_credentials */
        #[AutoWhenLoadedLazy('apiCredentials')]
        public Collection|Lazy $api_credentials,
        /** @var Collection<array-key,MusicTrackData>|Lazy $primary_tracks */
        #[AutoWhenLoadedLazy('primaryTracks')]
        public Collection|Lazy $primary_tracks,
        /** @var Collection<array-key,TrackSourceLinkData>|Lazy $track_source_links */
        #[AutoWhenLoadedLazy('trackSourceLinks')]
        public Collection|Lazy $track_source_links,
    ) {}
}
