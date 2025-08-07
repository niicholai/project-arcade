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
  isWhite: boolean;
}

const stars = ref<Star[]>([]);
const staticCloud = ref<HTMLImageElement | null>(null);
const twinklingStars = ref(new Set<number>());

onMounted(async () => {
  // Generate stars with new properties
  const totalStars = 67; // Reduced to 60% of 112
  const maxTwinklingStars = Math.floor(totalStars * 0.15); // 15% of stars can twinkle at once

  // Function to randomly select new stars to twinkle
  const updateTwinklingStars = () => {
    twinklingStars.value.clear();
    const numToTwinkle = Math.floor(Math.random() * maxTwinklingStars) + 1;
    while (twinklingStars.value.size < numToTwinkle) {
      twinklingStars.value.add(Math.floor(Math.random() * totalStars));
    }
  };

  // Start the random twinkling cycle
  setInterval(updateTwinklingStars, 1000); // Reduced from 2000ms to 1000ms

  for (let i = 0; i < totalStars; i++) {
    const maxSize = 26.6; // Reduced by 30% from 38
    const baseSize = Math.random() * (maxSize - 10) + 10; // Random size between 10 and 26.6
    stars.value.push({
      x: Math.random() * 100,
      y: Math.random() * 80, // Keep stars in top 80% of screen
      size: {
        width: baseSize,
        height: baseSize * 0.95 // Maintain aspect ratio close to original
      },
      isWhite: Math.random() < 0.5 // 50% chance of being white
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
  const minDistance = window.innerWidth * 0.75; // Much larger minimum distance between clouds

  // Function to create clouds of one type
  const createClouds = (image: HTMLImageElement, count: number) => {
    for (let i = 0; i < count; i++) {
      let x: number = 0;
      let y: number = 0;
      let attempts = 0;
      const maxAttempts = 10;

      do {
        // Spread clouds out more when spawning
        x = window.innerWidth + (Math.random() * window.innerWidth * 1.5) + (i * window.innerWidth * 0.5);
        // Calculate available height (total height minus big cloud height)
        const header = document.querySelector('.main-content') as HTMLElement;
        const bigCloud = document.querySelector('.static-cloud') as HTMLElement;
        const headerHeight = header ? header.offsetTop + 60 : 60; // Add padding for header
        const bigCloudHeight = bigCloud ? bigCloud.offsetHeight : window.innerHeight * 0.2;
        const availableHeight = window.innerHeight - bigCloudHeight - headerHeight;
        
        // Divide available space into sections for better distribution
        const section = Math.floor(i % 3); // 3 vertical sections
        const sectionHeight = availableHeight / 3;
        const sectionStart = headerHeight + (section * sectionHeight);
        y = sectionStart + (Math.random() * sectionHeight * 0.8); // Keep some padding in each section
        
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

      const scale = 0.02625; // Further reduced by 30% (0.0375 * 0.7)
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

  // Mount the canvas first
  const container = document.querySelector('.app-container') as HTMLElement;
  if (container) {
    canvallax.mount(container);
  }

                // Delay cloud creation by half a second
              setTimeout(() => {
                createClouds(cloudPastel1, 3);
                createClouds(cloudPastel2, 3);
              }, 500);

  // Load and set up the static background cloud
  const bigCloud = await loadImage(bigCloudPastel1Url);
  staticCloud.value = bigCloud;
  
  // Handle window resize for static cloud
  const resizeStaticCloud = () => {
    const container = document.querySelector('.static-cloud') as HTMLElement;
    const img = container?.querySelector('img') as HTMLImageElement;
    if (container && img) {
      const aspectRatio = bigCloud.width / bigCloud.height;
      const baseHeight = window.innerWidth / aspectRatio;
      container.style.height = `${baseHeight * 0.5}px`; // 50% of original height
      img.style.width = '100%';
      img.style.height = '100%';
      img.style.objectFit = 'cover';
    }
  };
  
  window.addEventListener('resize', resizeStaticCloud);
  // Delay initial sizing to ensure image is loaded
  setTimeout(resizeStaticCloud, 100);
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
    <div class="cosmic-bg">
      <!-- Static Background Cloud -->
      <div class="static-cloud">
        <img :src="bigCloudPastel1Url" alt="Background Cloud" />
      </div>
      
      <!-- Animated Stars -->
      <div class="stars">
        <div 
          v-for="(star, index) in stars"
          :key="`star-${index}`"
          class="star"
          :class="{ 'twinkle': twinklingStars.has(index) }"
          :style="{
            left: star.x + '%',
            top: star.y + '%',
            width: star.size.width + 'px',
            height: star.size.height + 'px'
          }"
        >
          <img :src="star1Url" alt="Star" :class="{ 'white-star': star.isWhite }" />
        </div>
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

.cosmic-bg {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 0;
  pointer-events: none;
}

.static-cloud {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  pointer-events: none;
}

.static-cloud img {
  width: 100%;
  height: 100%;
  display: block;
  object-fit: cover;
}

.stars {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
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

.white-star {
  filter: brightness(1.2) saturate(0) contrast(1.2); /* Makes the star appear white */
}

.star.twinkle {
  animation: twinkle 2s ease-in-out infinite;
}

@keyframes twinkle {
  0%, 100% { 
    opacity: 0.8;
    transform: scale(0.95);
    filter: brightness(1);
  }
  50% { 
    opacity: 1;
    transform: scale(1.2);
    filter: brightness(1.5);
  }
}

.main-content {
  position: relative;
  z-index: 2;
  min-height: 100vh;
}
</style>