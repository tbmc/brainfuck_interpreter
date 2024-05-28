import { defineConfig, type UserConfig } from 'vitest/config';
import * as path from 'node:path';
import tsconfigPaths from 'vite-tsconfig-paths';

const sharedLibPath = path.resolve(
    __dirname,
    '..',
    'svelte',
    'src',
    'lib',
    'shared'
);

export default defineConfig(() => {
    return {
        resolve: {
            alias: {
                '#shared': path.resolve(__dirname, '../svelte/lib/shared'),
            },
        },
        plugins: [tsconfigPaths()],
    };
});
