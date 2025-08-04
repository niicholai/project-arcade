import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { AppConfig } from '@/types'
import * as api from '@/services/api'

export const useSettingsStore = defineStore('settings', () => {
  const config = ref<AppConfig>({ installDirectory: null })
  const isLoading = ref(false)

  const fetchConfig = async () => {
    isLoading.value = true;
    try {
      config.value = await api.getConfig()
    } catch (error) {
      console.error('Failed to fetch config:', error)
    } finally {
        isLoading.value = false;
    }
  }

  const saveConfig = async () => {
    try {
      await api.saveConfig(config.value);
    } catch (error) {
      console.error('Failed to save config:', error)
    }
  }
  
  // Automatically save when the config changes
  watch(config, saveConfig, { deep: true });

  return { config, fetchConfig, saveConfig, isLoading }
})
