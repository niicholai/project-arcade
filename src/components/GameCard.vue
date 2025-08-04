<script setup lang="ts">
import type { Game } from '@/types';
import { RouterLink } from 'vue-router';
import { computed } from 'vue';

const props = defineProps<{
  game: Game;
}>();

// Compute status styling
const statusColor = computed(() => {
  switch (props.game.status) {
    case 'Installed': return 'var(--cosmic-teal)';
    case 'Ready to Install': return 'var(--sunset-orange)';
    case 'Installing': return 'var(--sunset-pink)';
    default: return 'var(--text-muted)';
  }
});

// Generate a placeholder based on game title
const placeholderImage = computed(() => {
  return `https://via.placeholder.com/300x400/2D1B69/FFFFFF?text=${encodeURIComponent(props.game.title)}`;
});

// Helper functions
const getActionText = (status: string) => {
  switch (status) {
    case 'Installed': return 'Play';
    case 'Ready to Install': return 'Install';
    case 'Installing': return 'Installing...';
    default: return 'View';
  }
}

const handleQuickAction = () => {
  // This would typically emit an event or call the appropriate API
  console.log(`Quick action for ${props.game.title}: ${props.game.status}`);
}
</script>

<template>
  <RouterLink :to="{ name: 'game-details', params: { id: game.id } }" class="game-card-link">
    <article class="game-card glass-card">
      <!-- Game Cover -->
      <div class="card-cover">
        <img 
          :src="game.coverUrl || placeholderImage" 
          :alt="`Cover for ${game.title}`" 
          class="cover-image"
          loading="lazy"
        >
        
        <!-- Hover Overlay -->
        <div class="card-overlay">
          <div class="overlay-content">
            <svg class="play-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1m-6 4h1m4 0h1m-6-8h8a2 2 0 012 2v8a2 2 0 01-2 2H8a2 2 0 01-2-2V8a2 2 0 012-2z"/>
            </svg>
            <span class="overlay-text">View Details</span>
          </div>
        </div>

        <!-- Status Badge -->
        <div class="status-badge" :style="{ color: statusColor }">
          <span class="status-indicator" :style="{ backgroundColor: statusColor }"></span>
          {{ game.status }}
        </div>
      </div>

      <!-- Game Info -->
      <div class="card-info">
        <h3 class="game-title" :title="game.title">{{ game.title }}</h3>
        
        <!-- Additional Info -->
        <div class="game-meta">
          <span v-if="game.releaseDate" class="meta-item">
            {{ new Date(game.releaseDate).getFullYear() }}
          </span>
          <span v-if="game.genre" class="meta-item">
            {{ game.genre }}
          </span>
        </div>

        <!-- Quick Action -->
        <div class="quick-action">
          <button 
            class="action-btn"
            :class="{
              'btn-install': game.status === 'Ready to Install',
              'btn-play': game.status === 'Installed',
              'btn-installing': game.status === 'Installing'
            }"
            @click.prevent="handleQuickAction"
          >
            {{ getActionText(game.status) }}
          </button>
        </div>
      </div>
    </article>
  </RouterLink>
</template>



<style scoped>
.game-card-link {
  display: block;
  text-decoration: none;
  color: inherit;
}

.game-card {
  position: relative;
  display: flex;
  flex-direction: column;
  height: 320px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.game-card:hover {
  transform: translateY(-8px) scale(1.02);
}

/* Card Cover */
.card-cover {
  position: relative;
  flex: 1;
  overflow: hidden;
}

.cover-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.game-card:hover .cover-image {
  transform: scale(1.1);
}

/* Hover Overlay */
.card-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--glass-dark-strong);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.game-card:hover .card-overlay {
  opacity: 1;
}

.overlay-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  color: white;
}

.play-icon {
  width: 3rem;
  height: 3rem;
  opacity: 0.9;
}

.overlay-text {
  font-size: 0.875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* Status Badge */
.status-badge {
  position: absolute;
  top: 0.75rem;
  right: 0.75rem;
  background: var(--glass-dark-strong);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 20px;
  padding: 0.25rem 0.75rem;
  font-size: 0.75rem;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 0.375rem;
}

.status-indicator {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

/* Card Info */
.card-info {
  padding: 1rem;
  background: var(--glass-white);
  backdrop-filter: blur(20px);
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.game-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  line-height: 1.2;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.game-meta {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 0.75rem;
  flex-wrap: wrap;
}

.meta-item {
  font-size: 0.75rem;
  color: var(--text-muted);
  padding: 0.125rem 0.5rem;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

/* Quick Action */
.quick-action {
  opacity: 0;
  transform: translateY(8px);
  transition: all 0.3s ease;
}

.game-card:hover .quick-action {
  opacity: 1;
  transform: translateY(0);
}

.action-btn {
  width: 100%;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 8px;
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.btn-install {
  background: linear-gradient(135deg, var(--sunset-orange), var(--sunset-pink));
  color: white;
}

.btn-play {
  background: linear-gradient(135deg, var(--cosmic-teal), var(--cosmic-lavender));
  color: white;
}

.btn-installing {
  background: var(--glass-white-strong);
  color: var(--text-secondary);
  cursor: not-allowed;
}

.action-btn:hover:not(.btn-installing) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .game-card {
    height: 280px;
  }
  
  .card-info {
    padding: 0.75rem;
  }
  
  .game-title {
    font-size: 0.875rem;
  }
  
  .quick-action {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
