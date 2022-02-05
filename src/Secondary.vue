<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from '@tauri-apps/api/window'
import { ref, onMounted } from 'vue'
import { BroadcastChannel } from 'broadcast-channel';

const message = ref('Hello from broadcast window')

const hideMe = async () => await invoke('hide_broadcast_window')

const channel = new BroadcastChannel('work')

onMounted(() => {
  channel.onmessage = () => {
    console.log('Receive message from main window')
  }
})

const sendMessage = () => {
  channel.postMessage(String(Date.now()) + ' ' + message.value)
  message.value = ''
}
</script>

<template>
  <div data-tauri-drag-region class="flex items-center justify-between bg-blue-400 cursor-grab active:cursor-grabbing select-none">
    <div data-tauri-drag-region class="">Title bar (drag me)</div>
    <button @click="hideMe" class="px-4 text-white bg-red-600">&times;</button>
  </div>

  <div class="m-4">
    <form @submit.prevent="sendMessage">
      <input type="text" v-model="message" class="w-full" />
      <button type="submit" class="bg-blue-700 text-white px-4 py-2 rounded">Broadcast</button>
    </form>
  </div>
</template>
