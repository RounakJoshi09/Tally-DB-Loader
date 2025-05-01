Explicit Rules: Project Context, Tech Stack, Consistent

Project Context:
This project aims to deliver a Windows desktop application for loading and synchronizing Tally Prime data into multiple database systems. It is a migration and upgrade of the tally-database-loader (Node.js/TypeScript) (https://github.com/dhananjay1405/tally-database-loader) to a Tauri (Rust + JS/TS) stack. The application will provide a user-friendly GUI, support configuration and logging, and maintain compatibility with all databases and sync modes currently supported.I familiar with Rust and modern frontend frameworks, and the product must be robust, portable, and easy to install for end-users on Windows 10+.

Project Structure:
Frontend and backend code must be clearly separated (src/ for frontend, src-tauri/ for backend).
All configuration files must be at the project root or in src-tauri/ as required.
No business logic should be duplicated between frontend and backend; use Tauri commands for communication.
Keep auto-generated build and output directories (build/, target/, node_modules/) out of version control.
The architecture will be modular, maintainable, and leverage Rust best practices for safety and performance.
