<script lang="ts">
  import type { HeapNode } from "$lib/types";
  import TreeNode from "./TreeNode.svelte";

  interface Props {
    node: HeapNode;
    depth?: number;
  }

  let { node, depth = 0 }: Props = $props();

  const initialDepth = depth;
  let expanded = $state(initialDepth < 2); // Auto-expand first 2 levels

  function formatBytes(bytes: number): string {
    if (bytes >= 1024 * 1024 * 1024) {
      return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
    }
    if (bytes >= 1024 * 1024) {
      return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
    }
    if (bytes >= 1024) {
      return `${(bytes / 1024).toFixed(2)} KB`;
    }
    return `${bytes} B`;
  }

  function toggle() {
    if (node.children.length > 0) {
      expanded = !expanded;
    }
  }
</script>

<div class="tree-node">
  <div
    class="node-header"
    class:expandable={node.children.length > 0}
    onclick={toggle}
    onkeydown={(e) => e.key === "Enter" && toggle()}
    role="button"
    tabindex="0"
  >
    {#if node.children.length > 0}
      <span class="toggle">{expanded ? "▼" : "▶"}</span>
    {:else}
      <span class="toggle-placeholder"></span>
    {/if}
    <span class="bytes">{formatBytes(node.bytes)}</span>
    <span class="function">{node.function}</span>
    {#if node.file_info}
      <span class="file">({node.file_info})</span>
    {/if}
  </div>

  {#if expanded && node.children.length > 0}
    <div class="children">
      {#each node.children as child}
        <TreeNode node={child} depth={depth + 1} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .tree-node {
    font-family: "SF Mono", Monaco, "Cascadia Code", monospace;
    font-size: 12px;
  }

  .node-header {
    display: flex;
    align-items: baseline;
    gap: 8px;
    padding: 3px 8px;
    border-radius: 3px;
    cursor: default;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .node-header.expandable {
    cursor: pointer;
  }

  .node-header:hover {
    background: #2a2d2e;
  }

  .toggle {
    width: 12px;
    flex-shrink: 0;
    color: #808080;
    font-size: 10px;
  }

  .toggle-placeholder {
    width: 12px;
    flex-shrink: 0;
  }

  .bytes {
    color: #4ec9b0;
    font-weight: 600;
    min-width: 80px;
    flex-shrink: 0;
  }

  .function {
    color: #dcdcaa;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file {
    color: #6a9955;
    font-size: 11px;
  }

  .children {
    border-left: 1px solid #3c3c3c;
    margin-left: 20px;
  }
</style>
