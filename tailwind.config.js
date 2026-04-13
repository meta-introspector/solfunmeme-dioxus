/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        'true-blue': '#dc143c',
        'blue-yonder': '#ff6b6b',
        'cobalt-blue': '#8b0000',
        'rich-black': '#0a0a0a'
      },
      boxShadow: {
        'strong-dark': '0 1px 5px rgba(220,20,60, 0.5)',
        'soft-dark': '0 1px 5px rgba(0, 0, 0, 0.6)',
      },
      zIndex: {
        '1000': '1000',
      },
      keyframes: {
        'fade-in': {
          '0%': { opacity: '0', transform: 'translateY(10px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
      },
      animation: {
        'fade-in': 'fade-in 0.5s ease-out forwards',
      },
    },
  },
  plugins: [],
}

