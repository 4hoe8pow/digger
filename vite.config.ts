import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	// for Docker
	server: {
		host: true,
		watch: {
			usePolling: true,
			interval: 2000
		}
	}
});
