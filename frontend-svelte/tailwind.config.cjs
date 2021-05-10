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
			'blue-300-400': [colors.blue['300'], colors.blue['400']],
			'blue-400-500': [colors.blue['400'], colors.blue['500']],
			'blue-500-600': [colors.blue['500'], colors.blue['600']],

			'orange-400-500': [colors.orange['400'], colors.orange['500']],
			'orange-500-600': [colors.orange['500'], colors.orange['600']],

			'violet-400-500': [colors.violet['400'], colors.violet['500']],
			'violet-500-600': [colors.violet['500'], colors.violet['600']],

			'yellow-400-500': [colors.yellow['400'], colors.yellow['500']],
			'yellow-500-600': [colors.yellow['500'], colors.yellow['600']],
		}),
		extend: {
			colors: {
				gray: colors.gray,
			},
		},
	},
	variants: {
		stripes: ['hover'],
		extend: {},
	},
	plugins: [
		require("./plugins/gradients-stripes.cjs"),
	],
}
