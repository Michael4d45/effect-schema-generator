<?php

declare(strict_types=1);

namespace App\Data\Response;

use App\Data\Models\UserData;
use Spatie\LaravelData\Data;

class AuthResponse extends Data
{
    public function __construct(
        public string $token,
        public UserData $user,
    ) {}
}
