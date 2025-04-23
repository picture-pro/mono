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
    transitionDuration: {
      'DEFAULT': '100ms',
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
      keyframes: {
        'fade-in': {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        'fade-out': {
          '0%': { opacity: '1' },
          '100%': { opacity: '0' },
        },
      },
      animation: {
        'fade-in': 'fade-in 0.3s ease-in-out',
        'fade-out': 'fade-out 0.3s ease-in-out',
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


