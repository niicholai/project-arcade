<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { dialog } from '@tauri-apps/api';

const settingsStore = useSettingsStore();
const router = useRouter();
const isSaving = ref(false);
const saveSuccess = ref(false);

onMounted(() => {
    settingsStore.fetchConfig();
});

const handleSave = async () => {
    isSaving.value = true;
    saveSuccess.value = false;
    
    try {
        await settingsStore.saveConfig();
        saveSuccess.value = true;
        setTimeout(() => {
            saveSuccess.value = false;
        }, 3000);
    } catch (error) {
        console.error('Failed to save settings:', error);
    } finally {
        isSaving.value = false;
    }
}

const goBack = () => {
    router.push('/');
}

// Handle folder picker (this would need to be implemented in the backend)
const selectFolder = async () => {
    try {
        const selected = await dialog.open({
            directory: true,
            multiple: false,
            defaultPath: settingsStore.config.installDirectory || undefined
        });
        
        if (selected) {
            settingsStore.config.installDirectory = selected as string;
        }
    } catch (error) {
        console.error('Failed to open folder dialog:', error);
    }
}
</script>

<template>
    <div class="settings-container">
        <!-- Navigation -->
        <nav class="settings-nav glass">
            <button @click="goBack" class="back-btn">
                <svg class="w-2 h-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                </svg>
                Back to Library
            </button>
        </nav>

        <!-- Settings Content -->
        <main class="settings-main">
            <div class="settings-card glass-card">
                <!-- Header -->
                <header class="settings-header">
                    <div class="header-icon">
                        <svg class="w-2 h-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                        </svg>
                    </div>
                    <div class="header-text">
                        <h1 class="settings-title">Application Settings</h1>
                        <p class="settings-subtitle">Configure your Project Arcade preferences</p>
                    </div>
                </header>

                <!-- Loading State -->
                <div v-if="settingsStore.isLoading" class="loading-section">
                    <div class="loading-spinner"></div>
                    <p>Loading settings...</p>
                </div>

                <!-- Settings Form -->
                <form v-else @submit.prevent="handleSave" class="settings-form">
                    <!-- Installation Directory -->
                    <section class="form-section">
                        <h2 class="section-title">
                            <svg class="w-2 h-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"/>
                            </svg>
                            Game Installation
                        </h2>
                        <p class="section-description">
                            Choose where your games will be installed on your computer.
                        </p>

                        <div class="form-group">
                            <label for="install-dir" class="form-label">
                                Installation Directory
                            </label>
                            <div class="path-input-group">
                                <input
                                    id="install-dir"
                                    type="text"
                                    v-model="settingsStore.config.installDirectory"
                                    class="form-input path-input"
                                    placeholder="C:\Games\Arcade"
                                />
                                <button 
                                    type="button"
                                    @click="selectFolder"
                                    class="btn-secondary folder-btn"
                                >
                                    <svg class="w-2 h-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-5l-2-2H5a2 2 0 00-2 2z"/>
                                    </svg>
                                    Browse
                                </button>
                            </div>
                            <p class="form-help">
                                Games will be extracted and stored in this directory. Make sure you have enough free space.
                            </p>
                        </div>
                    </section>

                    <!-- Future Settings Sections -->
                    <section class="form-section">
                        <h2 class="section-title">
                            <svg class="w-2 h-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                            Metadata & Integration
                        </h2>
                        <p class="section-description">
                            Configure how Project Arcade fetches and displays game information.
                        </p>

                        <div class="form-group">
                            <div class="info-box">
                                <svg class="w-2 h-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                                <div>
                                    <p class="info-title">Automatic Metadata Fetching</p>
                                    <p class="info-text">Project Arcade automatically fetches game information from IGDB when you add games to your library.</p>
                                </div>
                            </div>
                        </div>
                    </section>

                    <!-- Save Button -->
                    <div class="form-actions">
                        <button 
                            type="submit" 
                            class="btn-primary save-btn"
                            :disabled="isSaving"
                        >
                            <svg v-if="isSaving" class="w-2 h-2 animate-spin" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
                            </svg>
                            <svg v-else-if="saveSuccess" class="w-2 h-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                            </svg>
                            <svg v-else class="w-2 h-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4"/>
                            </svg>
                            {{ isSaving ? 'Saving...' : saveSuccess ? 'Saved!' : 'Save Settings' }}
                        </button>
                    </div>

                    <!-- Success Message -->
                    <div v-if="saveSuccess" class="success-message">
                        <svg class="w-2 h-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        Settings saved successfully!
                    </div>
                </form>
            </div>
        </main>
    </div>
</template>

<style scoped>
.settings-container {
    min-height: 100vh;
    background: var(--gradient-cosmic);
}

/* Navigation */
.settings-nav {
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

/* Main Content */
.settings-main {
    padding: 2rem;
    display: flex;
    justify-content: center;
}

.settings-card {
    width: 100%;
    max-width: 800px;
    padding: 2rem;
}

/* Header */
.settings-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 2rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.header-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    background: var(--gradient-sunset);
    border-radius: 6px;
    color: white;
}

.header-text {
    flex: 1;
}

.settings-title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 0.25rem;
}

.settings-subtitle {
    color: var(--text-secondary);
    font-size: 1rem;
}

/* Loading */
.loading-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 3rem;
    color: var(--text-secondary);
}

.loading-spinner {
    width: 3rem;
    height: 3rem;
    border: 3px solid rgba(255, 255, 255, 0.3);
    border-top: 3px solid var(--sunset-pink);
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

/* Form */
.settings-form {
    display: flex;
    flex-direction: column;
    gap: 2rem;
}

.form-section {
    padding: 1.5rem;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 16px;
    background: var(--glass-white);
    transition: none !important; /* Remove any inherited transitions */
    transform: none !important;
}

.form-section:hover {
    /* Completely prevent all hover effects */
    background: var(--glass-white) !important;
    border-color: rgba(255, 255, 255, 0.1) !important;
    transform: none !important;
    box-shadow: none !important;
    opacity: 1 !important;
}

.form-section * {
    transition: none !important;
}

.form-section *:hover {
    transform: none !important;
}

.section-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.5rem;
}

.section-description {
    color: var(--text-secondary);
    margin-bottom: 1.5rem;
    line-height: 1.5;
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.form-label {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
}

.path-input-group {
    display: flex;
    gap: 0.5rem;
}

.path-input {
    flex: 1;
}

.folder-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem 1.5rem;
    white-space: nowrap;
}

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

.form-help {
    font-size: 0.875rem;
    color: var(--text-muted);
    line-height: 1.4;
}

/* Info Box */
.info-box {
    display: flex;
    gap: 1rem;
    padding: 1rem;
    background: rgba(0, 180, 216, 0.1);
    border: 1px solid rgba(0, 180, 216, 0.3);
    border-radius: 12px;
    color: var(--cosmic-teal);
}

.info-title {
    font-weight: 600;
    margin-bottom: 0.25rem;
}

.info-text {
    font-size: 0.875rem;
    opacity: 0.9;
}

/* Form Actions */
.form-actions {
    display: flex;
    justify-content: flex-end;
    padding-top: 1rem;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.save-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem 2rem;
    font-size: 1rem;
    font-weight: 600;
    min-width: 180px;
    justify-content: center;
}

.save-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
}

/* Success Message */
.success-message {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    background: rgba(0, 180, 216, 0.1);
    border: 1px solid rgba(0, 180, 216, 0.3);
    border-radius: 12px;
    color: var(--cosmic-teal);
    font-weight: 500;
    animation: slide-in 0.3s ease;
}

@keyframes slide-in {
    from {
        opacity: 0;
        transform: translateY(-10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

/* Responsive Design */
@media (max-width: 768px) {
    .settings-main {
        padding: 1rem;
    }
    
    .settings-card {
        padding: 1.5rem;
    }
    
    .settings-header {
        flex-direction: column;
        text-align: center;
        gap: 1rem;
    }
    
    .path-input-group {
        flex-direction: column;
    }
    
    .form-actions {
        justify-content: stretch;
    }
    
    .save-btn {
        width: 100%;
    }
}
</style>
