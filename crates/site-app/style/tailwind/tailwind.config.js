module.exports = {
  content: {
    files: ["crates/site-app/src/**/*.rs"],
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: true,
    prefix: "d-",
  },
};
