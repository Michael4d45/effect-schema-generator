<?php

declare(strict_types=1);

namespace EffectSchemaGenerator;

use EffectSchemaGenerator\Commands\GenerateSchemasCommand;
use Illuminate\Support\ServiceProvider;

class EffectSchemaGeneratorServiceProvider extends ServiceProvider
{
    public function register(): void
    {
        $this->mergeConfigFrom(__DIR__ . '/Config/config.php', 'effect-schema');
    }

    public function boot(): void
    {
        if ($this->app->runningInConsole()) {
            $this->commands([GenerateSchemasCommand::class]);
            $this->publishes([
                __DIR__ . '/Config/config.php' => config_path('effect-schema.php'),
            ], 'effect-schema-config');
        }
    }
}
