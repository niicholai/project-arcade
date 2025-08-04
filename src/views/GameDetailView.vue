<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import * as api from '@/services/api';
import type { Game } from '@/types';
import { listen, Event } from '@tauri-apps/api/event';

interface InstallProgress {
    id: number;
    progress: number;
    status: string;
}

const route = useRoute();
const router = useRouter();
const gameId = computed(() => Number(route.params.id));

const game = ref<Game | null>(null);
const isLoading = ref(true);
const installStatus = ref('');
const installProgress = ref(0);

let unlistenMetadata: (() => void) | null = null;
let unlistenInstall: (() => void) | null = null;

const fetchDetails = async () => {
    isLoading.value = true;
    installStatus.value = '';
    try {
        console.log(`Fetching details for game ${gameId.value}...`);
        game.value = await api.getGameDetails(gameId.value);
    } catch(e) {
        console.error("Failed to fetch game details", e);
    } finally {
        isLoading.value = false;
    }
}

onMounted(async () => {
  await fetchDetails();

  unlistenMetadata = await listen<number>('metadata_updated', (event) => {
    if (event.payload === gameId.value) {
      fetchDetails();
    }
  });

  unlistenInstall = await listen<InstallProgress>('install_progress', (event) => {
      if (event.payload.id === gameId.value) {
          installStatus.value = event.payload.status;
          installProgress.value = event.payload.progress;
          if (event.payload.progress === 100) {
              // Refresh details once installation is complete
              setTimeout(() => fetchDetails(), 1000);
          }
      }
  });
});

onUnmounted(() => {
  if (unlistenMetadata) unlistenMetadata();
  if (unlistenInstall) unlistenInstall();
});

const handleInstall = () => {
    if(!game.value) return;
    installStatus.value = 'Starting installation...';
    installProgress.value = 0;
    api.installGame(game.value.id);
}

const handlePlay = () => {
    if(!game.value) return;
    console.log(`Launching ${game.value.title}`);
    api.launchGame(game.value.id);
}

const goBack = () => {
    router.push('/');
}

// Helper function to format date
const formatDate = (dateString: string | null) => {
    if (!dateString) return 'Unknown';
    try {
        return new Date(dateString).toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    } catch {
        return 'Invalid Date';
    }
}

// Helper function to get status color
const getStatusColor = (status: string) => {
    switch (status) {
        case 'Installed': return 'var(--cosmic-teal)';
        case 'Ready to Install': return 'var(--sunset-orange)';
        case 'Installing': return 'var(--sunset-pink)';
        default: return 'var(--text-muted)';
    }
}
</script>

<template>
    <div class="game-detail-container">
        <!-- Back Navigation -->
        <nav class="detail-nav glass">
            <button @click="goBack" class="back-btn">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                </svg>
                Back to Library
            </button>
        </nav>

        <!-- Loading State -->
        <div v-if="isLoading && !game" class="loading-container">
            <div class="loading-content glass-card">
                <div class="loading-spinner"></div>
                <h2>Loading Game Details...</h2>
                <p>Fetching information and metadata</p>
            </div>
        </div>

        <!-- Game Content -->
        <div v-else-if="game" class="game-content">
            <!-- Hero Section -->
            <section class="hero-section">
                <div class="hero-backdrop" :style="{ backgroundImage: game.coverUrl ? `url(${game.coverUrl})` : 'none' }"></div>
                <div class="hero-overlay glass-dark"></div>
                
                <div class="hero-content">
                    <!-- Game Cover -->
                    <div class="game-cover">
                        <img 
                            :src="game.coverUrl || `https://via.placeholder.com/300x400/2D1B69/FFFFFF?text=${encodeURIComponent(game.title)}`" 
                            :alt="`Cover for ${game.title}`"
                            class="cover-image"
                        >
                        
                        <!-- Status Badge -->
                        <div class="hero-status-badge" :style="{ borderColor: getStatusColor(game.status) }">
                            <span class="status-dot" :style="{ backgroundColor: getStatusColor(game.status) }"></span>
                            {{ game.status }}
                        </div>
                    </div>

                    <!-- Game Info -->
                    <div class="game-info">
                        <div class="game-header">
                            <h1 class="game-title">{{ game.title }}</h1>
                            <p v-if="isLoading" class="loading-text">
                                <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
                                </svg>
                                Updating metadata...
                            </p>
                        </div>

                        <!-- Game Description -->
                        <div class="game-description">
                            <p>{{ game.description || 'No description available. The system is fetching additional details from IGDB.' }}</p>
                        </div>

                        <!-- Game Meta Info -->
                        <div class="game-meta">
                            <div class="meta-grid">
                                <div class="meta-item">
                                    <span class="meta-label">Release Date</span>
                                    <span class="meta-value">{{ formatDate(game.releaseDate) }}</span>
                                </div>
                                <div class="meta-item">
                                    <span class="meta-label">Genre</span>
                                    <span class="meta-value">{{ game.genre || 'Unknown' }}</span>
                                </div>
                                <div class="meta-item">
                                    <span class="meta-label">Developer</span>
                                    <span class="meta-value">{{ game.developer || 'Unknown' }}</span>
                                </div>
                                <div class="meta-item" v-if="game.metacriticScore">
                                    <span class="meta-label">Metacritic Score</span>
                                    <span class="meta-value score">{{ game.metacriticScore }}</span>
                                </div>
                            </div>
                        </div>

                        <!-- Action Button -->
                        <div class="action-section">
                            <button 
                                v-if="game.status === 'Ready to Install'" 
                                @click="handleInstall" 
                                class="btn-primary action-btn"
                                :disabled="!!installStatus"
                            >
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"/>
                                </svg>
                                Install Game
                            </button>
                            
                            <button 
                                v-else-if="game.status === 'Installed'" 
                                @click="handlePlay" 
                                class="btn-primary action-btn play-btn"
                            >
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1m-6 4h1m4 0h1m-6-8h8a2 2 0 012 2v8a2 2 0 01-2 2H8a2 2 0 01-2-2V8a2 2 0 012-2z"/>
                                </svg>
                                Play Game
                            </button>
                            
                            <button v-else disabled class="btn-secondary action-btn" style="opacity: 0.6; cursor: not-allowed;">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                                {{ game.status }}...
                            </button>
                        </div>
                    </div>
                </div>
            </section>

            <!-- Installation Progress -->
            <section v-if="installStatus" class="progress-section glass-card">
                <div class="progress-header">
                    <h3>Installation Progress</h3>
                    <span class="progress-percentage">{{ installProgress }}%</span>
                </div>
                
                <div class="progress-bar-container">
                    <div class="progress-bar">
                        <div class="progress-fill" :style="{ width: installProgress + '%' }"></div>
                    </div>
                </div>
                
                <p class="progress-status">{{ installStatus }}</p>
            </section>
        </div>

        <!-- Error State -->
        <div v-else class="error-container">
            <div class="error-content glass-card">
                <div class="error-icon">
                    <svg class="w-16 h-16" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.732-.833-2.5 0L4.268 18.5c-.77.833.192 2.5 1.732 2.5z"/>
                    </svg>
                </div>
                <h2>Game Not Found</h2>
                <p>The requested game could not be found in your library.</p>
                <button @click="goBack" class="btn-primary">
                    Return to Library
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
.game-detail-container {
    min-height: 100vh;
    background: var(--gradient-cosmic);
}

/* Navigation */
.detail-nav {
    position: sticky;
    top: 0;
    z-index: 100;
    padding: 1rem 2rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.back-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: var(--glass-white);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 12px;
    color: var(--text-primary);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.back-btn:hover {
    background: var(--glass-white-strong);
    transform: translateX(-2px);
}

/* Loading State */
.loading-container,
.error-container {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 80vh;
    padding: 2rem;
}

.loading-content,
.error-content {
    text-align: center;
    padding: 3rem 2rem;
    max-width: 400px;
}

.loading-spinner {
    width: 4rem;
    height: 4rem;
    border: 4px solid rgba(255, 255, 255, 0.3);
    border-top: 4px solid var(--sunset-pink);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 2rem auto;
}

.error-icon {
    margin-bottom: 1.5rem;
    color: var(--sunset-orange);
}

/* Hero Section */
.hero-section {
    position: relative;
    min-height: 70vh;
    display: flex;
    align-items: center;
    overflow: hidden;
}

.hero-backdrop {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-size: cover;
    background-position: center;
    filter: blur(20px) brightness(0.3);
    transform: scale(1.1);
}

.hero-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

.hero-content {
    position: relative;
    display: grid;
    grid-template-columns: 300px 1fr;
    gap: 3rem;
    padding: 4rem 2rem;
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
}

.game-cover {
    position: relative;
}

.cover-image {
    width: 100%;
    height: 400px;
    object-fit: cover;
    border-radius: 16px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
}

.hero-status-badge {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: var(--glass-dark-strong);
    backdrop-filter: blur(20px);
    border: 2px solid;
    border-radius: 20px;
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
}

/* Game Info */
.game-info {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    color: var(--text-primary);
}

.game-header {
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    padding-bottom: 1rem;
}

.game-title {
    font-size: 3rem;
    font-weight: 700;
    line-height: 1.1;
    margin-bottom: 0.5rem;
    background: var(--gradient-sunset);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}

.loading-text {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--sunset-orange);
    font-size: 0.875rem;
}

.game-description p {
    font-size: 1.125rem;
    line-height: 1.6;
    color: var(--text-secondary);
}

.meta-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
}

.meta-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.meta-label {
    font-size: 0.875rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.meta-value {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary);
}

.meta-value.score {
    color: var(--cosmic-teal);
}

.action-section {
    margin-top: 1rem;
}

.action-btn {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 2rem;
    font-size: 1.125rem;
    font-weight: 600;
    min-width: 200px;
    justify-content: center;
}

.play-btn {
    background: linear-gradient(135deg, var(--cosmic-teal), var(--cosmic-lavender)) !important;
}

/* Progress Section */
.progress-section {
    margin: 2rem auto;
    max-width: 800px;
    padding: 2rem;
}

.progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
}

.progress-header h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
}

.progress-percentage {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--sunset-pink);
}

.progress-bar-container {
    margin-bottom: 1rem;
}

.progress-bar {
    width: 100%;
    height: 8px;
    background: var(--glass-dark);
    border-radius: 4px;
    overflow: hidden;
}

.progress-fill {
    height: 100%;
    background: var(--gradient-sunset);
    border-radius: 4px;
    transition: width 0.3s ease;
}

.progress-status {
    text-align: center;
    color: var(--text-secondary);
    font-size: 0.875rem;
}

/* Responsive Design */
@media (max-width: 1024px) {
    .hero-content {
        grid-template-columns: 1fr;
        gap: 2rem;
        text-align: center;
    }
    
    .game-cover {
        max-width: 300px;
        margin: 0 auto;
    }
}

@media (max-width: 768px) {
    .game-title {
        font-size: 2rem;
    }
    
    .meta-grid {
        grid-template-columns: 1fr;
        gap: 1rem;
    }
    
    .hero-content {
        padding: 2rem 1rem;
    }
    
    .detail-nav {
        padding: 1rem;
    }
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}
</style>
