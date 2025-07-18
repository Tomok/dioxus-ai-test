/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  darkMode: 'class', // Use 'class' to enable toggling via dark class
  theme: {
    extend: {
      colors: {
        // Light mode colors
        'background': '#ffffff',
        'text': '#333333',
        'grid': '#dddddd',
        'axis': '#888888',
        'tooltip-bg': '#333333',
        'tooltip-text': '#ffffff',
        
        // Dark mode colors will be automatically handled by Tailwind
      },
    },
  },
  plugins: [],
};
