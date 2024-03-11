<template>
  <div
    class="flex h-dvh w-screen flex-wrap place-content-center gap-4 bg-neutral-200"
  >
    <BuzzerButton
      @start-buzzing="startBuzzing()"
      @stop-buzzing="stopBuzzing()"
    ></BuzzerButton>
  </div>
</template>

<script setup lang="ts">
import BuzzerButton from "./components/BuzzerButton.vue";

const websocket = new WebSocket("ws://" + location.hostname + "/ws");
websocket.onopen = function (event) {
  console.log("Successfully connected to websocket.", event);
};

function startBuzzing(){
  if(websocket.readyState === WebSocket.OPEN){
    websocket.send("start");
  }
}

function stopBuzzing(){
  if(websocket.readyState === WebSocket.OPEN){
    websocket.send("stop");
  }
}
</script>
