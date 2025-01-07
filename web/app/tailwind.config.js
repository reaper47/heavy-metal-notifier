/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "../../src/web/templates/*.rs",
        "../../src/model/*.rs",
    ],
    theme: {
      extend: {},
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
    daisyui: {
        themes: ["light", "forest"],
        darkTheme: "forest",
    }
  }
