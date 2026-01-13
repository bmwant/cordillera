<script lang="ts">
  interface Props {
    tabs: string[];
    activeTab: string;
    onTabChange: (tab: string) => void;
  }

  let { tabs, activeTab, onTabChange }: Props = $props();
</script>

<div class="tabs">
  {#each tabs as tab}
    <button
      class="tab"
      class:active={activeTab === tab}
      onclick={() => onTabChange(tab)}
    >
      {tab}
    </button>
  {/each}
</div>

<style>
  .tabs {
    display: flex;
    gap: 4px;
    border-bottom: 1px solid #3c3c3c;
    padding: 0 16px;
    background: #252526;
  }

  .tab {
    padding: 10px 20px;
    border: none;
    background: transparent;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
    color: #808080;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    transition: all 0.15s ease;
  }

  .tab:hover {
    color: #d4d4d4;
    background: #2d2d2d;
  }

  .tab.active {
    color: #4fc3f7;
    border-bottom-color: #4fc3f7;
  }
</style>
