/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    screens: {
      'sm': '640px',
      'md': '768px',
      'lg': '1024px',
      'xl': '1280px',
      // '2xl': '1536px',
    },
    extend: {
      fontFamily: {
        sans: ["Roboto", "sans-serif"],
        serif: ["Aleo", "serif"],
      },
      colors: {
        // radix's `--color-surface`
        surface: "#ffffffd9",
        surfacedark: "#00000040",
      },
    },
  },
  safelist: [ ],
  plugins: [
    require('tailwindcss-radix-colors')({
      aliases: {
        slate: "base",
        indigo: "primary",
        red: "danger",
        grass: "success",
        amber: "warning",
      },
    }),
  ],
}


