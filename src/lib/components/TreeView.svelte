<script lang="ts">
  import type { Snapshot } from "$lib/types";
  import TreeNode from "./TreeNode.svelte";

  interface Props {
    snapshots: Snapshot[];
  }

  let { snapshots }: Props = $props();

  // Get snapshots that have detailed heap trees
  let detailedSnapshots = $derived(
    snapshots.filter((s) => s.heap_tree !== null)
  );

  let selectedIndex = $state(0);

  let selectedSnapshot = $derived(detailedSnapshots[selectedIndex] || null);

  function formatBytes(bytes: number): string {
    if (bytes >= 1024 * 1024) {
      return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
    }
    if (bytes >= 1024) {
      return `${(bytes / 1024).toFixed(2)} KB`;
    }
    return `${bytes} B`;
  }
</script>

<div class="tree-view">
  {#if detailedSnapshots.length === 0}
    <div class="no-data">
      <p>No detailed heap trees available.</p>
      <p class="hint">
        Detailed heap trees are only recorded for peak snapshots. Run massif
        with <code>--detailed-freq=1</code> for more detailed data.
      </p>
    </div>
  {:else}
    <div class="snapshot-selector">
      <label for="snapshot-select">Snapshot:</label>
      <select id="snapshot-select" bind:value={selectedIndex}>
        {#each detailedSnapshots as snapshot, i}
          <option value={i}>
            #{snapshot.snapshot_num} - {formatBytes(snapshot.mem_heap_b)}
          </option>
        {/each}
      </select>
    </div>

    {#if selectedSnapshot?.heap_tree}
      <div class="tree-container">
        <TreeNode node={selectedSnapshot.heap_tree} />
      </div>
    {/if}
  {/if}
</div>

<style>
  .tree-view {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .no-data {
    padding: 32px;
    text-align: center;
    color: #666;
  }

  .no-data .hint {
    font-size: 13px;
    color: #888;
    margin-top: 8px;
  }

  .no-data code {
    background: #f5f5f5;
    padding: 2px 6px;
    border-radius: 4px;
    font-family: monospace;
  }

  .snapshot-selector {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-bottom: 1px solid #e0e0e0;
    background: #fafafa;
  }

  .snapshot-selector label {
    font-weight: 500;
    color: #555;
  }

  .snapshot-selector select {
    padding: 6px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 14px;
    background: white;
    cursor: pointer;
  }

  .tree-container {
    flex: 1;
    overflow: auto;
    padding: 8px 0;
  }

  @media (prefers-color-scheme: dark) {
    .no-data {
      color: #aaa;
    }

    .no-data .hint {
      color: #777;
    }

    .no-data code {
      background: #333;
      color: #e0e0e0;
    }

    .snapshot-selector {
      background: #2a2a2a;
      border-bottom-color: #444;
    }

    .snapshot-selector label {
      color: #bbb;
    }

    .snapshot-selector select {
      background: #333;
      border-color: #555;
      color: #e0e0e0;
    }
  }
</style>
