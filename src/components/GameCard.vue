<script setup lang="ts">
import type { Game } from '@/types';
import { useRouter } from 'vue-router';
import { computed } from 'vue';

const props = defineProps<{
  game: Game;
}>();

const router = useRouter();

// Generate a placeholder based on game title
const placeholderImage = computed(() => {
  return `https://via.placeholder.com/300x400/2D1B69/FFFFFF?text=${encodeURIComponent(props.game.title)}`;
});

const handleClick = () => {
  // Navigate to game details page
  router.push({ name: 'game-details', params: { id: props.game.id } });
}
</script>

<template>
  <div class="game-card" @click="handleClick">
    <img 
      :src="game.coverUrl || placeholderImage" 
      :alt="`Cover for ${game.title}`" 
      class="game-image"
      loading="lazy"
    >
  </div>
</template>

<style scoped>
.game-card {
  width: 140px;
  height: 196px;
  cursor: pointer;
  border-radius: 8px;
  overflow: hidden;
  background: var(--glass-dark);
}

.game-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .game-card {
    width: 112px;
    height: 157px;
  }
}
</style>
