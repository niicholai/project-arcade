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
    console.log(`Received metadata_updated event for game ID: ${event.payload}, current game ID: ${gameId.value}`);
    if (event.payload === gameId.value) {
      console.log('Metadata updated for current game, refreshing details...');
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

const handleRefreshMetadata = async () => {
    if(!game.value) return;
    console.log(`Refreshing metadata for ${game.value.title}`);
    try {
        await api.refreshMetadata(game.value.id);
        // The metadata_updated event listener will handle the UI update
    } catch(e) {
        console.error("Failed to refresh metadata", e);
    }
}

const goBack = () => {
    router.push('/');
}

// Helper function to format date
const formatDate = (dateString: string | null | undefined) => {
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

// Parse screenshots JSON
const parsedScreenshots = computed(() => {
    if (!game.value?.screenshots) {
        console.log('No screenshots data available');
        return [];
    }
    try {
        const screenshots = JSON.parse(game.value.screenshots) as string[];
        console.log('Parsed screenshots:', screenshots);
        return screenshots;
    } catch (error) {
        console.error('Failed to parse screenshots:', error);
        return [];
    }
});

// Parse videos JSON
const parsedVideos = computed(() => {
    if (!game.value?.videos) {
        console.log('No videos data available');
        return [];
    }
    try {
        const videos = JSON.parse(game.value.videos) as Array<{id: string, title: string}>;
        console.log('Parsed videos:', videos);
        return videos;
    } catch (error) {
        console.error('Failed to parse videos:', error);
        return [];
    }
});

// Modal state
const showScreenshotModal = ref(false);
const showVideoModal = ref(false);
const currentScreenshot = ref('');
const currentVideoId = ref('');

// Carousel state
const currentVideoIndex = ref(0)
const currentScreenshotIndex = ref(0)

// Modal functions
const openScreenshotModal = (screenshotUrl: string) => {
    currentScreenshot.value = screenshotUrl;
    showScreenshotModal.value = true;
};

const openVideoModal = (videoId: string) => {
    console.log('Opening video modal with ID:', videoId);
    console.log('Current video ID before setting:', currentVideoId.value);
    currentVideoId.value = videoId;
    console.log('Current video ID after setting:', currentVideoId.value);
    showVideoModal.value = true;
    console.log('Modal state set to:', showVideoModal.value);
};

const closeScreenshotModal = () => {
    showScreenshotModal.value = false;
    currentScreenshot.value = '';
};

const closeVideoModal = () => {
    showVideoModal.value = false;
    currentVideoId.value = '';
};

const handleVideoThumbnailError = (event: any) => {
    const img = event.target as HTMLImageElement;
    const videoId = img.src.split('/').pop()?.split('.')[0];
    if (videoId) {
        // Try hqdefault first, then mqdefault as final fallback
        if (img.src.includes('maxresdefault')) {
            img.src = `https://img.youtube.com/vi/${videoId}/hqdefault.jpg`;
        } else if (img.src.includes('hqdefault')) {
            img.src = `https://img.youtube.com/vi/${videoId}/mqdefault.jpg`;
        }
    }
};

const previousVideo = () => {
  if (currentVideoIndex.value > 0) {
    currentVideoIndex.value--
  }
}

const nextVideo = () => {
  if (parsedVideos.value && currentVideoIndex.value < parsedVideos.value.length - 3) {
    currentVideoIndex.value++
  }
}

const previousScreenshot = () => {
  if (currentScreenshotIndex.value > 0) {
    currentScreenshotIndex.value--
  }
}

const nextScreenshot = () => {
  if (parsedScreenshots.value && currentScreenshotIndex.value < parsedScreenshots.value.length - 3) {
    currentScreenshotIndex.value++
  }
}
</script>

<template>
    <div class="game-detail-container">
        <!-- Back Navigation -->
        <nav class="detail-nav glass">
            <button @click="goBack" class="back-btn">
                <svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
            <!-- Banner Section -->
            <section v-if="game.bannerUrl" class="banner-section">
                <img :src="game.bannerUrl" :alt="`Banner for ${game.title}`" class="banner-image">
            </section>
            
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
                         
                         <!-- Action Buttons -->
                         <div class="action-buttons-container">
                             <button 
                                 v-if="game.status === 'Ready to Install'" 
                                 @click="handleInstall" 
                                 class="btn-primary action-btn"
                                 :disabled="!!installStatus"
                             >
                                 <svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"/>
                                 </svg>
                                 Install Game
                             </button>
                             
                             <button 
                                 v-else-if="game.status === 'Installed'" 
                                 @click="handlePlay" 
                                 class="btn-primary action-btn play-btn"
                             >
                                 <svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1m-6 4h1m4 0h1m-6-8h8a2 2 0 012 2v8a2 2 0 01-2 2H8a2 2 0 01-2-2V8a2 2 0 012-2z"/>
                                 </svg>
                                 Play Game
                             </button>
                             
                             <button v-else disabled class="btn-secondary action-btn" style="opacity: 0.6; cursor: not-allowed;">
                                 <svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                 </svg>
                                 {{ game.status }}...
                             </button>
                             
                             <!-- Refresh Metadata Button -->
                             <button 
                                 @click="handleRefreshMetadata" 
                                 class="btn-primary action-btn"
                                 :disabled="isLoading"
                             >
                                 <svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                                 </svg>
                                 Refresh Metadata
                             </button>
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
                                 <div class="info-item">
                                     <strong>Genre:</strong> {{ game?.genre || 'Unknown' }}
                                 </div>
                                 <div class="info-item">
                                     <strong>Developer:</strong> {{ game?.developer || 'Unknown' }}
                                 </div>
                                 <div class="info-item">
                                     <strong>Publisher:</strong> {{ game?.publisher || 'Unknown' }}
                                 </div>
                                 <div class="info-item">
                                     <strong>Release Date:</strong> {{ game?.releaseDate || 'Unknown' }}
                                 </div>
                                 <div class="info-item" v-if="game?.themes">
                                     <strong>Themes:</strong> {{ game.themes }}
                                 </div>
                             </div>
                         </div>

                                                   <!-- Videos Section -->
          <div class="media-section" v-if="parsedVideos && parsedVideos.length > 0">
            <h3>Videos</h3>
            <div class="carousel-container">
              <button 
                class="carousel-arrow carousel-arrow-left" 
                @click="previousVideo"
                :disabled="currentVideoIndex === 0"
              >
                ‹
              </button>
              <div class="carousel">
                <div 
                  v-for="(video, index) in parsedVideos" 
                  :key="video.id"
                  class="carousel-item video-item"
                  @click.stop="openVideoModal(video.id)"
                  v-show="index >= currentVideoIndex && index < currentVideoIndex + 3"
                >
                  <img 
                    :src="`https://img.youtube.com/vi/${video.id}/maxresdefault.jpg`" 
                    :alt="video.title"
                    class="video-thumbnail"
                    @error="handleVideoThumbnailError"
                  />
                  <div class="video-overlay">
                    <div class="video-play-button">▶</div>
                  </div>
                  <div class="video-title">{{ video.title }}</div>
                </div>
              </div>
              <button 
                class="carousel-arrow carousel-arrow-right" 
                @click="nextVideo"
                :disabled="currentVideoIndex >= parsedVideos.length - 3"
              >
                ›
              </button>
            </div>
          </div>
          
          <!-- Screenshots Section -->
          <div class="media-section" v-if="parsedScreenshots && parsedScreenshots.length > 0">
            <h3>Screenshots</h3>
            <div class="carousel-container">
              <button 
                class="carousel-arrow carousel-arrow-left" 
                @click="previousScreenshot"
                :disabled="currentScreenshotIndex === 0"
              >
                ‹
              </button>
              <div class="carousel">
                <div 
                  v-for="(screenshot, index) in parsedScreenshots" 
                  :key="index"
                  class="carousel-item screenshot-item"
                  @click.stop="openScreenshotModal(screenshot)"
                  v-show="index >= currentScreenshotIndex && index < currentScreenshotIndex + 3"
                >
                  <img 
                    :src="screenshot" 
                    :alt="`Screenshot ${index + 1}`"
                    class="screenshot-preview"
                  />
                  <div class="screenshot-overlay">
                    <div class="screenshot-zoom-icon">🔍</div>
                  </div>
                </div>
              </div>
              <button 
                class="carousel-arrow carousel-arrow-right" 
                @click="nextScreenshot"
                :disabled="currentScreenshotIndex >= parsedScreenshots.length - 3"
              >
                ›
              </button>
            </div>
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
     
     <!-- Screenshot Modal Overlay -->
     <div v-if="showScreenshotModal" class="modal-overlay" @click="closeScreenshotModal">
         <div class="modal-content" @click.stop>
             <button class="modal-close-btn" @click="closeScreenshotModal">
                 <svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                 </svg>
             </button>
             <img :src="currentScreenshot" :alt="`Screenshot of ${game?.title}`" class="modal-image">
         </div>
     </div>
     
     <!-- Video Modal Overlay -->
     <div v-if="showVideoModal" class="modal-overlay" @click="closeVideoModal">
         <div class="modal-content" @click.stop>
             <button class="modal-close-btn" @click="closeVideoModal">
                 <svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                 </svg>
             </button>
             <iframe 
                 :src="`https://www.youtube.com/embed/${currentVideoId}?autoplay=1`" 
                 frameborder="0" 
                 allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
                 allowfullscreen
                 class="modal-video"
             ></iframe>
         </div>
     </div>
 </template>

<style scoped>
.game-detail-container {
    min-height: 100vh;
    background: var(--gradient-cosmic);
    overflow-y: auto;
    overflow-x: hidden;
    height: 100vh;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
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
    min-width: 0;
}

.game-cover {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.cover-image {
    width: 100%;
    height: 400px;
    object-fit: cover;
    border-radius: 16px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
}

.action-buttons-container {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;
}

.action-buttons-container .action-btn {
    width: 100%;
    justify-content: center;
}



/* Game Info */
.game-info {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    color: var(--text-primary);
    min-width: 0;
    overflow: hidden;
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
    -webkit-text-stroke: 0.5px rgba(255, 255, 255, 0.3);
    text-stroke: 0.5px rgba(255, 255, 255, 0.3);
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
    min-width: 0;
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

.action-buttons {
    display: flex;
    gap: 1rem;
    align-items: center;
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

.refresh-section {
    margin-left: auto;
}

/* Media Sections */
.media-section {
    margin-top: 2rem;
}

.media-section .section-header h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 1rem;
}

.carousel-container {
    position: relative;
    width: 100%;
    overflow: hidden;
    margin: 1rem 0;
    padding: 0 50px;
}

.carousel {
    display: flex;
    gap: 1rem;
    width: 100%;
}

.carousel-item {
    flex: 0 0 220px;
    position: relative;
    cursor: pointer;
    transition: transform 0.3s ease;
}

.carousel-item:hover {
    transform: scale(1.02);
}

/* Video Items */
.video-item {
    width: 220px;
    height: 140px;
    flex-shrink: 0;
}

.video-thumbnail {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 8px;
}

.video-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
}

.video-play-button {
    color: white;
    font-size: 2rem;
}

.video-title {
    padding: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-primary);
    text-align: center;
    margin-top: 0.5rem;
}

/* Screenshot Items */
.screenshot-item {
    width: 220px;
    height: 140px;
    flex-shrink: 0;
}

.screenshot-preview {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 8px;
}

.screenshot-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.2s ease;
    border-radius: 8px;
}

.screenshot-item:hover .screenshot-overlay {
    opacity: 1;
}

.screenshot-zoom-icon {
    color: white;
    font-size: 1.5rem;
}

/* Modal Overlays */
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.9);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 2rem;
}

.modal-content {
    position: relative;
    max-width: 90vw;
    max-height: 90vh;
    background: var(--glass-dark);
    border-radius: 16px;
    overflow: hidden;
}

.modal-close-btn {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: rgba(0, 0, 0, 0.7);
    border: none;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    cursor: pointer;
    z-index: 1001;
    transition: background 0.2s ease;
}

.modal-close-btn:hover {
    background: rgba(0, 0, 0, 0.9);
}

.modal-image {
    width: 100%;
    height: auto;
    max-height: 90vh;
    object-fit: contain;
}

.modal-video {
    width: 100%;
    height: 70vh;
    max-width: 1200px;
    aspect-ratio: 16/9;
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

/* Banner Section */
.banner-section {
    margin: -2rem -2rem 0.4rem -2rem;
    height: 300px;
    overflow: hidden;
    position: relative;
}

.banner-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: center;
}



/* Responsive Design */
@media (max-width: 1024px) {
    .hero-content {
        grid-template-columns: 1fr;
        gap: 2rem;
        text-align: center;
        padding: 2rem 1rem;
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
        padding: 1rem;
    }
    
    .detail-nav {
        padding: 1rem;
    }
    
    .carousel-item {
        width: 150px;
    }
    
    .screenshot-item {
        width: 150px;
        height: 90px;
    }
}

@media (max-width: 480px) {
    .hero-content {
        padding: 1rem 0.5rem;
    }
    
    .game-title {
        font-size: 1.5rem;
    }
    
    .cover-image {
        height: 300px;
    }
    
    .carousel-item {
        width: 120px;
    }
    
    .screenshot-item {
        width: 120px;
        height: 72px;
    }
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

.carousel-arrow {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  background: rgba(0, 0, 0, 0.7);
  color: white;
  border: none;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  font-size: 20px;
  cursor: pointer;
  z-index: 10;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease;
}

.carousel-arrow:hover {
  background: rgba(0, 0, 0, 0.9);
}

.carousel-arrow:disabled {
  background: rgba(0, 0, 0, 0.3);
  cursor: not-allowed;
}

.carousel-arrow-left {
  left: 0;
}

.carousel-arrow-right {
  right: 0;
}
</style>
