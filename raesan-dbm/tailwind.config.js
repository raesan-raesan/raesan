const { addIconSelectors } = require("@iconify/tailwind");

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./templates/**/*.{html,js}", "./static/create_test.js"],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: ["business"],
    darkTheme: "business",
  },
  plugins: [require("daisyui"), addIconSelectors(["mdi"])],
};
