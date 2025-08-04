# @changelog.md

This file documents all significant changes made to the `project_arcade` application.

### 08-04-2025

- **Fix**: Resolved Tauri build error by correcting `distDir` configuration path in `tauri.conf.json`.
- **UI Enhancement**: Replaced cloud and star assets with high-resolution images.
  - Implemented new pastel-colored cloud system with two different cloud types (`src/lib/canvallax.ts`, `src/App.vue`)
  - Added static background cloud with proper window resizing support (`src/App.vue`)
  - Replaced particle stars with high-resolution star images and improved twinkling animation system (`src/App.vue`)

### 08-02-2025

- **Feature**: Implemented Tauri folder selection dialog in Settings view for choosing game installation directory. (`src/views/SettingsView.vue`)

### 08-01-2025

- **UI Polish**: Enhanced visual experience based on user feedback.
  - **Settings Page**: Reduced icon sizes, improved icons for Game Installation (download) and Metadata & Integration (document), removed unwanted hover effects. (`src/views/SettingsView.vue`)
  - **Recent Activity**: Now only displays when games are present in library, shows "No recent activity yet" when empty. (`src/views/HomeView.vue`)
  - **Cosmic Background**: Added slow-moving shooting stars and increased particle count from 50 to 80 for richer visual experience. (`src/App.vue`)
- **Further UI Refinements**: Additional polish based on detailed user feedback.
  - **Settings Icons**: Further reduced all icon sizes to proper proportions (header: w-4 h-4, sections: w-3 h-3), made header icon container smaller. (`src/views/SettingsView.vue`)
  - **Settings Hover Effects**: Completely eliminated all hover effects and movement from settings sections using !important overrides. (`src/views/SettingsView.vue`)
  - **Shooting Stars Direction**: Fixed shooting stars to travel from top-right to bottom-left with trails properly positioned behind them. (`src/App.vue`)
  - **Star Animations**: Enhanced stationary stars with random occasional twinkling (8s cycles) while keeping shooting stars as separate moving elements. (`src/App.vue`)
- **Final Polish**: Critical fixes for visual issues.
  - **Settings Icons**: Made icons properly small (header: w-3 h-3, sections: w-2.5 h-2.5), reduced header container to 1.5rem for text-proportional sizing. (`src/views/SettingsView.vue`)
  - **Shooting Star Trails**: Fixed trail positioning to actually appear behind stars, reduced trail size and opacity to eliminate "transparent wall" effect. (`src/App.vue`)
  - **Shooting Star Performance**: Reduced count from 5 to 3, increased delays (10-30s), shortened durations (1-3s), and constrained spawn area for better performance. (`src/App.vue`)
- **Bug Fix**: Fixed Vue compilation error caused by mixed `<script setup>` and regular `<script>` blocks in GameCard component. (`src/components/GameCard.vue`)

### 07-31-2025

- **UI Redesign**: Complete visual overhaul with glassmorphism and cosmic sunset theme inspired by Playnite.
  - **Global Styles**: Added comprehensive color palette with sunset/galactic colors (oranges, pinks, purples, blues) and glassmorphism utility classes. (`src/style.css`)
  - **Animated Background**: Implemented cosmic background with floating particles and gradient effects. (`src/App.vue`)
  - **Navigation**: Redesigned with sticky glass navigation bar, modern branding, and improved button styling. (`src/views/HomeView.vue`)
  - **Layout**: Added Playnite-inspired layout with sidebar for recent activity and library stats. (`src/views/HomeView.vue`)
  - **Game Cards**: Complete redesign with hover effects, status badges, glassmorphism styling, and quick action buttons. (`src/components/GameCard.vue`)
  - **Modal Design**: Modern modal with backdrop blur, animations, improved form styling, and better UX. (`src/components/AddGameModal.vue`)
  - **Visual Effects**: Added smooth transitions, hover animations, glassmorphism effects, and cosmic particle animations throughout.

### 07-30-2025

- **Project**: Initialized the complete project structure for `project_arcade`.
  - Created the Rust backend with Tauri, `sqlx` for SQLite database management, and modules for commands, configuration, and services.
  - Set up the Vue.js 3 frontend with TypeScript, Pinia for state management, Vue Router for navigation, and Tailwind CSS for styling.
  - Added initial database migration for the `games` table.
  - Created initial documentation with `changelog.md` and `learning.md`
