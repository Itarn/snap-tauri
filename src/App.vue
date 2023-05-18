<script setup>
import { registerGloabalShortcutForSpecificApp } from './utils'
import { open } from '@tauri-apps/api/dialog'
// import { appWindow } from '@tauri-apps/api/window'
import { Command, open as shellOpen } from '@tauri-apps/api/shell'
import { register, unregisterAll } from '@tauri-apps/api/globalShortcut'
// import { appDataDir } from '@tauri-apps/api/path'
// Open a selection dialog for directories
// appWindow.setTitle('hello').then((res) => {
//   console.log(res)
// })

function openApplicationsDir () {
  // open({
  //   directory: false,
  //   multiple: false,
  //   defaultPath: '/Applications',
  // }).then((res) => {
  //   register('Option+a', () => {
  //     // shellOpen(res)
  //     // console.log('Shortcut triggered')
  //   })
  //   // console.log(res)
  // })

  registerGloabalShortcutForSpecificApp({
    shortcut: 'Option+a',
    appName: 'Twitter'
  })
}

function cancelShortcut () {
  unregisterAll()
}
</script>

<template>
  <div class="container">
    <button @click="openApplicationsDir">注册快捷键</button>
    <button @click="cancelShortcut">取消快捷键</button>
  </div>
</template>

<style scoped>
body {
  margin: 0 !important;
}
.container {
  width: calc(100vw - 16px); height: calc(100vh - 16px);
  overflow: hidden;
  background-color: #e9e9e9;
}
</style>
