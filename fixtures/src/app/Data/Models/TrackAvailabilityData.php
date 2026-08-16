<?php

declare(strict_types=1);

namespace App\Data\Models;

use Illuminate\Support\Carbon;
use Spatie\LaravelData\Attributes\AutoWhenLoadedLazy;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Lazy;

class TrackAvailabilityData extends Data
{
    public function __construct(
        public string $id,
        public string $track_source_link_id,
        public ?string $country_code,
        public bool $is_available,
        public ?Carbon $last_checked_at,
        /** @var TrackSourceLinkData|Lazy $track_source_link */
        #[AutoWhenLoadedLazy('trackSourceLink')]
        public Lazy|TrackSourceLinkData $track_source_link,
    ) {}
}
