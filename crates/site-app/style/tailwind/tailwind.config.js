/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Roboto", "sans-serif"],
        serif: ["Aleo", "serif"],
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


