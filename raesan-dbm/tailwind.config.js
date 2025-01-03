import franken from "franken-ui/shadcn-ui/preset-quick";
const { addIconSelectors } = require("@iconify/tailwind");

/** @type {import('tailwindcss').Config} */
export default {
  presets: [franken()],
  content: ["./templates/**/*.{html,js}", "./static/**/*.{html,js}"],
  safelist: [
    {
      pattern: /^uk-/,
    },
    "ProseMirror",
    "ProseMirror-focused",
    "tiptap",
    "mr-2",
    "mt-2",
    "opacity-50",
  ],
  theme: {
    extend: {},
  },
  plugins: [addIconSelectors(["mdi"])],
};
