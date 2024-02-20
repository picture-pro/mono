module.exports = {
  content: {
    files: ["crates/site-app/src/**/*.rs"],
  },
  theme: {
    fontFamily: {
			'sans': ['inter', 'ui-sans-serif', 'system-ui', 'sans-serif', "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"],
    },
    screens: {
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
          // "fontFamily": "Chalkboard,comic sans ms,'sans-serif'",
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
          // "--rounded-box": "0.2rem",
          // "--rounded-btn": "0.2rem",
          // "--rounded-badge": "0.2rem",
          // "--tab-radius": "0.2rem",
        }
      },
      "light",
      "dark",
    ],
    prefix: "d-",
  },
};
