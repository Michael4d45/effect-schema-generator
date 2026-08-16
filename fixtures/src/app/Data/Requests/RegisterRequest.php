<?php

declare(strict_types=1);

namespace App\Data\Requests;

use Spatie\LaravelData\Attributes\Validation\Confirmed;
use Spatie\LaravelData\Attributes\Validation\Email;
use Spatie\LaravelData\Attributes\Validation\Max;
use Spatie\LaravelData\Attributes\Validation\Min;
use Spatie\LaravelData\Attributes\Validation\Required;
use Spatie\LaravelData\Attributes\Validation\StringType;
use Spatie\LaravelData\Attributes\Validation\Unique;
use Spatie\LaravelData\Data;

class RegisterRequest extends Data
{
    public function __construct(
        #[Required]
        #[StringType]
        #[Min(2)]
        #[Max(255)]
        public string $name,

        #[Required]
        #[Email]
        #[Max(255)]
        #[Unique(
            table: 'users',
            column: 'email',
        )]
        public string $email,

        #[Required]
        #[StringType]
        #[Min(8)]
        #[Confirmed]
        public string $password,

        #[Required]
        #[StringType]
        public string $password_confirmation,
    ) {}
}
