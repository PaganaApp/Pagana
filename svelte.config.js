import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [
		preprocess()
	],

	kit: {
		ssr: false,
		adapter: adapter({
			pages: 'src',
			assets: 'src',
			fallback: 'index.html'
		}),
		files: {
			template: 'src/index.html'
		},
	},
};

export default config;
