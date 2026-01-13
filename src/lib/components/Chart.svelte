<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    Chart,
    LineController,
    LineElement,
    PointElement,
    LinearScale,
    Title,
    Tooltip,
    Legend,
    Filler,
  } from "chart.js";
  import type { Snapshot } from "$lib/types";

  Chart.register(
    LineController,
    LineElement,
    PointElement,
    LinearScale,
    Title,
    Tooltip,
    Legend,
    Filler
  );

  interface Props {
    snapshots: Snapshot[];
    timeUnit: string;
  }

  let { snapshots, timeUnit }: Props = $props();

  let canvas: HTMLCanvasElement;
  let chart: Chart | null = null;

  function formatTime(time: number): number {
    // If time_unit is 'i' (instructions), convert to approximate seconds
    // Assuming roughly 1 billion instructions per second as rough estimate
    if (timeUnit === "i") {
      return time / 1_000_000_000;
    }
    // If time_unit is 'ms', convert to seconds
    if (timeUnit === "ms") {
      return time / 1000;
    }
    // If time_unit is 'B' (bytes), just return as-is
    return time;
  }

  function formatBytes(bytes: number): number {
    return bytes / (1024 * 1024); // Convert to MB
  }

  function createChart() {
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const labels = snapshots.map((s) => formatTime(s.time));
    const data = snapshots.map((s) => formatBytes(s.mem_heap_b));

    chart = new Chart(ctx, {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "Heap Memory (MB)",
            data,
            borderColor: "#1976d2",
            backgroundColor: "rgba(25, 118, 210, 0.1)",
            fill: true,
            tension: 0.1,
            pointRadius: 3,
            pointHoverRadius: 6,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            display: true,
            position: "top",
          },
          tooltip: {
            callbacks: {
              title: (items) => {
                const idx = items[0]?.dataIndex;
                if (idx !== undefined) {
                  const time = formatTime(snapshots[idx].time);
                  return `Time: ${time.toFixed(2)}s`;
                }
                return "";
              },
              label: (item) => {
                const idx = item.dataIndex;
                const snapshot = snapshots[idx];
                const heapMB = formatBytes(snapshot.mem_heap_b);
                const extraMB = formatBytes(snapshot.mem_heap_extra_b);
                return [
                  `Heap: ${heapMB.toFixed(2)} MB`,
                  `Extra: ${extraMB.toFixed(2)} MB`,
                  `Snapshot: #${snapshot.snapshot_num}`,
                ];
              },
            },
          },
        },
        scales: {
          x: {
            type: "linear",
            title: {
              display: true,
              text: "Time (seconds)",
            },
          },
          y: {
            type: "linear",
            title: {
              display: true,
              text: "Memory (MB)",
            },
            beginAtZero: true,
          },
        },
      },
    });
  }

  function destroyChart() {
    if (chart) {
      chart.destroy();
      chart = null;
    }
  }

  onMount(() => {
    createChart();
  });

  onDestroy(() => {
    destroyChart();
  });

  $effect(() => {
    // Re-create chart when snapshots change
    if (snapshots && canvas) {
      destroyChart();
      createChart();
    }
  });
</script>

<div class="chart-container">
  <canvas bind:this={canvas}></canvas>
</div>

<style>
  .chart-container {
    width: 100%;
    height: 400px;
    padding: 16px;
  }
</style>
