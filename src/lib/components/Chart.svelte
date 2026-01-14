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
  import type { Snapshot, HeapNode } from "$lib/types";

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
  let showStacked = $state(true);

  // Colors for the stacked areas
  const COLORS = [
    { border: "#e57373", bg: "rgba(229, 115, 115, 0.7)" }, // red
    { border: "#64b5f6", bg: "rgba(100, 181, 246, 0.7)" }, // blue
    { border: "#81c784", bg: "rgba(129, 199, 132, 0.7)" }, // green
    { border: "#ffb74d", bg: "rgba(255, 183, 77, 0.7)" },  // orange
    { border: "#ba68c8", bg: "rgba(186, 104, 200, 0.7)" }, // purple
    { border: "#4dd0e1", bg: "rgba(77, 208, 225, 0.5)" },  // cyan (Other)
  ];

  function formatTime(time: number): number {
    if (timeUnit === "i") {
      return time / 1_000_000_000;
    }
    if (timeUnit === "ms") {
      return time / 1000;
    }
    return time;
  }

  function formatBytes(bytes: number): number {
    return bytes / (1024 * 1024);
  }

  // Get a short label for a heap node
  function getNodeLabel(node: HeapNode): string {
    if (node.function.includes("in ") && node.function.includes("places")) {
      return "Other allocations";
    }
    const func = node.function;
    if (func.length > 30) {
      return func.substring(0, 27) + "...";
    }
    return func || "Unknown";
  }

  // Check if we have detailed heap data
  function hasDetailedData(): boolean {
    return snapshots.some(s => s.heap_tree && s.heap_tree.children.length > 0);
  }

  // Extract top contributors from heap trees
  function extractContributors(): {
    labels: string[];
    datasets: { label: string; data: number[]; borderColor: string; backgroundColor: string; fill: boolean; tension: number; pointRadius: number; pointHoverRadius: number }[];
  } {
    const timeLabels = snapshots.map((s) => formatTime(s.time));

    const contributorBytes: Map<string, number[]> = new Map();
    const contributorMaxBytes: Map<string, number> = new Map();

    snapshots.forEach((snapshot, idx) => {
      if (snapshot.heap_tree && snapshot.heap_tree.children.length > 0) {
        snapshot.heap_tree.children.forEach((child) => {
          const label = getNodeLabel(child);
          if (!contributorBytes.has(label)) {
            contributorBytes.set(label, new Array(snapshots.length).fill(0));
            contributorMaxBytes.set(label, 0);
          }
          contributorBytes.get(label)![idx] = formatBytes(child.bytes);
          const currentMax = contributorMaxBytes.get(label) || 0;
          if (child.bytes > currentMax) {
            contributorMaxBytes.set(label, child.bytes);
          }
        });
      }
    });

    const sortedContributors = [...contributorMaxBytes.entries()]
      .sort((a, b) => b[1] - a[1])
      .slice(0, 5)
      .map(([label]) => label);

    const otherData = snapshots.map((snapshot, idx) => {
      const total = formatBytes(snapshot.mem_heap_b);
      const top5Sum = sortedContributors.reduce((sum, label) => {
        const data = contributorBytes.get(label);
        return sum + (data ? data[idx] : 0);
      }, 0);
      return Math.max(0, total - top5Sum);
    });

    const datasets = sortedContributors.map((label, i) => ({
      label,
      data: contributorBytes.get(label) || [],
      borderColor: COLORS[i].border,
      backgroundColor: COLORS[i].bg,
      fill: true,
      tension: 0.1,
      pointRadius: 2,
      pointHoverRadius: 4,
    }));

    datasets.push({
      label: "Other",
      data: otherData,
      borderColor: COLORS[5].border,
      backgroundColor: COLORS[5].bg,
      fill: true,
      tension: 0.1,
      pointRadius: 2,
      pointHoverRadius: 4,
    });

    return { labels: timeLabels.map(String), datasets };
  }

  function createSimpleChart() {
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const labels = snapshots.map((s) => String(formatTime(s.time)));
    const data = snapshots.map((s) => formatBytes(s.mem_heap_b));

    chart = new Chart(ctx, {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "Heap Memory (MB)",
            data,
            borderColor: "#4fc3f7",
            backgroundColor: "rgba(79, 195, 247, 0.3)",
            fill: true,
            tension: 0.1,
            pointRadius: 3,
            pointHoverRadius: 6,
          },
        ],
      },
      options: getChartOptions(false),
    });
  }

  function createStackedChart() {
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const { labels, datasets } = extractContributors();

    chart = new Chart(ctx, {
      type: "line",
      data: { labels, datasets },
      options: {
        ...getChartOptions(true),
        plugins: {
          ...getChartOptions(true).plugins,
          legend: {
            display: true,
            position: "top",
            labels: {
              color: "#d4d4d4",
              boxWidth: 12,
              padding: 8,
              font: { size: 11 },
            },
          },
        },
      },
    });
  }

  function createChart() {
    if (showStacked && hasDetailedData()) {
      createStackedChart();
    } else {
      createSimpleChart();
    }
  }

  function getChartOptions(stacked: boolean) {
    return {
      responsive: true,
      maintainAspectRatio: false,
      interaction: {
        mode: "index" as const,
        intersect: false,
      },
      plugins: {
        legend: {
          display: true,
          position: "top" as const,
          labels: {
            color: "#d4d4d4",
          },
        },
        tooltip: {
          backgroundColor: "#252526",
          titleColor: "#d4d4d4",
          bodyColor: "#d4d4d4",
          borderColor: "#3c3c3c",
          borderWidth: 1,
          callbacks: {
            title: (items: any[]) => {
              const idx = items[0]?.dataIndex;
              if (idx !== undefined) {
                const time = formatTime(snapshots[idx].time);
                return `Time: ${time.toFixed(2)}s (Snapshot #${snapshots[idx].snapshot_num})`;
              }
              return "";
            },
            label: (item: any) => {
              const value = item.raw as number;
              return `${item.dataset.label}: ${value.toFixed(2)} MB`;
            },
            footer: (items: any[]) => {
              const idx = items[0]?.dataIndex;
              if (idx !== undefined && stacked) {
                const total = formatBytes(snapshots[idx].mem_heap_b);
                return `Total: ${total.toFixed(2)} MB`;
              }
              return "";
            },
          },
        },
      },
      scales: {
        x: {
          type: "linear" as const,
          title: {
            display: true,
            text: "Time (seconds)",
            color: "#808080",
          },
          ticks: {
            color: "#808080",
          },
          grid: {
            color: "#2d2d2d",
          },
        },
        y: {
          type: "linear" as const,
          title: {
            display: true,
            text: "Memory (MB)",
            color: "#808080",
          },
          ticks: {
            color: "#808080",
          },
          grid: {
            color: "#2d2d2d",
          },
          beginAtZero: true,
          stacked: stacked,
        },
      },
    };
  }

  function destroyChart() {
    if (chart) {
      chart.destroy();
      chart = null;
    }
  }

  // Export chart as SVG
  export function exportSvg(): string | null {
    if (!chart || !canvas) return null;

    const width = canvas.width;
    const height = canvas.height;
    const ctx = chart.ctx;

    // Get chart area and scales
    const chartArea = chart.chartArea;
    const xScale = chart.scales.x;
    const yScale = chart.scales.y;

    // Start building SVG
    let svg = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}">
  <rect width="100%" height="100%" fill="#1e1e1e"/>
  <style>
    .axis-label { font-family: -apple-system, BlinkMacSystemFont, sans-serif; font-size: 12px; fill: #808080; }
    .title { font-family: -apple-system, BlinkMacSystemFont, sans-serif; font-size: 14px; fill: #d4d4d4; }
    .legend { font-family: -apple-system, BlinkMacSystemFont, sans-serif; font-size: 11px; fill: #d4d4d4; }
  </style>
`;

    // Draw grid lines
    svg += `  <g class="grid">\n`;
    const gridColor = "#2d2d2d";

    // Y grid lines
    yScale.ticks.forEach((tick: any, i: number) => {
      const y = yScale.getPixelForValue(tick.value);
      svg += `    <line x1="${chartArea.left}" y1="${y}" x2="${chartArea.right}" y2="${y}" stroke="${gridColor}" stroke-width="1"/>\n`;
    });

    // X grid lines
    xScale.ticks.forEach((tick: any, i: number) => {
      const x = xScale.getPixelForValue(tick.value);
      svg += `    <line x1="${x}" y1="${chartArea.top}" x2="${x}" y2="${chartArea.bottom}" stroke="${gridColor}" stroke-width="1"/>\n`;
    });
    svg += `  </g>\n`;

    // Draw datasets
    svg += `  <g class="datasets">\n`;

    chart.data.datasets.forEach((dataset: any, datasetIndex: number) => {
      const meta = chart.getDatasetMeta(datasetIndex);
      const points: { x: number; y: number }[] = [];

      meta.data.forEach((point: any, i: number) => {
        points.push({ x: point.x, y: point.y });
      });

      if (points.length > 0) {
        // Draw fill area if fill is enabled
        if (dataset.fill) {
          let areaPath = `M ${points[0].x} ${chartArea.bottom} `;
          points.forEach((p) => {
            areaPath += `L ${p.x} ${p.y} `;
          });
          areaPath += `L ${points[points.length - 1].x} ${chartArea.bottom} Z`;
          svg += `    <path d="${areaPath}" fill="${dataset.backgroundColor}" opacity="0.7"/>\n`;
        }

        // Draw line
        let linePath = `M ${points[0].x} ${points[0].y}`;
        for (let i = 1; i < points.length; i++) {
          linePath += ` L ${points[i].x} ${points[i].y}`;
        }
        svg += `    <path d="${linePath}" fill="none" stroke="${dataset.borderColor}" stroke-width="2"/>\n`;

        // Draw points
        points.forEach((p) => {
          svg += `    <circle cx="${p.x}" cy="${p.y}" r="${dataset.pointRadius || 3}" fill="${dataset.borderColor}"/>\n`;
        });
      }
    });
    svg += `  </g>\n`;

    // Draw axis labels
    svg += `  <g class="axis-labels">\n`;

    // X axis label
    svg += `    <text class="axis-label" x="${(chartArea.left + chartArea.right) / 2}" y="${height - 10}" text-anchor="middle">Time (seconds)</text>\n`;

    // Y axis label
    svg += `    <text class="axis-label" x="15" y="${(chartArea.top + chartArea.bottom) / 2}" text-anchor="middle" transform="rotate(-90, 15, ${(chartArea.top + chartArea.bottom) / 2})">Memory (MB)</text>\n`;

    // X axis tick labels
    xScale.ticks.forEach((tick: any) => {
      const x = xScale.getPixelForValue(tick.value);
      svg += `    <text class="axis-label" x="${x}" y="${chartArea.bottom + 20}" text-anchor="middle">${tick.value.toFixed(1)}</text>\n`;
    });

    // Y axis tick labels
    yScale.ticks.forEach((tick: any) => {
      const y = yScale.getPixelForValue(tick.value);
      svg += `    <text class="axis-label" x="${chartArea.left - 10}" y="${y + 4}" text-anchor="end">${tick.value.toFixed(1)}</text>\n`;
    });

    svg += `  </g>\n`;

    // Draw legend
    svg += `  <g class="legend">\n`;
    let legendX = chartArea.left;
    const legendY = 20;

    chart.data.datasets.forEach((dataset: any, i: number) => {
      svg += `    <rect x="${legendX}" y="${legendY - 8}" width="12" height="12" fill="${dataset.backgroundColor || dataset.borderColor}"/>\n`;
      svg += `    <text class="legend" x="${legendX + 16}" y="${legendY}">${dataset.label}</text>\n`;
      legendX += 16 + dataset.label.length * 7 + 20;
    });

    svg += `  </g>\n`;
    svg += `</svg>`;

    return svg;
  }

  function toggleMode() {
    showStacked = !showStacked;
    destroyChart();
    createChart();
  }

  onMount(() => {
    createChart();
  });

  onDestroy(() => {
    destroyChart();
  });

  $effect(() => {
    if (snapshots && canvas) {
      destroyChart();
      createChart();
    }
  });
</script>

<div class="chart-wrapper">
  <div class="chart-toolbar">
    <div class="toggle-group">
      <button
        class="toggle-btn"
        class:active={!showStacked}
        onclick={toggleMode}
        disabled={!hasDetailedData()}
      >
        Total
      </button>
      <button
        class="toggle-btn"
        class:active={showStacked}
        onclick={toggleMode}
        disabled={!hasDetailedData()}
      >
        Stacked
      </button>
    </div>
  </div>
  <div class="chart-container">
    <canvas bind:this={canvas}></canvas>
  </div>
</div>

<style>
  .chart-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #1e1e1e;
  }

  .chart-toolbar {
    display: flex;
    justify-content: flex-end;
    padding: 12px 16px;
    border-bottom: 1px solid #3c3c3c;
    background: #252526;
  }

  .toggle-group {
    display: flex;
    border-radius: 4px;
    overflow: hidden;
    border: 1px solid #3c3c3c;
  }

  .toggle-btn {
    padding: 6px 16px;
    border: none;
    background: #2d2d2d;
    color: #808080;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .toggle-btn:first-child {
    border-right: 1px solid #3c3c3c;
  }

  .toggle-btn:hover:not(:disabled) {
    background: #3c3c3c;
    color: #d4d4d4;
  }

  .toggle-btn.active {
    background: #4fc3f7;
    color: #1e1e1e;
    font-weight: 500;
  }

  .toggle-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .chart-container {
    flex: 1;
    padding: 16px;
    box-sizing: border-box;
    min-height: 0;
  }
</style>
