<script lang="ts">
import { isTauri } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';
import { onMount } from 'svelte';

/** 画布尺寸 */
const CANVAS_SIZE = 512;
/** 导出文件名 */
const EXPORT_FILENAME = 'life-trajectory-icon.png';

type IconShape = {
  color: string;
  size: number;
  centerX: number;
  centerY: number;
  rotation: number;
};

/**
 * 图标形状配置
 * color: 颜色
 * size: 大小
 * centerX: 中心X坐标
 * centerY: 中心Y坐标
 * rotation: 旋转角度
 */
const ICON_SHAPES: IconShape[] = [
  { color: '#FFE270', size: 138, centerX: 256, centerY: 158.42, rotation: 45 },
  { color: '#89A9C2', size: 138, centerX: 256, centerY: 353.58, rotation: 45 },
  { color: '#9CC8B5', size: 138, centerX: 158.42, centerY: 256, rotation: 45 },
  { color: '#D8CDE3', size: 138, centerX: 353.58, centerY: 256, rotation: 45 },
  { color: '#F8F9FA', size: 56, centerX: 256, centerY: 256, rotation: 0 }
];

let canvas: HTMLCanvasElement;

/**
 * 绘制图标
 * @param ctx CanvasRenderingContext2D
 */
function drawIcon(ctx: CanvasRenderingContext2D) {
  ctx.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);

  for (const shape of ICON_SHAPES) {
    const half = shape.size / 2;

    ctx.save();
    ctx.translate(shape.centerX, shape.centerY);
    ctx.rotate((shape.rotation * Math.PI) / 180);
    ctx.fillStyle = shape.color;
    ctx.fillRect(-half, -half, shape.size, shape.size);
    ctx.restore();
  }
}

/**
 * 将画布转换为 Blob
 * @returns Blob | null
 */
function canvasToBlob(): Promise<Blob | null> {
  return new Promise((resolve) => {
    canvas.toBlob(resolve, 'image/png', 1);
  });
}

/**
 * 在浏览器中导出 PNG
 * @param blob Blob
 */
async function exportPngInBrowser(blob: Blob) {
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = EXPORT_FILENAME;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}

async function exportPngInTauri(blob: Blob) {
  const path = await save({
    defaultPath: EXPORT_FILENAME,
    filters: [{ name: 'PNG', extensions: ['png'] }]
  });

  if (!path) {
    return;
  }

  const data = new Uint8Array(await blob.arrayBuffer());
  await writeFile(path, data);
}

async function exportPng() {
  const blob = await canvasToBlob();
  if (!blob) {
    return;
  }

  if (isTauri()) {
    await exportPngInTauri(blob);
    return;
  }

  await exportPngInBrowser(blob);
}

onMount(() => {
  const ctx = canvas.getContext('2d');
  if (!ctx) {
    return;
  }

  drawIcon(ctx);
});
</script>

<main class="container">
  <h1>Life Trajectory</h1>
  <canvas bind:this={canvas} width={CANVAS_SIZE} height={CANVAS_SIZE} aria-label="Life Trajectory icon"></canvas>
  <button type="button" onclick={exportPng}>导出 PNG</button>
</main>

<style>
.container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  align-items: center;
  justify-content: center;
  padding-top: 10vh;
  margin: 0;
  text-align: center;
}

h1 {
  text-align: center;
}

canvas {
  width: 512px;
  height: 512px;
}

button {
  padding: 0.5rem 1rem;
  font: inherit;
  cursor: pointer;
  background: #fff;
  border: 1px solid #ccc;
  border-radius: 6px;
}

button:hover {
  background: #f5f5f5;
}
</style>
