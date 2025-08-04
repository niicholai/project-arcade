<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue';
import { useLibraryStore } from '@/stores/library';
import { open } from '@tauri-apps/api/dialog';

const emit = defineEmits<{
  close: []
}>();

const libraryStore = useLibraryStore();

const filePath = ref('');
const igdbId = ref('');
const isSubmitting = ref(false);
const modalRef = ref<HTMLElement>();

const selectFile = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Game Archive', extensions: ['rar'] }]
    });
    if (typeof selected === 'string') {
      filePath.value = selected;
    }
  } catch (error) {
    console.error('Failed to select file:', error);
  }
};

const handleSubmit = async () => {
  if (!filePath.value || !igdbId.value) {
    return;
  }

  isSubmitting.value = true;
  try {
    await libraryStore.addGame(filePath.value, parseInt(igdbId.value, 10));
    emit('close');
  } catch (error) {
    console.error('Failed to add game:', error);
  } finally {
    isSubmitting.value = false;
  }
};

const handleClickOutside = (event: MouseEvent) => {
  if (modalRef.value && !modalRef.value.contains(event.target as Node)) {
    emit('close');
  }
};

const handleEscape = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    emit('close');
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleEscape);
  return () => {
    document.removeEventListener('keydown', handleEscape);
  };
});
</script>

<template>
  <!-- Modal Backdrop -->
  <div 
    class="modal-backdrop"
    @click="handleClickOutside"
  >
    <!-- Modal Container -->
    <div 
      ref="modalRef"
      class="modal-container glass-strong"
      @click.stop
    >
      <!-- Modal Header -->
      <header class="modal-header">
        <div class="header-content">
          <div class="header-icon">
            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
            </svg>
          </div>
          <div class="header-text">
            <h2 class="modal-title">Add New Game</h2>
            <p class="modal-subtitle">Import a game archive to your library</p>
          </div>
        </div>
        
        <button @click="emit('close')" class="close-btn">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
          </svg>
        </button>
      </header>

      <!-- Modal Body -->
      <main class="modal-body">
        <form @submit.prevent="handleSubmit" class="form-container">
          <!-- File Selection -->
          <div class="form-group">
            <label class="form-label">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
              </svg>
              Game Archive File
            </label>
            <p class="form-help">Select a .rar file containing your game</p>
            
            <div class="file-input-group">
              <input
                type="text"
                readonly
                :value="filePath ? filePath.split('\\').pop() || filePath.split('/').pop() : ''"
                class="form-input file-display"
                placeholder="No file selected..."
              />
              <button 
                @click="selectFile" 
                type="button" 
                class="btn-secondary file-browse"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-5l-2-2H5a2 2 0 00-2 2z"/>
                </svg>
                Browse
              </button>
            </div>
            
            <div v-if="filePath" class="file-path">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              {{ filePath }}
            </div>
          </div>

          <!-- IGDB ID Input -->
          <div class="form-group">
            <label for="igdb-id" class="form-label">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4V2a1 1 0 011-1h8a1 1 0 011 1v2M7 4h10M7 4a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V6a2 2 0 00-2-2"/>
              </svg>
              IGDB Game ID
            </label>
            <p class="form-help">
              Find the game on 
              <a href="https://www.igdb.com" target="_blank" class="form-link">igdb.com</a>
              and copy the ID from the URL
            </p>
            
            <input
              id="igdb-id"
              type="text"
              v-model="igdbId"
              class="form-input"
              placeholder="e.g., 1074"
              pattern="[0-9]+"
              title="Please enter a valid IGDB ID (numbers only)"
            />
          </div>

          <!-- Form Actions -->
          <div class="form-actions">
            <button 
              type="button" 
              @click="emit('close')" 
              class="btn-secondary"
              :disabled="isSubmitting"
            >
              Cancel
            </button>
            
            <button 
              type="submit" 
              class="btn-primary"
              :disabled="!filePath || !igdbId || isSubmitting"
            >
              <svg v-if="isSubmitting" class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
              </svg>
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
              </svg>
              {{ isSubmitting ? 'Adding Game...' : 'Add Game' }}
            </button>
          </div>
        </form>
      </main>
    </div>
  </div>
</template>

<style scoped>
/* Modal Backdrop */
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 2rem;
  animation: backdrop-fade-in 0.3s ease;
}

@keyframes backdrop-fade-in {
  from { opacity: 0; }
  to { opacity: 1; }
}

/* Modal Container */
.modal-container {
  width: 100%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  border-radius: 24px;
  animation: modal-slide-in 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes modal-slide-in {
  from { 
    opacity: 0; 
    transform: scale(0.9) translateY(-20px); 
  }
  to { 
    opacity: 1; 
    transform: scale(1) translateY(0); 
  }
}

/* Modal Header */
.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 2rem 2rem 1rem 2rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.header-content {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.header-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 3rem;
  height: 3rem;
  background: var(--gradient-sunset);
  border-radius: 16px;
  color: white;
}

.header-text {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.modal-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.modal-subtitle {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin: 0;
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2.5rem;
  height: 2.5rem;
  background: var(--glass-white);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: var(--glass-white-strong);
  color: var(--text-primary);
}

/* Modal Body */
.modal-body {
  padding: 2rem;
}

.form-container {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

/* Form Groups */
.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.form-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.form-help {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
  line-height: 1.4;
}

.form-link {
  color: var(--cosmic-teal);
  text-decoration: none;
  font-weight: 500;
}

.form-link:hover {
  text-decoration: underline;
}

/* Form Inputs */
.form-input {
  padding: 1rem 1.25rem;
  background: var(--glass-white);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  color: var(--text-primary);
  font-size: 1rem;
  transition: all 0.2s ease;
}

.form-input:focus {
  outline: none;
  border-color: var(--cosmic-teal);
  background: var(--glass-white-strong);
  box-shadow: 0 0 0 3px rgba(0, 180, 216, 0.1);
}

.form-input::placeholder {
  color: var(--text-muted);
}

/* File Input Group */
.file-input-group {
  display: flex;
  gap: 0.5rem;
}

.file-display {
  flex: 1;
  cursor: default;
}

.file-browse {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem 1.5rem;
  white-space: nowrap;
}

.file-path {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  background: rgba(0, 180, 216, 0.1);
  border: 1px solid rgba(0, 180, 216, 0.3);
  border-radius: 8px;
  font-size: 0.875rem;
  color: var(--cosmic-teal);
  word-break: break-all;
}

/* Form Actions */
.form-actions {
  display: flex;
  gap: 1rem;
  padding-top: 1rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.form-actions .btn-secondary {
  flex: 1;
}

.form-actions .btn-primary {
  flex: 2;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.form-actions .btn-primary:disabled,
.form-actions .btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Responsive Design */
@media (max-width: 768px) {
  .modal-backdrop {
    padding: 1rem;
  }
  
  .modal-container {
    max-height: 95vh;
  }
  
  .modal-header {
    padding: 1.5rem 1.5rem 1rem 1.5rem;
  }
  
  .modal-body {
    padding: 1.5rem;
  }
  
  .file-input-group {
    flex-direction: column;
  }
  
  .form-actions {
    flex-direction: column;
  }
  
  .form-actions .btn-secondary,
  .form-actions .btn-primary {
    flex: none;
  }
}
</style>
