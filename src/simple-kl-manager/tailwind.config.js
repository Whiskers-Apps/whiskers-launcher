/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx,vue}",
  ],
  theme: {
    extend: {
      colors:{
        background: "#24273a",
        text: "#cad3f5",
        accent: "#8aadf4",
        "on-accent": "#181926",
        danger: "#ee99a0",
        "on-danger": "#181926"
      }
    },
  },
  plugins: [],
}

