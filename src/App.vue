<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { dialog, fs, path, notification } from '@tauri-apps/api'
import TButton from './components/TButton.vue';

const helloRust = async () => {
  console.log(await invoke('hello_rust'))
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
  const result = await invoke('write_file', {
    path: "/Users/mul14/sample.txt"
  })

  console.log(result)
}

const readFile = async () => {
  const result = await invoke('read_file', {
    path: "/Users/mul14/.zshrc"
  })

  console.log(result)
}

const openDialog = async () => {
  const result = await dialog.save({
    defaultPath: "/Users/mul14/sample.txt"
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
</script>

<template>
  <div class="max-w-xl mx-auto">
    <div class="grid grid-cols-3 gap-4">
      <TButton @click="helloRust">Send to Rust</TButton>
      <TButton @click="openDialog">Open Dialog</TButton>
      <TButton @click="readDir">Read Directory</TButton>
      <TButton @click="writeFile">Append date to ~/sample.txt</TButton>
      <TButton @click="readFile">Read ~/.zshrc</TButton>
      <TButton @click="nofity">Notification</TButton>
      <TButton @click="httpGet">HTTP GET</TButton>
      <TButton @click="httpPost">HTTP POST</TButton>
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
