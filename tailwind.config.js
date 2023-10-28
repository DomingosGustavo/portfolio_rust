/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "**/*.html"],
  },
  darkMode: "class", // 'media' or 'class'
  theme: {
    container: {
      center: true,
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },
    
    extend: {
      backgroundImage: {
        "grid-pattern": `linear-gradient(to right, rgba(128, 0, 128, 0.3) 0.5px, transparent 5px),
         linear-gradient(to bottom, rgba(128, 0, 128, 0.3) 0.5px, transparent 13px)`,
      },
      backgroundSize: {
        40: "50px 50px",
      },
     fontFamily: {
        "boogieeee": ["Boogie", ]
      },  
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
