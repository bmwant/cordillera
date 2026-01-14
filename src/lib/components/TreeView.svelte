<script lang="ts">
  import type { Snapshot } from "$lib/types";
  import TreeNode from "./TreeNode.svelte";

  interface Props {
    snapshots: Snapshot[];
    selectedIndex: number;
    onSelectedIndexChange: (index: number) => void;
  }

  let { snapshots, selectedIndex, onSelectedIndexChange }: Props = $props();

  // Get snapshots that have detailed heap trees
  let detailedSnapshots = $derived(
    snapshots.filter((s) => s.heap_tree !== null)
  );

  let selectedSnapshot = $derived(detailedSnapshots[selectedIndex] || null);

  // Find peak memory snapshot
  let peakSnapshot = $derived(() => {
    if (detailedSnapshots.length === 0) return null;
    return detailedSnapshots.reduce((peak, s) =>
      s.mem_heap_b > peak.mem_heap_b ? s : peak
    , detailedSnapshots[0]);
  });

  let peakMemory = $derived(peakSnapshot() ? peakSnapshot()!.mem_heap_b : 0);

  function handleSelectChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    onSelectedIndexChange(Number(target.value));
  }

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
    <div class="tree-header">
      <div class="peak-info">
        <span class="peak-label">Peak Memory:</span>
        <span class="peak-value">{formatBytes(peakMemory)}</span>
      </div>
      <div class="snapshot-selector">
        <label for="snapshot-select">Snapshot:</label>
        <select id="snapshot-select" value={selectedIndex} onchange={handleSelectChange}>
          {#each detailedSnapshots as snapshot, i}
            <option value={i}>
              #{snapshot.snapshot_num} - {formatBytes(snapshot.mem_heap_b)}
            </option>
          {/each}
        </select>
      </div>
    </div>

    {#if selectedSnapshot?.heap_tree}
      <div class="tree-container">
        <TreeNode node={selectedSnapshot.heap_tree} maxBytes={selectedSnapshot.heap_tree.bytes} />
      </div>
    {/if}
  {/if}
</div>

<style>
  .tree-view {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: #1e1e1e;
  }

  .no-data {
    padding: 32px;
    text-align: center;
    color: #808080;
  }

  .no-data .hint {
    font-size: 13px;
    color: #6a6a6a;
    margin-top: 8px;
  }

  .no-data code {
    background: #2d2d2d;
    padding: 2px 6px;
    border-radius: 4px;
    font-family: "SF Mono", Monaco, "Cascadia Code", monospace;
    color: #ce9178;
  }

  .tree-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid #3c3c3c;
    background: #252526;
  }

  .peak-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .peak-label {
    font-size: 13px;
    color: #808080;
  }

  .peak-value {
    font-size: 14px;
    font-weight: 600;
    color: #4ec9b0;
    font-family: "SF Mono", Monaco, "Cascadia Code", monospace;
  }

  .snapshot-selector {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .snapshot-selector label {
    font-weight: 500;
    color: #cccccc;
    font-size: 13px;
  }

  .snapshot-selector select {
    padding: 6px 12px;
    border: 1px solid #3c3c3c;
    border-radius: 4px;
    font-size: 13px;
    background: #3c3c3c;
    color: #d4d4d4;
    cursor: pointer;
  }

  .snapshot-selector select:hover {
    border-color: #4fc3f7;
  }

  .tree-container {
    flex: 1;
    overflow: auto;
    padding: 8px 0;
  }
</style>
