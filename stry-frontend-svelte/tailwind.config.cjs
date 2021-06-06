const colors = require('tailwindcss/colors');

module.exports = {
	purge: [
		'./src/**/*.svelte',
		'./src/**/*.html',
	],
	darkMode: false,
	theme: {
		orange: colors.orange,
		violet: colors.violet,
		stripes: _ => ({
			'blue-300': [colors.blue['300'], 85],
			'blue-400': [colors.blue['400'], 85],
			'blue-500': [colors.blue['500'], 85],

			'indigo-300': [colors.indigo['300'], 80],
			'indigo-400': [colors.indigo['400'], 80],
			'indigo-500': [colors.indigo['500'], 80],

			'orange-400': [colors.orange['400'], 80],
			'orange-500': [colors.orange['500'], 80],

			'violet-400': [colors.violet['400'], 80],
			'violet-500': [colors.violet['500'], 80],

			'yellow-400': [colors.yellow['400'], 90],
			'yellow-500': [colors.yellow['500'], 90],
		}),
		extend: {
			colors: {
				gray: colors.gray,
				// https://breezezin.github.io/tailwind-color-palettes/
				'blue-6': {
					100: '#E3F2FD',
					200: '#BBDEFB',
					300: '#90CAF9',
					400: '#64B5F6',
					500: '#42A5F5',
					600: '#2196F3',
					700: '#1E88E5',
					800: '#1565C0',
					900: '#0D47A1'
				}
			},
		},
	},
	variants: {
		stripes: ['hover'],
		extend: {},
	},
	plugins: [
		require('./plugins/gradients-stripes.cjs'),
	],
}
