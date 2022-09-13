<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { rainbowCursor, emojiCursor } from "cursor-effects";
  type MouseStatus = {
    x: number;
    y: number;
  };
  let prev: MouseStatus = { x: 0, y: 0 };
  // new emojiCursor({ emoji: ["🔥", "🐬", "🦆"] });
  new rainbowCursor({
    length: 10,
    colors: ["#5bcffa", "#f5aab9", "#ffffff", "#f5aab9", "#5bcffa"],
    size: 5,
  });

  onMount(async () => {
    let canvas = document.querySelector("canvas");
    setInterval(async () => {
      let res = (await invoke("get_mouse_position")) as MouseStatus;
      if (res != prev) {
        let event = new MouseEvent("mousemove", {
          view: window,
          bubbles: true,
          cancelable: true,
          clientX: res.x / 1.5,
          clientY: res.y / 1.5,
        });
        canvas.dispatchEvent(event);
        prev = res;
      }
    }, 100);
  });
</script>

<main class="container">
  <h1>Welcome to Cursor Effects Desktop!</h1>
</main>

<style>
  main.container {
    height: 100%;
    width: 100%;
  }
</style>
