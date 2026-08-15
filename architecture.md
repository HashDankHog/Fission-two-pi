# Overview
Parametrox is written in Rust for the backend and TypeScript for the frontend. Tauri is used to render the UI and manage interaction between the frontend and the backend. 

Beyond Tauri, an app framework; Serde, to serialize json objects; and dyn-clone for cloning trait objects; **all functions have been written entirely by myself. No code has been written by AI** 
## File Structure
Instead of using [Tauri's default file structure](https://v2.tauri.app/start/project-structure/), I opted to implement my own. Parametrox is split into four Cargo crates:
1. ### src:
    * Handles all interactions between the front end 
    and backend. Contains the code to boot Parametrox 
2. ### solver:
    * Math library for solving constraints, computing sweeps/boolean operations, evalutating numerical expressions, etc. 
3. ### app:
    * Split into two parts: a 3D rendering library, and all of the frontend code.
4. ### file: [UNIMPLEMENTED]
    * Handles reading and writing of files.

These crates have their own architecture.md files in case you would like to learn more.
