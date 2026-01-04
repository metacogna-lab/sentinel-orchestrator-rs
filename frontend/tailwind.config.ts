import type { Config } from 'tailwindcss'

export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Primary Colors (Rust-inspired, neo-punk aesthetic)
        'rust-orange': '#FF6B35',
        'dark-slate': '#1A1A2E',
        'deep-navy': '#16213E',
        'cyan-electric': '#00D9FF',
        'neon-green': '#39FF14',
        'warning-amber': '#FFB800',
        'error-red': '#FF1744',
        // Neutral Colors
        'pure-white': '#FFFFFF',
        'light-gray': '#E0E0E0',
        'medium-gray': '#9E9E9E',
        'dark-gray': '#424242',
        'charcoal': '#2A2A3E',
      },
      fontFamily: {
        // Monospace for technical feel
        'mono': ['JetBrains Mono', 'Fira Code', 'Consolas', 'monospace'],
        // Futuristic sans-serif for headings
        'display': ['Orbitron', 'Exo 2', 'Rajdhani', 'sans-serif'],
        // Clean sans-serif for body text
        'body': ['Inter', 'Roboto', 'system-ui', 'sans-serif'],
      },
      fontSize: {
        'display-1': ['4rem', { lineHeight: '1.2', fontWeight: '700' }],
        'display-2': ['3rem', { lineHeight: '1.2', fontWeight: '700' }],
        'h1': ['2.25rem', { lineHeight: '1.3', fontWeight: '600' }],
        'h2': ['1.875rem', { lineHeight: '1.4', fontWeight: '600' }],
        'h3': ['1.5rem', { lineHeight: '1.4', fontWeight: '600' }],
        'h4': ['1.25rem', { lineHeight: '1.5', fontWeight: '600' }],
        'body-lg': ['1.125rem', { lineHeight: '1.6', fontWeight: '400' }],
        'body': ['1rem', { lineHeight: '1.6', fontWeight: '400' }],
        'body-sm': ['0.875rem', { lineHeight: '1.5', fontWeight: '400' }],
        'caption': ['0.75rem', { lineHeight: '1.4', fontWeight: '400' }],
        'code': ['0.875rem', { lineHeight: '1.5', fontWeight: '400' }],
      },
      boxShadow: {
        'glow-rust': '0 0 20px rgba(255, 107, 53, 0.3)',
        'glow-cyan': '0 0 15px rgba(0, 217, 255, 0.4)',
        'glow-green': '0 0 15px rgba(57, 255, 20, 0.3)',
      },
      backgroundImage: {
        'gradient-primary': 'linear-gradient(135deg, #1A1A2E 0%, #16213E 100%)',
        'gradient-card': 'linear-gradient(180deg, rgba(22, 33, 62, 0.8) 0%, rgba(26, 26, 46, 0.6) 100%)',
      },
    },
  },
  plugins: [],
} satisfies Config

