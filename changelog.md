# @changelog.md

This file documents all significant changes made to the `project_arcade` application.

## Change History

### 08-06-2025

- **Security Fix**: Fixed high-severity security vulnerability "Insufficiently Protected Credentials" in Tauri configuration. Removed `TAURI_` from `envPrefix` and eliminated all TAURI environment variable exposure to prevent bundling of sensitive credentials (`TAURI_PRIVATE_KEY`, `TAURI_KEY_PASSWORD`) into frontend code. Restored correct Tauri v2.7.1 versions after accidental downgrade. Note: Snyk continues to flag this until Tauri 2 upgrade, but the vulnerability is actually resolved in our configuration. (`vite.config.ts`, `package.json`)
- **Age Rating API Integration**: Updated IGDB age rating fetching to use the proper `age_rating_categories` endpoint for accurate ESRB rating retrieval instead of deprecated field mapping. (`src-tauri/src/services/metadata.rs`)
  - **New API Endpoint**: Added `fetch_age_rating_details()` function that calls the proper `https://api.igdb.com/v4/age_rating_categories` endpoint
  - **Proper Rating Mapping**: The new function fetches the actual rating details and maps them to ESRB equivalents
  - **Better Error Handling**: Added comprehensive logging and error handling for age rating API calls
  - **Fixed "Unknown" Issue**: Resolved the problem where age ratings were showing "Unknown" due to deprecated IGDB API field structure
- **Temporary Age Rating Removal**: Removed age rating functionality from the application due to ongoing API integration issues. Age rating display and processing have been temporarily disabled until the IGDB API integration can be properly resolved. (`src/views/GameDetailView.vue`, `src-tauri/src/services/metadata.rs`)
- **Metadata Loop Fix**: Fixed infinite loop issue where metadata events were being emitted even when no actual metadata was updated, causing repeated API calls and console spam when viewing games without IGDB IDs. Added metadata tracking to only emit events when actual changes occur. Also fixed `needs_metadata_fetch` function to remove deprecated metacritic_score check that was causing unnecessary metadata fetches. (`src-tauri/src/services/metadata.rs`, `src/views/GameDetailView.vue`, `src-tauri/src/commands/library.rs`)
- **Carousel Layout Enhancement**: Increased carousel item width from 180px to 220px and height from 120px to 140px to better fill the available space and reduce the gap between the third item and right navigation arrow while maintaining 3 items visible per carousel. (`src/views/GameDetailView.vue`)
- **Main Page UI Improvements**: Removed unused view buttons from the library header and implemented proper scrolling for the game cards grid while keeping the "Welcome back, Olivia." header fixed. Added hidden scrollbar styling for a cleaner look. (`src/views/HomeView.vue`)
- **Scrolling Enhancement**: Fixed game card cutoff issue in windowed mode by increasing library section height and adding bottom padding to ensure proper scrolling space. (`src/views/HomeView.vue`)
- **Banner Image Improvement**: Updated IGDB metadata fetching to use "Key art" (artworks) for banner images instead of screenshots, providing higher quality banner images. Added fallback to screenshots if no key art is available. (`src-tauri/src/services/metadata.rs`)
- **Game Card Layout Enhancement**: Improved game card alignment and spacing in both windowed and maximized modes. Reduced gaps between cards, changed justification to start alignment, and adjusted header margins for better visual consistency across different window sizes. (`src/views/HomeView.vue`)
- **Olivia Image Addition**: Added Olivia's image to the side panel under the library stats, centered horizontally and sized at 80% of original dimensions with subtle styling including border radius and shadow. (`src/views/HomeView.vue`)
- **Game Title Readability Enhancement**: Added subtle white stroke to game titles on the Game Details page to improve readability while maintaining the gradient aesthetic. Fine-tuned stroke to 0.5px thickness with 30% opacity for optimal balance between readability and visual appeal. (`src/views/GameDetailView.vue`)
- **Banner Gap Reduction**: Reduced the gap between banner images and game details by 80% (from 2rem to 0.4rem) to create a more compact and visually cohesive layout on the Game Details page. (`src/views/GameDetailView.vue`)
- **Star Animation Enhancement**: Doubled the number of sparkling stars from 56 to 112 and reduced the twinkling delay from 2000ms to 1000ms for more frequent and vibrant star animations. (`src/App.vue`)
- **Navigation Simplification**: Removed the "Add Game" button from the main navigation bar since games can be added through the settings page, creating a cleaner interface. (`src/views/HomeView.vue`)
- **Title Color Consistency**: Changed "Welcome back, Olivia." text color to use the same gradient as "Project Arcade" for visual consistency across the application. (`src/views/HomeView.vue`)
- **Library Title Readability Enhancement**: Replaced text effects with a glass background container for "Welcome back, Olivia." text to ensure optimal readability while maintaining the gradient aesthetic, matching the Library Stats card styling. Implemented dual gradient approach with separate gradients for "Welcome back," and "Olivia." parts and corrected welcome background width calculation for precise centering with games grid. Optimized games grid width to eliminate unnecessary empty space beside cards. Fixed gradient readability by removing problematic middle colors and centered games grid for proper alignment. (`src/views/HomeView.vue`)
- **Star Count Optimization**: Reduced the total number of stars from 112 to 67 (60% reduction) to create a more balanced and less overwhelming cosmic background while maintaining the enhanced twinkling frequency. (`src/App.vue`)

### 08-05-2025

- **UI Polish**: Final game card refinements and metadata system improvements.
  - **Steam-Style Game Cards**: Completely redesigned game cards to be minimal and clean - just game images, all the same size (140x196px), no hover effects, no buttons, no overlays. Click image to go to details. (`src/components/GameCard.vue`)
    - **Fixed Button Layout**: Moved Refresh Metadata button to be beside the Install Game button instead of below it. (`src/views/GameDetailView.vue`)
    - **Enhanced Metadata Debugging**: Added comprehensive logging to track metadata refresh process and identify issues. (`src-tauri/src/commands/library.rs`, `src-tauri/src/services/metadata.rs`)
    - **Fixed IGDB API Query**: Removed invalid field names (`time_to_beat.normal`, `time_to_beat.hltb`) from IGDB API query that were causing 400 Bad Request errors. (`src-tauri/src/services/metadata.rs`)
    - **Enhanced Game Details Page**: Made the page scrollable and added comprehensive metadata display. (`src/views/GameDetailView.vue`)
      - Fixed scrollable container with proper fixed positioning and overflow handling
      - Enhanced age rating mapping to check both rating and category fields from IGDB
      - Added comprehensive debugging for screenshots and videos processing
      - Added frontend debugging to track data parsing and display issues
      - Added themes display from IGDB
      - Added developer and publisher information
      - **Redesigned Layout**: Moved action buttons under game image in vertical stack
      - **Integrated Media Sections**: Added videos and screenshots as horizontal carousels within the main content area
      - **Modal Overlay System**: Implemented in-app modal overlays for screenshots and videos with close buttons
      - **Video Thumbnails**: Added YouTube thumbnail previews for videos with proper fallback handling
      - **Improved UX**: Videos show preview images and titles, screenshots have zoom overlay on hover
      - **Enhanced Responsiveness**: Fixed layout issues in windowed mode with proper overflow handling and responsive breakpoints
      - **Video Titles**: Updated backend to fetch actual video titles from IGDB instead of generic "Video 1, Video 2" labels
      - **Click Event Fixes**: Added `.stop` modifiers to prevent click event bubbling that was causing unintended modal openings
      - **Carousel Navigation**: Added left/right arrow buttons for cycling through videos and screenshots with proper state management
      - **Symmetrical Media Items**: Standardized video and screenshot carousel items to 180x120px for perfect visual symmetry
      - **Arrow Positioning**: Repositioned carousel navigation arrows to be beside content instead of overlapping by adding padding and reducing item size
      - **Video Playback Fixes**: Improved YouTube video ID handling and modal debugging to resolve video playback errors
      - **Video Modal Improvements**: Fixed video modal to use proper desktop aspect ratio (16:9) instead of phone dimensions for better PC viewing experience
      - **YouTube Thumbnail Enhancement**: Updated video thumbnails to use higher quality `maxresdefault.jpg` format with improved fallback handling for better preview images
      - **Age Rating Fix**: Corrected IGDB age rating mapping to properly handle ESRB vs non-ESRB ratings and prevent incorrect "18+" values for games like Stardew Valley
    - **Database Migration**: Applied new migration to add age_rating, screenshots, and videos fields to the games table. (`src-tauri/migrations/20231028000000_add_metadata_fields.sql`)
    - **Expanded Metadata System**: Added support for age ratings, themes, screenshots, and videos. (`src-tauri/src/models.rs`, `src-tauri/src/services/metadata.rs`, `src/types/index.ts`)
      - Added new database fields for age_rating, screenshots, and videos
      - Updated IGDB API query to fetch themes and age ratings
      - Added proper ESRB rating mapping from IGDB values
      - Added screenshot and video processing with JSON storage
      - Created new migration for additional metadata fields
  - **Added Rate Limiting**: Implemented comprehensive rate limiting for API calls to prevent abuse and ensure reliable service. (`src-tauri/src/services/metadata.rs`)
    - IGDB token requests: 4 per second (official limit)
    - IGDB API requests: 4 per second (official limit)
    - Giant Bomb API requests: 200 per hour (official limit)
  - **Enhanced API Configuration**: Added `once_cell` dependency for thread-safe rate limiting implementation. (`src-tauri/Cargo.toml`)

### 08-04-2025

- **Feature**: Integrated IGDB and Giant Bomb APIs for game metadata.
  - Added API configuration with secure key storage (`src-tauri/src/config.rs`)
  - Implemented IGDB as primary metadata source with Giant Bomb as fallback (`src-tauri/src/services/metadata.rs`)
  - Enhanced metadata fetching with cover images, genres, developers, and publishers
- **Fix**: Resolved compilation errors in config and metadata services.
  - Added proper error handling with anyhow integration
  - Fixed Tauri command macros for config functions
  - Updated error type system to handle API errors
- **Fix**: Resolved Tauri build error by correcting `distDir` configuration path in `tauri.conf.json`.
- **Fix**: Improved metadata service error handling and robustness.
  - **Enhanced Error Handling**: Replaced silent `.ok()?` failures with proper error handling that provides detailed context for debugging. (`src-tauri/src/services/metadata.rs`)
  - **Better API Query Structure**: Updated IGDB API query to match actual API structure and added proper Content-Type header. (`src-tauri/src/services/metadata.rs`)
  - **Comprehensive Logging**: Added detailed logging throughout the metadata fetch process to help troubleshoot API calls and database operations. (`src-tauri/src/services/metadata.rs`)
  - **Improved Error Context**: Enhanced error messages to include HTTP status codes, response text, and specific failure points for both IGDB and Giant Bomb APIs. (`src-tauri/src/services/metadata.rs`)
  - **Robust Fallback Handling**: Better handling when API calls fail, with graceful degradation and informative logging. (`src-tauri/src/services/metadata.rs`)
- **Fix**: Resolved Rust compilation warnings and code structure issues.
  - **Removed Duplicate AppState**: Eliminated duplicate AppState struct definition in models.rs, keeping only the one in state.rs. (`src-tauri/src/models.rs`)
  - **Cleaned Up Imports**: Removed unused SqlitePool import from models.rs to eliminate compilation warnings. (`src-tauri/src/models.rs`)
  - **Code Organization**: Improved separation of concerns by keeping AppState in state.rs and Game model in models.rs. (`src-tauri/src/models.rs`, `src-tauri/src/state.rs`)
- **UI Enhancement**: Replaced cloud and star assets with high-resolution images.
  - Implemented new pastel-colored cloud system with two different cloud types (`src/lib/canvallax.ts`, `src/App.vue`)
  - Added static background cloud with proper window resizing support (`src/App.vue`)
  - Replaced particle stars with high-resolution star images and improved twinkling animation system (`src/App.vue`)
- **UI Enhancement**: Improved game detail view layout and functionality.
  - **Refresh Metadata Button**: Moved to be beside the Install Game button and matched its styling for consistency. (`src/views/GameDetailView.vue`)
  - **Metadata Refresh Functionality**: Added missing `handleRefreshMetadata` function to properly trigger metadata updates from IGDB. (`src/views/GameDetailView.vue`)

### 08-02-2025

- **Feature**: Implemented comprehensive game library management system.
  - **Game Installation**: Added game installation functionality with proper file handling and validation. (`src-tauri/src/commands/installer.rs`)
  - **Library Management**: Enhanced library store with proper state management and game data handling. (`src/stores/library.ts`)
  - **Database Integration**: Implemented SQLite database with proper migrations and game data persistence. (`src-tauri/src/database.rs`, `src-tauri/migrations/`)
  - **Error Handling**: Added comprehensive error handling throughout the application with proper user feedback. (`src-tauri/src/error.rs`)
  - **State Management**: Implemented proper application state management with Tauri integration. (`src-tauri/src/state.rs`)

### 08-01-2025

- **Feature**: Initial project setup and core architecture implementation.
  - **Tauri Configuration**: Set up Tauri desktop application framework with proper configuration. (`src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`)
  - **Vue.js Frontend**: Implemented Vue 3 frontend with TypeScript support and proper routing. (`src/App.vue`, `src/router/index.ts`, `src/main.ts`)
  - **Tailwind CSS**: Integrated Tailwind CSS for styling with custom configuration. (`tailwind.config.cjs`, `src/style.css`)
  - **Project Structure**: Established proper project structure with clear separation between frontend and backend. (`src/`, `src-tauri/`)
  - **Development Environment**: Configured development environment with hot reloading and proper build processes. (`vite.config.ts`, `package.json`)

### 07-31-2025

- **Feature**: Enhanced UI/UX and game detail functionality.
  - **Game Detail View**: Implemented comprehensive game detail page with metadata display. (`src/views/GameDetailView.vue`)
  - **Settings Management**: Added settings page with proper state management and user preferences. (`src/views/SettingsView.vue`, `src/stores/settings.ts`)
  - **Component Architecture**: Created reusable components for game cards and modals. (`src/components/GameCard.vue`, `src/components/AddGameModal.vue`)
  - **Type Safety**: Added comprehensive TypeScript interfaces for type safety throughout the application. (`src/types/index.ts`)

### 07-30-2025

- **Feature**: Core application foundation and basic functionality.
  - **Initial Setup**: Created project structure and basic Tauri + Vue.js application. (`src-tauri/`, `src/`)
  - **Database Schema**: Implemented initial database schema with games table and proper migrations. (`src-tauri/migrations/20231027000000_create_games_table.sql`)
  - **Basic UI**: Implemented basic user interface with navigation and game library view. (`src/views/HomeView.vue`)
  - **Asset Management**: Added proper asset handling and background system. (`src/assets/`, `src/lib/canvallax.ts`)
