/** @type {import('tailwindcss').Config} */
// Most styling lives in src/styles.css as custom CSS; Tailwind is only used
// for a few utility classes in the loading spinner.
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  plugins: [],
}
