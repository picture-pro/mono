/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    extend: {},
  },
  plugins: [
    require('tailwindcss-radix-colors')({
      aliases: {
        slate: "primary",
      },
    }),
  ],
}
