<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Attributes\Lazy;
use Carbon\Carbon;

class UserData extends Data
{
    public function __construct(
        public string $id,
        public string $name,
        public string $email,
        #[Lazy]
        public ?ProfileData $profile,
        public Carbon $createdAt,
    ) {}
}