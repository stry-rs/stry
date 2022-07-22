const colors = require('tailwindcss/colors');
const plugin = require('tailwindcss/plugin');

const percentToHex = (p) => {
  const percent = Math.max(0, Math.min(100, p))
  const intValue = Math.round((percent / 100) * 255)
  const hexValue = intValue.toString(16)
  return hexValue.padStart(2, '0').toUpperCase()
}

const map = (array, iteratee) => {
  let index = -1
  const length = array == null ? 0 : array.length
  const result = new Array(length)

  while (++index < length) {
    result[index] = iteratee(array[index], index, array)
  }

  return result
}

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "templates/**/*.html"
  ],
  theme: {
    extend: {},
  },
  plugins: [
    plugin(function ({ matchUtilities, theme }) {
      matchUtilities(
        {
          stripe: ([normal, opacity]) => {
            let darker = normal + percentToHex(opacity)

            return {
              backgroundImage: `linear-gradient(135deg, ${normal} 25%, ${darker} 25%, ${darker} 50%, ${normal} 50%, ${normal} 75%, ${darker} 75%, ${darker} 100%)`,
              backgroundSize: `28.28px 28.28px`,
            }
          },
        },
        { values: theme('stripes') }
      )
    }),
  ],
}
