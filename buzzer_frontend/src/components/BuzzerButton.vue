<template>
  <button
    class="size-32 rounded-xl border-2 border-black bg-emerald-700 p-2 lg:size-40"
    @touchstart.prevent="$emit('startBuzzing')"
    @touchend.prevent="$emit('stopBuzzing')"
    @mousedown.prevent="$emit('startBuzzing')"
    @mouseup.prevent="$emit('stopBuzzing')"
  ></button>
</template>

<script setup lang="ts">
defineEmits(["startBuzzing", "stopBuzzing"]);
</script>
