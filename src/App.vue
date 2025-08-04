<script setup lang="ts">
import { RouterView } from 'vue-router'
import { ref, onMounted } from 'vue'
import { Canvallax, Cloud } from './lib/canvallax'
import cloudPastel1Url from './assets/images/cloudPastel1.png?url'
import cloudPastel2Url from './assets/images/cloudPastel2.png?url'
import bigCloudPastel1Url from './assets/images/bigCloud_pastelColors1.png?url'
import star1Url from './assets/images/star1.png?url'

// Add cosmic particles/stars with new properties
interface Star {
  x: number;
  y: number;
  size: { width: number; height: number };
  twinkleDelay: number;
  shouldTwinkle: boolean;
}

const stars = ref<Star[]>([]);
const staticCloud = ref<HTMLImageElement | null>(null);

onMounted(async () => {
  // Generate stars with new properties
  const totalStars = 80;
  const numTwinklingStars = Math.floor(Math.random() * 3) + 1; // Random 1-3 twinkling stars
  const twinklingStarIndices = new Set<number>();
  
  while (twinklingStarIndices.size < numTwinklingStars) {
    twinklingStarIndices.add(Math.floor(Math.random() * totalStars));
  }

  for (let i = 0; i < totalStars; i++) {
    const baseSize = Math.random() * (38 - 10) + 10; // Random size between 10 and 38
    stars.value.push({
      x: Math.random() * 100,
      y: Math.random() * 100,
      size: {
        width: baseSize,
        height: baseSize * 0.95 // Maintain aspect ratio close to original
      },
      twinkleDelay: Math.random() * 7 + 2, // Random delay between 2-9 seconds
      shouldTwinkle: twinklingStarIndices.has(i)
    });
  }

  // Initialize Canvallax
  const canvallax = new Canvallax({
    className: 'cloud-canvas',
    damping: 40
  });

  // Load all cloud images
  const [cloudPastel1, cloudPastel2] = await Promise.all([
    loadImage(cloudPastel1Url),
    loadImage(cloudPastel2Url)
  ]);

  // Create an array of random positions that ensures good distribution
  const positions: { x: number; y: number }[] = [];
  const minDistance = window.innerWidth * 0.25; // Increased minimum distance for larger clouds

  // Function to create clouds of one type
  const createClouds = (image: HTMLImageElement, count: number) => {
    for (let i = 0; i < count; i++) {
      let x: number = 0;
      let y: number = 0;
      let attempts = 0;
      const maxAttempts = 10;

      do {
        x = Math.random() * (window.innerWidth * 1.5);
        y = Math.random() * (window.innerHeight * 0.8); // Keep clouds in upper 80% of screen
        
        const isFarEnough = positions.every(pos => {
          const dx = x - pos.x;
          const dy = y - pos.y;
          return Math.sqrt(dx * dx + dy * dy) > minDistance;
        });

        if (isFarEnough || attempts >= maxAttempts) {
          positions.push({ x, y });
          break;
        }
        attempts++;
      } while (attempts < maxAttempts);

      const scale = 0.25; // 75% reduction in size
      const speed = Math.random() * 0.3 + 0.05; // Varied speed between 0.05 and 0.35
      
      const cloud = new Cloud({
        x,
        y,
        image,
        speed,
        scale,
        opacity: 0.9
      });
      canvallax.addCloud(cloud);
    }
  };

  // Create 3 clouds of each type
  createClouds(cloudPastel1, 3);
  createClouds(cloudPastel2, 3);

  // Mount the canvas
  const container = document.querySelector('.app-container') as HTMLElement;
  if (container) {
    canvallax.mount(container);
  }

  // Load and set up the static background cloud
  const bigCloud = await loadImage(bigCloudPastel1Url);
  staticCloud.value = bigCloud;
  
  // Handle window resize for static cloud
  const resizeStaticCloud = () => {
    if (staticCloud.value) {
      const container = document.querySelector('.static-cloud') as HTMLElement;
      if (container) {
        const aspectRatio = staticCloud.value.width / staticCloud.value.height;
        container.style.width = '100%';
        container.style.height = `${window.innerWidth / aspectRatio}px`;
      }
    }
  };
  
  window.addEventListener('resize', resizeStaticCloud);
  resizeStaticCloud(); // Initial sizing
});

// Helper function to load images
function loadImage(src: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => resolve(img);
    img.onerror = reject;
    img.src = src;
  });
}
</script>

<template>
  <div class="app-container">
    <!-- Cosmic Background -->
    <div class="cosmic-bg"></div>
    
    <!-- Static Background Cloud -->
    <div class="static-cloud">
      <img v-if="staticCloud" :src="bigCloudPastel1Url" alt="Background Cloud" />
    </div>
    
    <!-- Animated Stars -->
    <div class="stars">
      <div 
        v-for="(star, index) in stars"
        :key="`star-${index}`"
        class="star"
        :class="{ 'twinkle': star.shouldTwinkle }"
        :style="{
          left: star.x + '%',
          top: star.y + '%',
          width: star.size.width + 'px',
          height: star.size.height + 'px',
          animationDelay: star.shouldTwinkle ? star.twinkleDelay + 's' : undefined
        }"
      >
        <img :src="star1Url" alt="Star" />
      </div>
    </div>

    <!-- Main Content -->
    <div class="main-content">
      <RouterView />
    </div>
  </div>
</template>

<style scoped>
.app-container {
  position: relative;
  height: 100vh;
  overflow: hidden;
  margin: 0;
  padding: 0;
}

.static-cloud {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  z-index: 0;
  pointer-events: none;
}

.static-cloud img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: bottom;
}

.stars {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 1;
}

.star {
  position: absolute;
  display: flex;
  justify-content: center;
  align-items: center;
}

.star img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.star.twinkle {
  animation: twinkle 4s ease-in-out infinite;
}

@keyframes twinkle {
  0%, 100% { 
    opacity: 0.4;
    transform: scale(1);
  }
  50% { 
    opacity: 1;
    transform: scale(1.2);
  }
}

.main-content {
  position: relative;
  z-index: 2;
  min-height: 100vh;
}
</style>