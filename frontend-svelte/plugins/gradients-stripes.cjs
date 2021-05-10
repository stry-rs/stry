const _ = require('lodash');
const plugin = require('tailwindcss/plugin');

module.exports = plugin(function({ addUtilities, e, theme, variants }) {
    const stripes = theme('stripes', {});
    const stripesVariants = variants('stripes', []);

    const utilities = _.map(stripes, ([normal, dark], name) => ({
        [`.${e(`gradient-stripes-${name}`)}`]: {
            backgroundImage: `linear-gradient(135deg, ${normal} 25%, ${dark} 25%, ${dark} 50%, ${normal} 50%, ${normal} 75%, ${dark} 75%, ${dark} 100%)`,
            backgroundSize: `28.28px 28.28px`,
        }
    }));

    addUtilities(utilities, stripesVariants);
});
