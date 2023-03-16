/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {
        colors: {
          // 'purple-cx': '#0A465B',
        },
      },
    },
    plugins: [],
}