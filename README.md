# Minimal Atmosphere Client Base

## Overview

This project serves as a minimal foundation for building more complex Atmosphere (Bluesky) clients using Tauri, React, and Rust. It provides a starting point with essential features for developers looking to create custom experiences on the AT Protocol.

## Core Features

- Feed Handling
- Posting Capabilities
- Content Labeling
- Account Management & Views

## Getting Started

### Prerequisites

- Node.js (latest LTS version recommended)
- Rust (stable version)
- Tauri prerequisites: Follow the official guide at [https://tauri.app/v1/guides/getting-started/prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation and Running

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/yourproject.git
   ```
2. Navigate to the project directory:
   ```bash
   cd <project-directory>
   ```
3. Install frontend dependencies:
   ```bash
   npm install
   ```
4. Run the development server:
   ```bash
   npm run tauri dev
   ```

## Project Structure

- `src/`: Contains the React frontend application code (TypeScript, TSX, CSS).
    - `src/pages/`: Houses the different page components of the application (e.g., login, home).
    - `src/assets/`: Stores static assets like images and icons.
- `src-tauri/`: Contains the Rust backend code for the Tauri application.
    - `src-tauri/src/main.rs`: The main entry point for the Rust application.
    - `src-tauri/tauri.conf.json`: Tauri application configuration file.
- `public/`: Public assets that are copied to the build output.

## Contributing

We welcome contributions to improve and expand this minimal client base!

- Feel free to fork this repository and submit pull requests.
- If you encounter any bugs or have feature suggestions, please open an issue on the GitHub repository.
- Ensure your code follows the existing style and structure where possible.

##About AI
This README was written with Ai because I wanted to test jules from google. All of the actual code was written by me because this is a learning project and vibe coding won't teach me react. 