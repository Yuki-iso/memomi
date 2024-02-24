import defaultTheme from 'tailwindcss/defaultTheme';
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        jomhuria: ['Jomhuria', ...defaultTheme.fontFamily.sans],
      },
    },
  },
  plugins: [],
}

