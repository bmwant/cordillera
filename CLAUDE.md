# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Cordillera is a Valgrind Massif memory profiler viewer built with Tauri v2 (Rust backend) and SvelteKit (frontend). It parses `.out` files from `valgrind --tool=massif` and displays:
- **Chart tab**: Memory usage over time (Chart.js line chart)
- **Tree tab**: Expandable heap allocation tree for detailed snapshots

## Development Commands

```bash
# Start development (both frontend and Tauri)
npm run tauri dev

# Build for production
npm run tauri build

# Frontend-only development
npm run dev

# Type checking
npm run check
npm run check:watch  # with watch mode

# Build frontend only
npm run build
```

## Architecture

### Frontend (SvelteKit)
- `src/routes/+page.svelte` - Main application page
- `src/lib/components/` - UI components (Tabs, Chart, TreeView, TreeNode)
- `src/lib/types.ts` - TypeScript types for Massif data structures
- Uses Svelte 5 runes (`$state`, `$derived`, `$props`)
- Chart.js for memory visualization
- Dev server runs on port 1420 (required by Tauri)

### Backend (Tauri/Rust)
- `src-tauri/src/lib.rs` - Tauri command handlers and plugin registration
- `src-tauri/src/massif.rs` - Massif file parser (parses `.out` format into JSON)
- `src-tauri/capabilities/default.json` - Tauri permissions (dialog, opener)

### Frontend-Backend Communication
```typescript
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

// Open file picker
const filePath = await open({ filters: [{ name: "Massif", extensions: ["out"] }] });

// Parse massif file via Rust backend
const data = await invoke<MassifData>("parse_massif", { path: filePath });
```

Define Rust commands in `src-tauri/src/lib.rs` with `#[tauri::command]` and register in `invoke_handler`.
