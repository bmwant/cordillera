<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { listen } from "@tauri-apps/api/event";
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

  onMount(() => {
    const unlisten = listen("menu-open-file", () => {
      openFile();
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  });
</script>

<main class="app">
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
      <p class="hint">Use File menu or press <kbd>Cmd+O</kbd> to open a file</p>
      <p class="hint">
        Generate with: <code>valgrind --tool=massif ./your-program</code>
      </p>
    </div>
  {:else}
    <div class="empty">
      <p>Loading...</p>
    </div>
  {/if}
</main>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    width: 100%;
    background: #1e1e1e;
    color: #d4d4d4;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, sans-serif;
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    background: #1e1e1e;
  }

  .error {
    padding: 12px 16px;
    background: #5c2d2d;
    color: #f48771;
    border-bottom: 1px solid #6d3b3b;
  }

  .info {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    background: #252526;
    font-size: 13px;
    border-bottom: 1px solid #3c3c3c;
  }

  .info .cmd {
    color: #9cdcfe;
    font-family: "SF Mono", Monaco, "Cascadia Code", monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 70%;
  }

  .info .stats {
    color: #808080;
    flex-shrink: 0;
  }

  .content {
    flex: 1;
    overflow: auto;
    background: #1e1e1e;
  }

  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #808080;
    text-align: center;
    padding: 32px;
  }

  .empty p {
    margin: 8px 0;
  }

  .empty .hint {
    font-size: 13px;
    color: #6a6a6a;
  }

  .empty code {
    background: #2d2d2d;
    padding: 4px 8px;
    border-radius: 4px;
    font-family: "SF Mono", Monaco, "Cascadia Code", monospace;
    color: #ce9178;
  }

  .empty kbd {
    background: #2d2d2d;
    padding: 2px 6px;
    border-radius: 3px;
    border: 1px solid #3c3c3c;
    font-family: "SF Mono", Monaco, "Cascadia Code", monospace;
    font-size: 12px;
    color: #d4d4d4;
  }
</style>
