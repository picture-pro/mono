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
    themes: ["light", "dark", "wireframe"],
    prefix: "d-",
  },
};
