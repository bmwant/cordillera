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
    border-bottom: 1px solid #e0e0e0;
    padding: 0 16px;
  }

  .tab {
    padding: 12px 24px;
    border: none;
    background: transparent;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    color: #666;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    transition: all 0.2s ease;
  }

  .tab:hover {
    color: #333;
    background: #f5f5f5;
  }

  .tab.active {
    color: #1976d2;
    border-bottom-color: #1976d2;
  }

  @media (prefers-color-scheme: dark) {
    .tabs {
      border-bottom-color: #444;
    }

    .tab {
      color: #aaa;
    }

    .tab:hover {
      color: #fff;
      background: #333;
    }

    .tab.active {
      color: #64b5f6;
      border-bottom-color: #64b5f6;
    }
  }
</style>
