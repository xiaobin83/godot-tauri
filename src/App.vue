<template>
  <main class="container">
	<iframe id="godot_frame" src="/godot/PlayGround.html" width="100%" height="100%" frameborder="0"></iframe>
  </main>
</template>

<script lang="ts" setup>
import { onMounted, onBeforeUnmount} from 'vue';
import { invoke } from '@tauri-apps/api/core';

const handleMessage = (event: MessageEvent) => {
  console.log(event.data)
  if (event.data == 'toggleConsole') {
    invoke("toggle_console");
  }
}

onMounted(() => {
  const iframe = document.getElementById('godot_frame') as HTMLIFrameElement | null;
  if (iframe) {
    iframe.addEventListener('load', () => {
      try {
        const doc = iframe.contentDocument || iframe.contentWindow?.document;
        if (doc) {
          doc.body.style.background = 'none';
          doc.body.style.backgroundColor = 'transparent';
        }
      } catch (e) {
      }
    });
  }

  window.addEventListener('message', handleMessage);

});

onBeforeUnmount(() => {
  window.removeEventListener('message', handleMessage)
})


</script>

<style>
.container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: transparent;
}

iframe {
  width: 100%;
  height: 100%;
  border: none;
  display: block;
}
</style>
