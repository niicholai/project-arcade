// Shared TypeScript types mirroring the Rust structs

export interface Game {
    id: number;
    title: string;
    igdbId?: number;
    sourcePath: string;
    installPath?: string;
    status: 'Ready to Install' | 'Installed' | 'Updating' | 'Error';
    description?: string;
    coverUrl?: string;
    bannerUrl?: string;
    releaseDate?: string;
    developer?: string;
    publisher?: string;
    genre?: string;
    themes?: string;
    gameModes?: string;
    tags?: string;
    metacriticScore?: number;
    steamRatingPercent?: number;
    steamRatingText?: string;
    timeToBeat?: number;
    installSize?: number;
}

export interface AppConfig {
    installDirectory: string | null;
}
