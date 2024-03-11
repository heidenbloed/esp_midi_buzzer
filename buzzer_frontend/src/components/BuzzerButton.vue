<template>
  <button
    class="size-32 rounded-xl border-2 border-black p-2 lg:size-40"
    :class="colorClass"
    @touchstart.prevent="down()"
    @touchend.prevent="up()"
    @mousedown.prevent="down()"
    @mouseup.prevent="up()"
  ></button>
</template>

<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits(["startBuzzing", "stopBuzzing"]);
const colorClass = ref("bg-emerald-700");

function down() {
  colorClass.value = "bg-red-700";
  emit("startBuzzing");
}

function up() {
  colorClass.value = "bg-emerald-700";
  emit("stopBuzzing");
}
</script>
