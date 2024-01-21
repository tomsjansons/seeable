/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "**/*.html", "**/*.jinja"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
