<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useLibraryStore } from '@/stores/library';
import GameCard from '@/components/GameCard.vue';
import AddGameModal from '@/components/AddGameModal.vue';
import { RouterLink } from 'vue-router';

const libraryStore = useLibraryStore();
const showAddModal = ref(false);

onMounted(() => {
  libraryStore.fetchGames();
});

// Mock recent activity data (only show when games exist)
const recentActivity = ref<Array<{ title: string; playtime: string; action: string }>>([]);
</script>

<template>
  <div class="home-container">
    <!-- Top Navigation Bar -->
    <nav class="top-nav glass-strong">
      <div class="nav-content">
        <div class="nav-left">
          <h1 class="app-title">
            <span class="title-main">Project Arcade</span>
            <span class="title-sub">A gift to my gaming goddess</span>
          </h1>
        </div>
        
        <div class="nav-right">
          <RouterLink to="/settings" class="btn-secondary nav-btn">
            <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
            </svg>
            Settings
          </RouterLink>
        </div>
      </div>
    </nav>

    <!-- Main Content Area -->
    <main class="main-area">
      <!-- Side Panel for Library Stats -->
      <aside v-if="libraryStore.games.length > 0" class="side-panel glass">
        <div class="stats-section">
          <h4 class="stats-title">Library Stats</h4>
          <div class="stats-grid">
            <div class="stat-item">
              <span class="stat-number">{{ libraryStore.games.length }}</span>
              <span class="stat-label">Games</span>
            </div>
          </div>
        </div>
        
        <!-- Olivia Image -->
        <div class="olivia-image-container">
          <img src="/src/assets/images/olivia1.png" alt="Olivia" class="olivia-image">
        </div>
      </aside>

      <!-- Game Library Grid -->
      <section class="library-section">
        <!-- Library Header -->
        <div class="library-header">
          <div class="welcome-background glass">
            <h2 class="library-title">
              <span class="welcome-part">Welcome back,</span>
              <span class="olivia-part"> Olivia.</span>
            </h2>
          </div>
        </div>

        <!-- Loading State -->
        <div v-if="libraryStore.isLoading" class="loading-state">
          <div class="loading-spinner"></div>
          <p>Loading your games...</p>
        </div>

        <!-- Empty State -->
        <div v-else-if="libraryStore.games.length === 0" class="empty-state glass-card">
          <div class="empty-icon">
            <svg class="w-16 h-16" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/>
            </svg>
          </div>
          <h3>Your library is empty</h3>
          <p>Add your first game to get started with Project Arcade</p>
          <button @click="showAddModal = true" class="btn-primary">
            Add Your First Game
          </button>
        </div>

        <!-- Games Grid -->
        <div v-else class="games-grid">
          <GameCard v-for="game in libraryStore.games" :key="game.id" :game="game" />
        </div>
      </section>
    </main>

    <!-- Add Game Modal -->
    <AddGameModal v-if="showAddModal" @close="showAddModal = false" />
  </div>
</template>

<style scoped>
.home-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

/* Top Navigation */
.top-nav {
  position: sticky;
  top: 0;
  z-index: 100;
  padding: 1rem 2rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.nav-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  max-width: 1400px;
  margin: 0 auto;
}

.nav-left .app-title {
  display: flex;
  flex-direction: column;
}

.title-main {
  font-size: 1.5rem;
  font-weight: 700;
  background: var(--gradient-sunset);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.title-sub {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin-top: -2px;
}

.nav-right {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.nav-btn {
  display: flex;
  align-items: center;
  font-size: 0.875rem;
}

/* Main Content Area */
.main-area {
  flex: 1;
  display: grid;
  grid-template-columns: 300px 1fr;
  gap: 2rem;
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

/* Side Panel */
.side-panel {
  padding: 1.5rem;
  border-radius: 16px;
  height: fit-content;
  position: sticky;
  top: 100px;
}

.panel-title {
  font-size: 1.125rem;
  font-weight: 600;
  margin-bottom: 1rem;
  color: var(--text-primary);
}

.activity-list {
  margin-bottom: 2rem;
}

.activity-item {
  padding: 0.75rem 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.activity-item:last-child {
  border-bottom: none;
}

.activity-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.activity-game {
  font-weight: 500;
  color: var(--text-primary);
}

.activity-meta {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.no-activity {
  padding: 1rem;
  text-align: center;
}

.no-activity .text-muted {
  color: var(--text-muted);
  font-style: italic;
}

.stats-section {
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  padding-top: 1.5rem;
}

.stats-title {
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 1rem;
  color: var(--text-primary);
}

.stats-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.stat-item {
  text-align: center;
}

.stat-number {
  display: block;
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
}

.stat-label {
  display: block;
  font-size: 0.75rem;
  color: var(--text-muted);
}

/* Olivia Image */
.olivia-image-container {
  display: flex;
  justify-content: center;
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.olivia-image {
  width: 80%;
  height: auto;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

/* Library Section */
.library-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: calc(100vh - 140px); /* Increased space to prevent cutoff */
}

.library-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  flex-shrink: 0; /* Prevent header from shrinking */
}

.library-title {
  font-size: 2rem;
  font-weight: 700;
  margin: 0;
}

.welcome-part {
  background: linear-gradient(135deg, #003366 0%, #6a0572 50%, #ff6b35 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.olivia-part {
  background: linear-gradient(135deg, #003366 0%, #6a0572 50%, #ff6b35 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.welcome-background {
  padding: 1rem 1.5rem;
  border-radius: 16px;
  margin-bottom: 1rem;
  width: 780px; /* Match the games grid width exactly */
  margin-left: none;
  margin-right: none;
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
}

/* Games Grid */
.games-grid {
  display: grid;
  grid-template-columns: repeat(5, 140px);
  gap: 1rem;
  justify-content: center;
  align-content: start;
  overflow-y: auto;
  overflow-x: hidden;
  flex: 1;
  padding-bottom: 2rem; /* Only bottom padding for scrolling */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
  width: fit-content; /* Only as wide as needed */
}

.games-grid::-webkit-scrollbar {
  display: none; /* Chrome, Safari, Opera */
}

/* Loading State */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem;
  color: var(--text-secondary);
}

.loading-spinner {
  width: 3rem;
  height: 3rem;
  border: 3px solid rgba(255, 255, 255, 0.3);
  border-top: 3px solid var(--sunset-pink);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* Empty State */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 4rem 2rem;
  color: var(--text-secondary);
  background: var(--glass-white);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  transition: none !important;
  transform: none !important;
}

.empty-state:hover {
  transform: none !important;
  background: var(--glass-white) !important;
  border-color: rgba(255, 255, 255, 0.1) !important;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2) !important;
}

.empty-icon {
  margin-bottom: 1.5rem;
  opacity: 0.6;
}

.empty-state h3 {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  color: var(--text-primary);
}

.empty-state p {
  margin-bottom: 2rem;
  font-size: 1rem;
}

/* Responsive Design */
@media (max-width: 1024px) {
  .main-area {
    grid-template-columns: 1fr;
    gap: 1rem;
    padding: 1rem;
  }
  
  .side-panel {
    position: static;
    order: 2;
  }
  
  .library-section {
    order: 1;
  }
}

@media (max-width: 768px) {
  .nav-content {
    flex-direction: column;
    gap: 1rem;
  }
  
  .nav-right {
    flex-direction: column;
    width: 100%;
  }
  
  .nav-btn {
    width: 100%;
    justify-content: center;
  }
  
  .games-grid {
    grid-template-columns: repeat(auto-fill, 150px);
    gap: 1rem;
    justify-content: center;
  }
}

/* Large screen adjustments for better alignment */
@media (min-width: 1200px) {
  .games-grid {
    gap: 1.25rem;
    justify-content: start;
  }
  
  .library-header {
    margin-bottom: 1.5rem;
  }
}
</style>
