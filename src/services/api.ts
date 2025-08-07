import { invoke } from '@tauri-apps/api/tauri'
import type { Game, AppConfig } from './types'

export const getGames = () => invoke<Game[]>('get_games')
export const getGameDetails = (id: number) => invoke<Game>('get_game_details', { id })
export const addGameManually = (filePath: string, igdbId: number) => invoke<Game>('add_game_manually', { filePath, igdbId })
export const removeGame = (id: number) => invoke('remove_game', { id })
export const refreshMetadata = (id: number) => invoke('refresh_metadata', { id })

export const installGame = (id: number) => invoke('install_game', { id })
export const launchGame = (id: number) => invoke('launch_game', { id })

export const getConfig = () => invoke<AppConfig>('get_config')
export const saveConfig = (config: AppConfig) => invoke('save_config', { config })
