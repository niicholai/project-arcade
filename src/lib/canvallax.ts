// Canvallax implementation
export class Canvallax {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private clouds: Cloud[] = [];
  private lastTime: number = 0;
  private damping: number = 40;

  constructor(options: { className?: string; damping?: number } = {}) {
    console.log('Creating canvas...'); // Debug 1
    this.canvas = document.createElement('canvas');
    
    // Apply styles directly
    Object.assign(this.canvas.style, {
      position: 'fixed',
      top: '0',
      left: '0',
      width: '100vw',
      height: '100vh',
      pointerEvents: 'none',
      zIndex: '0' // Below UI elements (z-index: 1)
    });

    this.canvas.className = options.className || '';
    const ctx = this.canvas.getContext('2d', {
      alpha: true,
      imageSmoothingEnabled: true,
      imageSmoothingQuality: 'high'
    }) as CanvasRenderingContext2D | null;
    if (!ctx) {
      throw new Error('Failed to get 2D context');
    }
    this.ctx = ctx;
    
    // Enable image smoothing for better quality
    this.ctx.imageSmoothingEnabled = true;
    this.ctx.imageSmoothingQuality = 'high';
    this.damping = options.damping || 40;
    
    // Set initial dimensions
    this.resize();
    console.log('Canvas created with dimensions:', this.canvas.width, this.canvas.height); // Debug 2
    console.log('Canvas styles after direct application:', { 
      position: this.canvas.style.position,
      zIndex: this.canvas.style.zIndex,
      display: this.canvas.style.display,
      width: this.canvas.style.width,
      height: this.canvas.style.height
    }); // Debug 3

    // Add resize listener
    window.addEventListener('resize', () => {
      this.resize();
      console.log('Canvas resized to:', this.canvas.width, this.canvas.height); // Debug 4
    });
  }

  private startAnimation() {
    this.lastTime = performance.now();
    requestAnimationFrame((time) => this.animate(time));
  }

  private resize() {
    const width = window.innerWidth;
    const height = window.innerHeight;
    this.canvas.width = width;
    this.canvas.height = height;
    console.log('Canvas resized to:', width, 'x', height);
  }

  private animate(time: number) {
    // Clear the canvas
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    
    
    // Draw and update clouds
    this.clouds.forEach(cloud => {
      cloud.update(1);
      cloud.draw(this.ctx);
    });

    requestAnimationFrame((t) => this.animate(t));
  }

  public addCloud(cloud: Cloud) {
    this.clouds.push(cloud);
  }

  public mount(container: HTMLElement) {
    container.appendChild(this.canvas);
    this.startAnimation();
  }

  public getClouds() {
    return this.clouds;
  }
}

export class Cloud {
  private x: number;
  private y: number;
  private image: HTMLImageElement;
  private speed: number;
  private scale: number;
  private opacity: number;

  private skewX: number;
  private skewY: number;
  private rotation: number;

  constructor(options: {
    x: number;
    y: number;
    image: HTMLImageElement;
    speed?: number;
    scale?: number;
    opacity?: number;
  }) {
    this.x = options.x;
    this.y = options.y;
    this.image = options.image;
    this.speed = options.speed || 1;
    this.scale = options.scale || 1;
    this.opacity = options.opacity || 0.7;
    
    // Subtle random transformations for natural variety
    this.skewX = (Math.random() - 0.5) * 0.1; // Very slight skew
    this.skewY = (Math.random() - 0.5) * 0.05; // Even slighter vertical skew
    this.rotation = (Math.random() - 0.5) * 0.02; // Barely noticeable rotation
  }

  update(delta: number) {
    this.x -= this.speed;
    const cloudWidth = this.image.width * this.scale * 5.0; // Match the drawing width
    if (this.x + cloudWidth < 0) {
      this.x = window.innerWidth + cloudWidth;
    }
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.save();
    try {
      ctx.globalAlpha = this.opacity;
      ctx.drawImage(
        this.image,
        this.x,
        this.y,
        this.image.width * this.scale,
        this.image.height * this.scale
      );
    } catch (err) {
      console.error('Error drawing cloud:', err);
    }
    ctx.restore();
  }

  getPosition() {
    return {
      x: this.x,
      y: this.y,
      width: this.image.width * this.scale,
      height: this.image.height * this.scale
    };
  }
}