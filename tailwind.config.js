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
          danger: '#ef4444',
          warning: '#f59e0b',
          success: '#10b981',
        }
      }
    },
  },
  plugins: [],
};
