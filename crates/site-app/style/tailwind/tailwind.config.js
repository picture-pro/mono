module.exports = {
  content: {
    files: ["crates/site-app/src/**/*.rs"],
  },
  theme: {
    fontFamily: {
			'sans': ['inter', 'ui-sans-serif', 'system-ui', 'sans-serif', "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"],
    },
    screens: {
      'xs': '480px',
      'sm': '640px',
      'md': '768px',
      'lg': '1024px',
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        wireframe: {
          "color-scheme": "light",
          "primary": "#b8b8b8",
          "secondary": "#b8b8b8",
          "accent": "#b8b8b8",
          "neutral": "#ebebeb",
          "base-100": "oklch(100% 0 0)",
          "base-200": "#eeeeee",
          "base-300": "#dddddd",
          "base-content": "#282828",
          "info": "#0000ff",
          "success": "#008000",
          "warning": "#a6a659",
          "error": "#ff0000",
        },
        black: {
          "color-scheme": "dark",
          "primary": "#373737",
          "secondary": "#373737",
          "accent": "#373737",
          "base-100": "#000000",
          "base-200": "#141414",
          "base-300": "#262626",
          "base-content": "#d6d6d6",
          "neutral": "#373737",
          "info": "#0000ff",
          "success": "#008000",
          "warning": "#ffff00",
          "error": "#ff0000",
        },
      },
      "light",
      "dark",
    ],
    prefix: "d-",
  },
};
