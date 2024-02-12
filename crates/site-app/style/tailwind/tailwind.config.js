module.exports = {
  content: {
    files: ["crates/site-app/src/**/*.rs"],
  },
  theme: {
    fontFamily: {
			'sans': ['inter', 'ui-sans-serif', 'system-ui', 'sans-serif', "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"],
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: true,
    prefix: "d-",
  },
};
