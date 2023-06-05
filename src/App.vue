<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { extractAppName, registerGloabalShortcutForSpecificApp } from './utils'
import { open } from '@tauri-apps/api/dialog'
// import { appWindow } from '@tauri-apps/api/window'
import { Command, open as shellOpen } from '@tauri-apps/api/shell'
import { register, unregisterAll } from '@tauri-apps/api/globalShortcut'
// import { appDataDir } from '@tauri-apps/api/path'
// Open a selection dialog for directories
// appWindow.setTitle('hello').then((res) => {
//   console.log(res)
// })

// function registerGloabalShortcut () {
//   registerGloabalShortcutForSpecificApp({
//     shortcut: 'Option+a'
//   })
// }

// function cancelShortcut () {
//   unregisterAll()
// }

// function addStr () {
//   invoke('storage_insert', { key: '1', value: 'xxx' })
//   invoke('storage_insert', { key: '2', value: 'fff' })
//   invoke('storage_insert', { key: '3', value: 'xdddxx' })
//   invoke('storage_insert', { key: '4', value: 'xxgggx' })
// }

// function deleteStr () {
//   invoke('storage_delete', { key: '1' })
// }
// function modifyStr () {
//   invoke('storage_insert', { key: '2', value: 'yyy' })
// }

// function getAPP () {
//   invoke('get_frontmost_app_path').then(res => {
//     console.log(res)
//   })
// }

// // function activeAPP () {
// //   invoke('open_app', { bundleId: 'com.apple.Safari' }).then(res => {
// //     console.log(res)
// //   })
// // }
// function activeAPP () {
//   invoke('open_app', { bundleId: 'com.netease.163music' }).then(res => {
//     console.log(res)
//   })
// }
// function hideAPP () {
//   invoke('hide_frontmost_app').then(res => {
//     console.log(res)
//   })
// }
// function getfrontmostApp () {
//   invoke('get_focused_app_bundle_identifier').then(res => {
//     console.log(res)
//   }).catch(err => {
//     console.log(err)
//   })
// }

// function getAppId () {
//   invoke('get_bundle_identifier', { appPath: '/Applications/NeteaseMusic.app/' }).then(res => {
//     console.log(res)
//   })
// }

// 数据处理
const dataArr = ref([])
function loadDataArr () {
  invoke('load_storage').then(res => {
    const response = JSON.parse(res)
    if (response.length) dataArr.value = res
  })
}
loadDataArr()

// 按钮
function openApplicationsDir () {
  open({
    directory: false,
    multiple: false,
    defaultPath: '/Applications',
  }).then((res) => {
    console.log(res)
    const appname = extractAppName(res)
    invoke('get_bundle_identifier', { appPath: res }).then(id => {
      dataArr.value.push({
        key: id,
        appname,
        hotkey: ''
      })
    })
  })
}

// 聚焦
const focusedKey = ref('')
function setFocusedKey (key) {
  focusedKey.value = key

  window.addEventListener('keydown', keydownHandler.bind(null, key))
}


// 注册
let pressedKeys = new Set();
function clearPressedKeys () {
  pressedKeys.clear()
}

const mods = new Map([
  ['altKey', 'Alt'],
  ['ctrlKey', 'Control'],
  ['metaKey', 'Meta'],
  ['shiftKey', 'Shift']
])

function keydownHandler (id, event)  {
  console.log('============== keydownHandler ==================')
  if (!Array.from(mods.values()).includes(event.key)) {
    Array.from(mods.keys()).forEach(item => {
      if (event[item]) pressedKeys.add(mods.get(item))
    })
    pressedKeys.add(event.code.slice(3))

    const shortcut = Array.from(pressedKeys).join('+')
    dataArr.value.find(item => item.key === id).hotkey = shortcut

    registerGloabalShortcutForSpecificApp({ shortcut, id }).then(res => {
      clearPressedKeys()
      window.removeEventListener('keydown', keydownHandler)
    })
  }
}
</script>

<template>
  <div class="container">
    <div class="register-item" v-for="(item) in dataArr" :key="item.key">
      <div class="appname">{{ item.appname }}</div>
      <div
        class="hot-key-wrapper"
        :class="focusedKey === item.key ? 'selected' : ''"
        @click="setFocusedKey(item.key)"
      >{{ item.hotkey }}</div>
    </div>
    <button @click="openApplicationsDir">+</button>
    <button>-</button>
  </div>
</template>

<style scoped lang="scss">
body {
  margin: 0 !important;
}
.container {
  box-sizing: border-box;
  width: calc(100vw - 16px); height: calc(100vh - 16px);
  padding: 10px;
  overflow: hidden;
  background-color: #e9e9e9;
}
.register-item {
  display: flex; align-items: center; justify-content: space-between;
  font-size: 15px;
}
.appname {}
.hot-key-wrapper {
  min-width: 100px;
  height: 28px; line-height: 28px;
  border: 1px solid #fff;
  border-radius: 30px;
  text-align: center;

  &.selected {
    border-color: #ff5a05;
  }
}
</style>
