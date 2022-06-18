const colors = require('tailwindcss/colors');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,ts,tsx}'],
  theme: {
    extend: {},
    colors: {
      ...colors,
      gray: {
        ...colors.gray,
        900: '#1E1E1E',
        800: '#212020',
        700: '#363636',
        600: '#4A4A4A',
        500: '#666666',
        400: '#959595',
      },
    },
  },
  plugins: [],
};
