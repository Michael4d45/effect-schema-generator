<?php

declare(strict_types=1);

namespace App\Enums;

trait EnumUtil
{
    /**
     * Example dumped type: list<'editor'|'viewer'>
     *
     * @return list<value-of<static>>
     */
    public static function getValues(): array
    {
        return array_column(static::cases(), 'value');
    }

    public static function random(): static
    {
        $cases = static::cases();

        return $cases[array_rand($cases)];
    }
}
