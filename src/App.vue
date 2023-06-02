<script setup>
import { invoke } from '@tauri-apps/api/tauri'
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

function addStr () {
  invoke('storage_insert', { key: '1', value: 'xxx' })
  invoke('storage_insert', { key: '2', value: 'fff' })
  invoke('storage_insert', { key: '3', value: 'xdddxx' })
  invoke('storage_insert', { key: '4', value: 'xxgggx' })
}
function getStr () {
  invoke('load_storage').then(res => {
    console.log(res)
  })
}
function deleteStr () {
  invoke('storage_delete', { key: '1' })
}
function modifyStr () {
  invoke('storage_insert', { key: '2', value: 'yyy' })
}

function getAPP () {
  invoke('get_frontmost_app_path').then(res => {
    console.log(res)
  })
}

function activeAPP () {
  invoke('open_app', { bundleId: 'com.apple.Safari' }).then(res => {
    console.log(res)
  })
}

// 在前端代码中
let pressedKeys = new Set();

window.addEventListener('keydown', (event) => {
  pressedKeys.add(event.key);
  console.log(event)
})
window.addEventListener('keyup', (event) => {
  console.log(pressedKeys)
});
</script>

<template>
  <div class="container">
    <button @click="openApplicationsDir">注册快捷键</button>
    <button @click="cancelShortcut">取消快捷键</button>
    <button @click="addStr">添加数据</button>
    <button @click="deleteStr">删除数据</button>
    <button @click="modifyStr">修改数据</button>
    <button @click="getStr">获取已经添加过的数据</button>
    <button @click="getAPP">获取当前的APP</button>
    <button @click="activeAPP">打开APP</button>
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
