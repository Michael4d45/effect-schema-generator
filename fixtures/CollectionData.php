<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Tests\Fixtures;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\DataCollection;
use Spatie\LaravelData\PaginatedDataCollection;
use Spatie\LaravelData\CursorPaginatedDataCollection;
use Spatie\LaravelData\Attributes\Lazy;
use Illuminate\Support\Collection;

/**
 * Data class testing various collection types
 */
class CollectionData extends Data
{
    public function __construct(
        // Standard Laravel Collection
        /** @var Collection<array-key, UserData> */
        public Collection $users,

        // Spatie DataCollection
        /** @var DataCollection<array-key, UserData> */
        public DataCollection $userDataCollection,

        // Paginated collections
        /** @var PaginatedDataCollection<array-key, UserData> */
        public PaginatedDataCollection $paginatedUsers,

        /** @var CursorPaginatedDataCollection<array-key, UserData> */
        public CursorPaginatedDataCollection $cursorPaginatedUsers,

        // Lazy collections
        #[Lazy]
        /** @var DataCollection<array-key, ProfileData> */
        public DataCollection $lazyProfiles,

        // Nested collections
        /** @var Collection<array-key, DataCollection<array-key, TaskData>> */
        public Collection $nestedCollections,

        // Optional collections
        /** @var ?DataCollection<array-key, UserData> */
        public ?DataCollection $optionalUsers,

        // Union with collections
        /** @var Collection<array-key, UserData>|DataCollection<array-key, UserData> */
        public $usersOrDataCollection,
    ) {}
}