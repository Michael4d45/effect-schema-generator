<?php

declare(strict_types=1);

namespace EffectSchemaGenerator\Commands;

use EffectSchemaGenerator\Support\BinaryResolver;
use EffectSchemaGenerator\Support\ConfigPayload;
use Illuminate\Console\Command;
use Symfony\Component\Process\Process;

class GenerateSchemasCommand extends Command
{
    protected $signature = 'effect-schema:transform
                            {--dry-run : Show what would be generated without writing files}';

    protected $description = 'Generate TypeScript interfaces and Effect schemas from PHP Spatie Data classes';

    public function handle(): int
    {
        $config = config()->array('effect-schema', []);
        $binary = BinaryResolver::resolve($config, base_path());
        $toml = base_path('effect-schema.toml');

        if (!is_file($binary)) {
            $this->error("effect-schema binary not found: {$binary}");
            $this->line('Install the native binary with `npm install -D @michael4d45/effect-schema-generator`,');
            $this->line('or set EFFECT_SCHEMA_BIN to the binary path.');

            return self::FAILURE;
        }

        $args = [$binary, 'transform'];
        if (is_file($toml)) {
            $args[] = '--config';
            $args[] = $toml;
        } else {
            $payload = ConfigPayload::fromLaravelConfig(
                $config,
                resource_path('js/schemas'),
            );
            $args[] = '--config-json';
            $args[] = json_encode($payload);
        }

        if ($this->option('dry-run')) {
            $args[] = '--dry-run';
        }

        $this->info('Generating TypeScript schemas from PHP classes...');
        $this->line('Using binary: ' . $binary);

        $process = new Process($args, base_path());
        $process->setTimeout(null);
        $process->run(function (string $type, string $buffer): void {
            $this->output->write($buffer);
        });

        if (!$process->isSuccessful()) {
            $this->error('effect-schema exited with status ' . $process->getExitCode());

            return self::FAILURE;
        }

        return self::SUCCESS;
    }
}
