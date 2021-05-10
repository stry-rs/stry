const colors = require("tailwindcss/colors");

module.exports = {
	purge: [
		'./src/**/*.svelte',
		'./src/**/*.html',
	],
	darkMode: false,
	theme: {
		extend: {
			colors: {
				gray: colors.gray,
			},
		},
	},
	variants: {
		extend: {},
	},
	plugins: [],
}
