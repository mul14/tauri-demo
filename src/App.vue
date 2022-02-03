<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { dialog, fs, path } from '@tauri-apps/api'

const helloRust = async () => {
  console.log(await invoke('hello_rust'))
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
      <button
        class="px-4 py-2 from-gray-900 to-gray-800 bg-gray-700 hover:bg-gray-900 active:outline-dotted active:outline-gray-700 text-white rounded-lg"
        @click="helloRust"
      >Send to Rust</button>
      <button
        class="px-4 py-2 from-gray-900 to-gray-800 bg-gray-700 hover:bg-gray-900 active:outline-dotted active:outline-gray-700 text-white rounded-lg"
        @click="openDialog"
      >Open Dialog</button>
      <button
        class="px-4 py-2 from-gray-900 to-gray-800 bg-gray-700 hover:bg-gray-900 active:outline-dotted active:outline-gray-700 text-white rounded-lg"
        @click="readDir"
      >Read Directory</button>
      <button
        class="px-4 py-2 from-gray-900 to-gray-800 bg-gray-700 hover:bg-gray-900 active:outline-dotted active:outline-gray-700 text-white rounded-lg"
        @click="writeFile"
      >Append date to ~/sample.txt</button>
      <button
        class="px-4 py-2 from-gray-900 to-gray-800 bg-gray-700 hover:bg-gray-900 active:outline-dotted active:outline-gray-700 text-white rounded-lg"
        @click="readFile"
      >Read ~/.zshrc</button>
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
