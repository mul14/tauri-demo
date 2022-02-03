<script setup lang="ts">

// import { Store } from 'tauri-plugin-store-api'


import { invoke } from '@tauri-apps/api/tauri'
import { dialog, fs, path } from '@tauri-apps/api'

// const store = new Store('.settings.dat')

// const setStore = async () => {
//   console.log(await store.set('some-key', { value: 5 }))
// }

// const getStore = async () => {
//   const val = await store.get('some-key')

//   console.log(val, 'wwwww')
// }

const helloTauri = async () => {
  console.log(await invoke('my_custom_command'))
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
  <button @click="helloTauri">Communication with Rust backend</button>
  <button @click="openDialog">Dialog</button>
  <button @click="readDir">Read dir</button>
  <!-- <button @click="setStore">Set store</button>
  <button @click="getStore">Get store</button> -->
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
