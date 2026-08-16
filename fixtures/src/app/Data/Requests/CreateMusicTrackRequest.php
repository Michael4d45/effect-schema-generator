<?php

declare(strict_types=1);

namespace App\Data\Requests;

use Spatie\LaravelData\Attributes\Validation\Exists;
use Spatie\LaravelData\Attributes\Validation\IntegerType;
use Spatie\LaravelData\Attributes\Validation\Max;
use Spatie\LaravelData\Attributes\Validation\Min;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Attributes\Validation\StringType;
use Spatie\LaravelData\Data;

class CreateMusicTrackRequest extends Data
{
    public function __construct(
        #[Required]
        #[StringType]
        #[Max(255)]
        public string $title,

        #[Required]
        #[StringType]
        #[Max(255)]
        public string $artist_name,

        #[Required]
        #[StringType]
        #[Exists('sub_categories', 'id')]
        public string $sub_category_id,

        #[Required]
        #[StringType]
        #[Exists('music_sources', 'id')]
        public string $primary_source_id,

        #[StringType]
        #[Max(255)]
        public string|null $album_name = null,

        #[IntegerType]
        public int|null $release_year = null,

        #[StringType]
        #[Max(100)]
        public string|null $genre = null,

        #[IntegerType]
        #[Min(1)]
        public int|null $duration_ms = null,
    ) {}
}
