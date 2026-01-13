<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import Tabs from "$lib/components/Tabs.svelte";
  import Chart from "$lib/components/Chart.svelte";
  import TreeView from "$lib/components/TreeView.svelte";
  import type { MassifData } from "$lib/types";

  let massifData: MassifData | null = $state(null);
  let loading = $state(false);
  let error = $state("");
  let activeTab = $state("Chart");

  const tabs = ["Chart", "Tree"];

  async function openFile() {
    try {
      error = "";
      const filePath = await open({
        filters: [{ name: "Massif Output", extensions: ["out"] }],
      });

      if (!filePath) return;

      loading = true;
      massifData = await invoke<MassifData>("parse_massif", {
        path: filePath,
      });
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      massifData = null;
    } finally {
      loading = false;
    }
  }

  function handleTabChange(tab: string) {
    activeTab = tab;
  }
</script>

<main class="app">
  <header class="header">
    <h1>Cordillera</h1>
    <button class="open-btn" onclick={openFile} disabled={loading}>
      {loading ? "Loading..." : "Open File"}
    </button>
  </header>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if massifData}
    <div class="info">
      <span class="cmd" title={massifData.cmd}>{massifData.cmd}</span>
      <span class="stats">
        {massifData.snapshots.length} snapshots
      </span>
    </div>

    <Tabs {tabs} {activeTab} onTabChange={handleTabChange} />

    <div class="content">
      {#if activeTab === "Chart"}
        <Chart snapshots={massifData.snapshots} timeUnit={massifData.time_unit} />
      {:else if activeTab === "Tree"}
        <TreeView snapshots={massifData.snapshots} />
      {/if}
    </div>
  {:else if !loading}
    <div class="empty">
      <p>Open a Massif output file to visualize memory profiling data.</p>
      <p class="hint">
        Generate with: <code>valgrind --tool=massif ./your-program</code>
      </p>
    </div>
  {/if}
</main>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid #e0e0e0;
    background: #fafafa;
  }

  .header h1 {
    font-size: 18px;
    font-weight: 600;
    margin: 0;
    color: #333;
  }

  .open-btn {
    padding: 8px 16px;
    background: #1976d2;
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .open-btn:hover:not(:disabled) {
    background: #1565c0;
  }

  .open-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .error {
    padding: 12px 16px;
    background: #ffebee;
    color: #c62828;
    border-bottom: 1px solid #ef9a9a;
  }

  .info {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    background: #e3f2fd;
    font-size: 13px;
  }

  .info .cmd {
    color: #1565c0;
    font-family: monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 70%;
  }

  .info .stats {
    color: #666;
    flex-shrink: 0;
  }

  .content {
    flex: 1;
    overflow: auto;
  }

  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #666;
    text-align: center;
    padding: 32px;
  }

  .empty p {
    margin: 8px 0;
  }

  .empty .hint {
    font-size: 13px;
    color: #888;
  }

  .empty code {
    background: #f5f5f5;
    padding: 4px 8px;
    border-radius: 4px;
    font-family: monospace;
  }

  @media (prefers-color-scheme: dark) {
    .header {
      background: #1a1a1a;
      border-bottom-color: #333;
    }

    .header h1 {
      color: #e0e0e0;
    }

    .error {
      background: #4a1c1c;
      color: #ef9a9a;
      border-bottom-color: #6d2f2f;
    }

    .info {
      background: #1a2733;
    }

    .info .cmd {
      color: #64b5f6;
    }

    .info .stats {
      color: #aaa;
    }

    .empty {
      color: #aaa;
    }

    .empty .hint {
      color: #777;
    }

    .empty code {
      background: #333;
      color: #e0e0e0;
    }
  }
</style>
