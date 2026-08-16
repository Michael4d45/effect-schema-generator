<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Attributes\Lazy;
use Carbon\Carbon;
use Illuminate\Support\Collection;

/**
 * Task data with enum usage
 */
class TaskData extends Data
{
    public function __construct(
        public string $id,
        public string $title,
        public string $description,
        public TestStatus $status,
        public Priority $priority,
        public ?Color $color,
        public TestStatus|Priority|null $statusOrPriority,
        public Carbon $createdAt,
        public ?Carbon $completedAt,

        /** @var Collection<array-key, string> */
        public Collection $tags,

        #[Lazy]
        public ?UserData $assignedUser,
    ) {}
}