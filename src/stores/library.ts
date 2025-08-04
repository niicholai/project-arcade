import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Game } from '@/types'
import * as api from '@/services/api'

export const useLibraryStore = defineStore('library', () => {
  const games = ref<Game[]>([])
  const isLoading = ref(false)

  const fetchGames = async () => {
    isLoading.value = true
    try {
      games.value = await api.getGames()
    } catch (error) {
      console.error('Failed to fetch games:', error)
      // Here you might want to show a user-facing error
    } finally {
      isLoading.value = false
    }
  }

  const addGame = async (filePath: string, igdbId: number) => {
    try {
        const newGame = await api.addGameManually(filePath, igdbId);
        games.value.push(newGame);
    } catch (error) {
        console.error("Failed to add game:", error);
    }
  }

  const getGameById = (id: number) => {
    return computed(() => games.value.find(g => g.id === id))
  }

  return { games, isLoading, fetchGames, addGame, getGameById }
})
