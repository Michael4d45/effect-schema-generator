<?php

declare(strict_types=1);

namespace App\Data\Requests;

use Spatie\LaravelData\Attributes\Validation\Confirmed;
use Spatie\LaravelData\Attributes\Validation\CurrentPassword;
use Spatie\LaravelData\Attributes\Validation\Password;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Data;

class UpdatePasswordRequest extends Data
{
    public function __construct(
        #[Required]
        #[CurrentPassword]
        public string $current_password,

        #[Required]
        #[Confirmed]
        #[Password]
        public string $password,
    ) {}
}
