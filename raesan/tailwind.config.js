const { addIconSelectors } = require("@iconify/tailwind");

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./templates/**/*.{html,js}", "./static/**/*.{html,js}"],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: ["business"],
    darkTheme: "business",
  },
  plugins: [require("daisyui"), addIconSelectors(["mdi"])],
};
