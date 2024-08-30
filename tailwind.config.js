/** @type {import('tailwindcss').Config} */
export default {
  content: ["./templates/**/*.{html,js}"],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: ["business"],
    darkTheme: "business",
  },
  plugins: [require("daisyui")],
};
