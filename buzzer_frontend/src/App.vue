<template>
  <div
    class="flex h-screen w-screen flex-wrap place-content-center gap-4 bg-neutral-200"
  >
    <BuzzerButton
      @start-buzzing="console.log('start')"
      @stop-buzzing="console.log('stop')"
    ></BuzzerButton>
  </div>
</template>

<script setup lang="ts">
import BuzzerButton from "./components/BuzzerButton.vue";

// const websocket = new WebSocket("ws://" + location.hostname + "/ws");
// websocket.onopen = function (event) {
//   console.log("Successfully connected to websocket.", event);
// };
</script>
