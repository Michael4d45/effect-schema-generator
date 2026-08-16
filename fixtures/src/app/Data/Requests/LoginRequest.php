<?php

declare(strict_types=1);

namespace App\Data\Requests;

use Spatie\LaravelData\Attributes\Validation\Email;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Data;

class LoginRequest extends Data
{
    public function __construct(
        #[Required]
        #[Email]
        public string $email,
        #[Required]
        public string $password,
        public bool $remember = false,
    ) {}
}
