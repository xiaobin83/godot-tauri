<script lang="ts">
export default {
  data() {
    return {
      subPagePath: '/public/godot/PlayGround.html'
    };
  }
};
</script>

<template>
  <main class="container">
	<iframe :src="subPagePath" width="100%" height="100%" frameborder="0"></iframe>
  </main>
</template>

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
