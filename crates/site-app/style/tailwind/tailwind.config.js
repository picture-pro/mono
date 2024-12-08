/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    extend: {},
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


