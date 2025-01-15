import franken from "franken-ui/shadcn-ui/preset-quick";
const { addIconSelectors } = require("@iconify/tailwind");

/** @type {import('tailwindcss').Config} */
export default {
  presets: [
    franken({
      customPalette: {
        ".uk-theme-raesan": {
          "--background": "15 2% 95%",
          "--foreground": "15 2% 1%",
          "--card": "15 2% 90%",
          "--card-foreground": "15 2% 10%",
          "--popover": "15 2% 95%",
          "--popover-foreground": "15 95% 1%",
          "--primary": "15 45% 75%",
          "--primary-foreground": "0 0% 0%",
          "--secondary": "15 10% 70%",
          "--secondary-foreground": "0 0% 0%",
          "--muted": "-23 10% 85%",
          "--muted-foreground": "15 2% 35%",
          "--accent": "-23 10% 80%",
          "--accent-foreground": "15 2% 10%",
          "--destructive": "0 50% 30%",
          "--destructive-foreground": "15 2% 90%",
          "--border": "15 20% 50%",
          "--input": "15 20% 18%",
          "--ring": "15 45% 75%",
          "--radius": "0.5rem",
        },
        ".dark.uk-theme-raesan": {
          "--background": "15 10% 5%",
          "--foreground": "15 2% 90%",
          "--card": "15 2% 1%",
          "--card-foreground": "15 2% 90%",
          "--popover": "15 10% 5%",
          "--popover-foreground": "15 2% 90%",
          "--primary": "15 45% 75%",
          "--primary-foreground": "0 0% 0%",
          "--secondary": "15 10% 10%",
          "--secondary-foreground": "0 0% 100%",
          "--muted": "-23 10% 15%",
          "--muted-foreground": "15 2% 60%",
          "--accent": "-23 10% 15%",
          "--accent-foreground": "15 2% 90%",
          "--destructive": "0 50% 30%",
          "--destructive-foreground": "15 2% 90%",
          "--border": "15 20% 18%",
          "--input": "15 20% 18%",
          "--ring": "15 45% 75%",
          "--radius": "0.5rem",
        },
      },
    }),
  ],
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
