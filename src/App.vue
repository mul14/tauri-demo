<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { dialog, fs, path, notification } from '@tauri-apps/api'
import { ref, onMounted } from 'vue'
import { BroadcastChannel } from 'broadcast-channel';
import TButton from './components/TButton.vue';

const logs = ref<string[]>([])
const channel = new BroadcastChannel('work');

const helloRust = async () => {
  console.log(await invoke('hello_rust'))
}

const openWindow = async () => {
  await invoke('open_window', {
    label: 'broadcast'
  })
}

const nofity = async () => {
  const result = await invoke('notify', {
    title: "Hello from Tauri",
    body: "You have a message",
  })
}

const httpPost = async () => {
  const result = await invoke('http_post', {
    url: "https://httpbin.org/post"
  })

  console.log(result)
}

const httpGet = async () => {
  const result = await invoke('http_get', {
    url: "https://httpbin.org/get"
  })

  console.log(result)
}

const writeFile = async () => {
  const homeDir = await path.homeDir()

  const result = await invoke('write_file', {
    path: `${homeDir}/sample.txt`
  })

  console.log(result)
}

const readFile = async () => {
  const homeDir = await path.homeDir()

  const result = await invoke('read_file', {
    path: `${homeDir}/.bashrc`
  })

  console.log(result)
}

const openDialog = async () => {
  const downloadDir = await path.downloadDir()

  const result = await dialog.save({
    defaultPath: `${downloadDir}/sample.txt`
  })

  console.log(result)
}
const readDir = async () => {
  const home = await path.homeDir()
  const dir = await path.appDir()
  const current = await path.currentDir()
  const config = await path.configDir()
  const download = await path.downloadDir()
  const sep = path.sep
  console.log(home, dir, current, config, download, sep)
  const result = await fs.readDir(home, {

  })
  console.log(result)
}

onMounted(() => {
  channel.onmessage = (message: string) => {
    logs.value.unshift(message)
    console.log('Received', message)
  }

  channel.postMessage('Broadcast message from main-window');
})

</script>

<template>
  <div class="max-w-xl mx-auto">
    <div class="grid grid-cols-3 gap-4">
      <TButton @click="helloRust">Send to Rust</TButton>
      <TButton @click="openDialog">Open Dialog</TButton>
      <TButton @click="readDir">Read Directory</TButton>
      <TButton @click="writeFile">Append date to ~/sample.txt</TButton>
      <TButton @click="readFile">Read ~/.bashrc</TButton>
      <TButton @click="nofity">Notification</TButton>
      <TButton @click="httpGet">HTTP GET</TButton>
      <TButton @click="httpPost">HTTP POST</TButton>
      <TButton @click="openWindow">Open Window</TButton>
    </div>
    <div class="p-4 border m-4">
      <div v-for="(log, index) in logs" :key="index" class="text-left">
        {{ log }}
      </div>
    </div>
  </div>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
