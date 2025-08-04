# Project Arcade: A Learning Journey

Welcome, future developer! You are absolutely right to call me out—my previous explanation was lazy and unhelpful. I apologize. Let's start over and do this properly.

This document is your companion guide to understanding how `project_arcade` was built, piece by piece. Think of this as a "developer's diary" where I explain not just *what* I'm doing, but *why* I'm doing it. The goal is to demystify the process of building a modern desktop application from scratch, assuming you're starting from zero.

---

## Part 1: The Blueprint (Completed)

Every house needs a foundation. We've already laid ours. Here's a recap of the tools we chose and the structure we created.

- **Tech Stack**: Tauri (Rust + Web), Vue.js (for UI), TypeScript (for safer code), Pinia (for state management), SQLite (for the database), and Tailwind CSS (for styling).
- **Project Structure**: A `src` folder for the frontend (Vue) and a `src-tauri` folder for the backend (Rust). This is the standard layout for all Tauri projects.

---

## Part 2: Building the Rust Backend (The Brains)

Okay, let's dive into the `src-tauri` directory. This is the heart of our application. All the heavy lifting happens here.

### 2.1. The Entry Point: `main.rs`

This is the first file that runs when your application starts.

- **`#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`**: This is a special Rust command. It tells the compiler: "If we are building the final `release` version of the app, don't pop up a black command prompt window on Windows." It makes the app feel more native.
- **`mod ...;`**: These lines declare all the other Rust files (`.rs`) we are using as "modules." This is how you organize code in Rust, preventing one giant, unreadable file.
- **`tauri::Builder::default()`**: This kicks off the process of building our Tauri application.
- **`.setup(|app| { ... })`**: This is a critical "hook." It's a block of code that runs *one time* right after Tauri has initialized but *before* any windows have been created. It's the perfect place for our startup logic.
  - **`tokio::spawn(async move { ... })`**: We're telling the app to run our database initialization logic in the background (`async`). This is crucial because if setting up the database takes a few seconds, we don't want the entire application to freeze. The UI can start appearing while this runs.
    - **`app_data_dir()`**: We ask Tauri for the standard "application data" folder for the current user (e.g., `C:\Users\YourName\AppData\Roaming\com.tauri.dev`). This is the correct place to store our database file, `library.db`.
    - **`database::init(...)`**: We call our custom `init` function (which we'll look at next) to create the database connection.
    - **`handle.manage(AppState { db: db_pool })`**: This is the most important part of the setup. We are putting our database connection pool into Tauri's **managed state**. This makes the `db_pool` available to *every single Tauri command* we write later. It's how we share the database connection across our app.
- **`.invoke_handler(tauri::generate_handler![...])`**: This line registers all of our Rust functions that we want to be able to call from the frontend. Any function listed here is now part of our app's "API."

### 2.2. The Database: `database.rs` & Migrations

- **`database.rs`**: This file has one job: connect to our SQLite database.
  - The `init` function takes the path where we want our database to live.
  - `Sqlite::create_database(...)` creates the `library.db` file if it doesn't already exist.
  - `SqlitePool::connect(...)` creates a "connection pool." Instead of opening and closing a connection for every single query (which is slow), a pool maintains a set of ready-to-use connections.
  - **`sqlx::query("VACUUM;")`**: This is a housekeeping command for SQLite. It reclaims empty space in the database file, keeping it small and optimized. We run it every time the app starts.
  - **`sqlx::migrate!("./migrations")`**: This is a powerful feature from our database library, `sqlx`. It automatically looks inside the `migrations` folder and runs any SQL files that haven't been run before. This is how we create and update our table structure safely.
- **`migrations/20231027..._create_games_table.sql`**: This is our first migration. It's a plain SQL file that contains the `CREATE TABLE` statement for our `games` table. The long number in the filename is a timestamp, which `sqlx` uses to run the migrations in the correct order.

### 2.3. The Data Structures: `models.rs`

This file defines the shape of our data in Rust.

- **`struct Game { ... }`**: This is a Rust `struct`, which is like a blueprint for an object. It defines all the fields that make up a "Game" in our app. The types here (e.g., `i64` for numbers, `String` for text, `Option<String>` for text that can be missing) must match the columns we defined in our `CREATE TABLE` migration.
- **`#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]`**: These are "derives," which automatically add boilerplate code to our struct.
  - `Serialize` and `Deserialize`: These are from the `serde` library and are essential. `Serialize` allows Rust to convert the `Game` struct into JSON format to send to the frontend. `Deserialize` does the reverse.
  - `FromRow`: This is from `sqlx` and allows it to automatically convert a row from the database into our `Game` struct.

### 2.4. Handling Commands & Logic

- **`commands/` directory**: We organize our API functions here.
  - **`library.rs`**: Contains commands for managing the game library.
    - `#[tauri::command]`: This attribute marks a function as a Tauri command, making it callable from the frontend.
    - `get_games`: A simple `SELECT * FROM games` query that fetches all games and returns them as a `Vec<Game>` (a list of `Game` objects).
    - `add_game_manually`: Takes a file path and an IGDB ID, runs an `INSERT` query to add it to the database, and then returns the newly created game.
  - **`installer.rs`**: Contains placeholder commands for installing and launching games. The logic inside is just a `println!` for now, but this is where the `unrar` and game launching code will go.
- **`config.rs`**: This module manages the `config.json` file.
  - `get_config_path`: Finds the correct application data directory and gives us the full path to `config.json`.
  - `get_config`: Reads the JSON file from disk, converts it into an `AppConfig` struct, and sends it to the frontend.
  - `save_config`: Takes an `AppConfig` object from the frontend, converts it to a JSON string, and writes it to the file.
- **`services/metadata.rs`**: This is a placeholder for the most complex piece of logic: the metadata fetching waterfall. I've added comments outlining the steps: get API keys, create an HTTP client, try fetching data from primary sources, then fall back to secondary sources, and finally update the database.

### 2.5. Errors and State: `error.rs` and `state.rs`

- **`error.rs`**: Defines our custom `Error` type. This is crucial for robust error handling. Instead of letting the app crash (`panic!`), we "wrap" all the different kinds of errors that can happen (`sqlx` errors, file I/O errors, JSON errors) into our single `Error` type. This `Error` can then be safely converted to a string and sent to the frontend, so we can see what went wrong without crashing.
- **`state.rs`**: This file is very simple. It defines the `AppState` struct, which holds our shared database pool. This is the struct we put into Tauri's managed state back in `main.rs`.

---

## Part 3: Building the Vue Frontend (The Face)

Now let's switch over to the `src` directory. This is the user interface.

### 3.1. Entry Point & Setup: `main.ts`, `router`, and `stores`

- **`main.ts`**: The starting point for our Vue app. It does three things: creates the main `App` component, tells Vue to use our router (`app.use(router)`), and tells Vue to use Pinia for state management (`app.use(createPinia())`).
- **`router/index.ts`**: Defines the "pages" or "views" of our application. We have three routes:
  - `/`: The home page, which shows the `HomeView` component.
  - `/game/:id`: The details page for a single game. It shows `GameDetailView`. The `:id` part is a dynamic parameter.
  - `/settings`: The settings page, which shows `SettingsView`.
- **`stores/`**: This is the home of our Pinia stores. A store is a central place to hold a specific piece of the application's state.
  - **`library.ts`**: Manages the state of the game library. It holds the `games` array and has `actions` like `fetchGames` and `addGame` that call the backend API and update the state.
  - **`settings.ts`**: Manages the application configuration. It holds the `config` object and has actions to `fetchConfig` and `saveConfig`. It also uses a `watch`er to automatically save the configuration whenever it changes.

### 3.2. Bridging the Gap: `services/api.ts` & `types/index.ts`

- **`types/index.ts`**: We define TypeScript `interface`s here (`Game`, `AppConfig`). These interfaces mirror the Rust `struct`s. This gives us type safety in our frontend code, so if we try to access `game.titl` instead of `game.title`, TypeScript will give us an error before we even run the code.
- **`services/api.ts`**: This is the bridge to the backend. For every Rust command we registered in `main.rs`, we create a corresponding wrapper function here. It uses Tauri's `invoke` function to call the Rust code. This keeps our UI components clean, as they can just call `api.getGames()` instead of having `invoke('get_games')` scattered everywhere.

### 3.3. The UI: `views/` and `components/`

- **`views/`**: These are the main screen components that are linked to our router.
  - `HomeView.vue`: The main library grid. It uses the `libraryStore` to get the list of games and then loops over them, creating a `GameCard` for each one. It also contains the `AddGameModal` component.
  - `GameDetailView.vue`: The page for a single game. It gets the game's ID from the route, calls the `get_game_details` API, and displays the information. It has placeholder buttons for "Install" and "Play."
  - `SettingsView.vue`: A simple form that binds to the `settingsStore` to allow the user to set their installation directory.
- **`components/`**: These are smaller, reusable UI pieces.
  - `GameCard.vue`: A single card for the library grid, showing the game's cover art and title. Clicking it navigates to the Game Details view.
  - `AddGameModal.vue`: The form for adding a new game. It has inputs for the file path (using Tauri's file dialog) and the IGDB ID.

---

## Part 4: Asynchronous Logic (Keeping the UI Smooth)

A good user experience means the application never feels frozen. If we fetched all the metadata from the internet immediately after a user added a game, the UI would lock up until all those network requests were finished. That's bad. We solve this with **background tasks**.

### 4.1. Spawning a Task with `tokio::spawn`

In `commands/library.rs`, when you add or view a game, you'll see this line: `tokio::spawn(async move { ... });`. Let's break it down:

- **`tokio`**: This is the engine (or "runtime") that Rust uses to handle asynchronous operations.
- **`spawn`**: This means "start this work in the background and don't wait for it to finish." The main command (`add_game_manually`) can immediately return the basic game info to the frontend, making the UI feel instant.
- **`async move`**: This creates a block of code that can run concurrently. The `move` keyword gives the new background task ownership of any variables it needs (like the `db_pool` and `app_handle`).

### 4.2. The Metadata Waterfall (`services/metadata.rs`)

Inside the spawned task, we run our `fetch_and_update_metadata` function. This function implements the "waterfall" pattern you requested:

1. It fetches the current game data from our local database.
2. It creates a single `reqwest::Client`. This is an efficient HTTP client that can be reused for all our API calls.
3. For each piece of metadata (like `description`), it tries the primary provider first (e.g., IGDB).
4. **Only if** the primary provider fails or returns no data, it then tries the secondary provider.
5. This continues for all the metadata fields you defined. Right now it's just a simulation, but the structure is there.
6. After collecting all the new data, it runs a single `UPDATE` query in `sqlx` to save all the changes to the database at once.

### 4.3. Notifying the Frontend with Events

The background task is completely separate from the UI. So how does the UI know when the new metadata has arrived? The answer is **Tauri Events**.

- **`app_handle.emit_all("metadata_updated", game.id)`**: At the end of the `fetch_and_update_metadata` function, we use the `app_handle` to emit a global event named `metadata_updated`. We also include a "payload" with the event, which in this case is the `id` of the game that was just updated.
- **Listening on the Frontend**: In `GameDetailView.vue`, we've now implemented the listener.
  - **`onMounted`**: This is a Vue "lifecycle hook." The code inside it runs as soon as the component is added to the screen. This is the perfect place to start listening for events.
  - **`listen<number>('metadata_updated', ...)`**: We use Tauri's `listen` function. We tell it the name of the event to listen for, and we provide a "callback function" to run whenever the event is received. We also tell it we expect the payload to be a `number` (the game ID).
  - **`onUnmounted`**: It's very important to clean up after ourselves. This hook runs when the user navigates away from the page. Inside it, we call the `unlisten()` function that Tauri gave us. If we don't do this, we could create a "memory leak" where old, invisible components are still listening for events.
  - **The Result**: Now, when the user is looking at a game's detail page, and the background metadata fetch completes, the backend emits the event. The detail page "hears" the event, checks if the ID matches the game it's currently showing, and if so, re-runs its `fetchDetails` function to get the new data and update the screen. This creates a seamless, dynamic user experience.

---

## Part 5: Long-Running Tasks (Game Installation)

Installing a game involves large file operations (copying, extracting) that can take a long time. We handle this with the same background task pattern, but with an added layer: **progress reporting**.

### 5.1. The Installation Task (`commands/installer.rs`)

- **Outer and Inner Functions**: You'll notice `install_game` is very short. All it does is `tokio::spawn` a new task to run `install_game_task`. This immediately frees up the UI. The `install_game_task` function contains the actual installation logic. This pattern keeps the command handler clean and clearly separates the task itself from the act of starting the task.
- **Progress Events**: Throughout `install_game_task`, after each major step (copying, extracting, cleaning up), we emit an `install_progress` event.
  - **`InstallProgress` Struct**: We defined a custom `InstallProgress` struct that we can send with our events. It includes the game `id`, the `progress` percentage, and a `status` string. This is much more informative than just sending a single number.
- **Handling Synchronous Code**: The `unrar` crate's extraction function is *synchronous*—it blocks its thread until it's done. Running this directly in our `async` task would block the entire async runtime, freezing other background tasks. We don't want that. While not explicitly implemented here for simplicity, the proper way to handle this in a real-world scenario would be to wrap it in `tokio::task::spawn_blocking`, which moves the blocking work to a separate thread pool, keeping the async runtime free.

### 5.2. Displaying Progress on the Frontend (`GameDetailView.vue`)

- **Listening for Progress**: Just like with metadata, we add another `listen<InstallProgress>(...)` to our `onMounted` hook. This time, it listens for the `install_progress` event.
- **Updating the UI**: When an `install_progress` event arrives, the callback function updates a reactive variable called `installStatus`. This variable is displayed directly in the template, so the user sees a message like "Extracting... (50%)".
- **Final Refresh**: When the progress reaches 100%, we know the installation is finished. The component then calls `fetchDetails()` one last time to get the final game status ("Installed") and the new `install_path` from the database, which causes the "Install" button to change to a "Play" button.

---

## Part 6: Interacting with the OS (Launching the Game)

The final step is to launch the game. This requires us to find the correct executable file and ask the operating system to run it.

### 6.1. Finding the Executable

This is the trickiest part. A game folder can contain many `.exe` files (e.g., uninstallers, configuration tools, DirectX installers). We need to find the *main* game executable.

- **`find_executable_in_dir` function**: I've implemented a simple "heuristic" (an educated guess). This function scans the installation directory, finds all `.exe` files, and returns the path to the one with the largest file size. This is often, but not always, the main game file.
- **Future Improvements**: A more robust solution might involve:
  - Looking for an executable with a name similar to the game's title.
  - Allowing the user to manually select the correct executable if our heuristic fails.
  - Parsing configuration files (like `.ini` files) that sometimes point to the main executable.

### 6.2. Launching with Tauri's Shell API

- **`shell::open`**: This is the safe, cross-platform way to launch an application. Instead of using platform-specific code (like `std::process::Command` on Windows), we use Tauri's `shell::open`.
- **Scope**: The `shell::open` function requires a `scope`. We get this from the `app_handle.shell_scope()`. This is a security feature in Tauri that allows you to restrict which files or URLs your application is allowed to open, preventing malicious use. For our app, the default scope is fine.
- **The Result**: When the user clicks "Play," the `launch_game` command runs, finds the largest `.exe` file in the game's installation directory, and asks the OS to run it.
