import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { generateBrainFuckFiles } from './generateBrainFuckFilesPlugin';

export default defineConfig({
	plugins: [sveltekit(), generateBrainFuckFiles()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
