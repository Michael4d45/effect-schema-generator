<?php

declare(strict_types=1);

namespace App\Data\Requests;

use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Attributes\Validation\StringType;
use Spatie\LaravelData\Data;

class AuthenticateBroadcastingRequest extends Data
{
    public function __construct(
        #[Required]
        #[StringType]
        public string $socket_id,
        #[Required]
        #[StringType]
        public string $channel_name,
    ) {}
}
