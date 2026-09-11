/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{svelte,js,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        pos: {
          bg: 'var(--pos-bg)',
          card: 'var(--pos-card)',
          border: 'var(--pos-border)',
          primary: 'var(--pos-primary)',
          accent: 'var(--pos-accent)',
          text: 'var(--pos-text)',
          muted: 'var(--pos-muted)',
          danger: 'var(--pos-danger)',
          warning: 'var(--pos-warning)',
          success: 'var(--pos-success)',
        }
      }
    },
  },
  plugins: [],
};
