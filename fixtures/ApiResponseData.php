<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Illuminate\Support\Collection;
use Spatie\LaravelData\Data;

/**
 * API response data with various complex types
 */
class ApiResponseData extends Data
{
    public function __construct(
        public bool $success,
        public string $message,
        public int $statusCode,

        // Simple arrays
        public array $errors,
        public array $warnings,

        // Typed arrays (via PHPDoc)
        /** @var array<string> */
        public array $tags,

        /** @var array<string, string> */
        public array $headers,

        /** @var array<string, mixed> */
        public array $metadata,

        // Collections with complex types
        /** @var Collection<array-key, UserData> */
        public Collection $users,

        /** @var Collection<array-key, TaskData> */
        public Collection $tasks,

        // Paginated-like structure
        /** @var array{current_page: int, last_page: int, per_page: int, total: int} */
        public array $pagination,

        // Nested data
        public ?UserData $currentUser,

        public ?ProfileData $userProfile,
    ) {}
}
