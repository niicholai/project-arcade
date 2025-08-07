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
    ageRating?: string;
    screenshots?: string; // JSON array of screenshot URLs
    videos?: string; // JSON array of video IDs
    timeToBeat?: number;
    installSize?: number;
}

export interface AppConfig {
    installDirectory: string | null;
}
