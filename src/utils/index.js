import { Command } from '@tauri-apps/api/shell'
import { register } from '@tauri-apps/api/globalShortcut'
import { invoke } from '@tauri-apps/api/tauri'

export function registerGloabalShortcutForSpecificApp ({ shortcut, id }) {
    return register(shortcut, () => {
      // showOrHideApp(appName).then(() => {
      //   console.log('Shortcut triggered')
      // })

      invoke('get_focused_app_bundle_identifier').then(focusedAppId => {
        if (focusedAppId === id) {
          invoke('hide_frontmost_app')
        } else {
          invoke('open_app', { bundleId: id })
        }
      })
    })
}

export function showOrHideApp (appName) {
  return showOrHideAppUseOsascript(appName)
}

export function showOrHideAppUseOsascript (appName) {
  // return runOsascript(['-e', 'tell application "Finder"', '-e', `set visible of process "${appName}" to false`, '-e', 'end tell'])
  return runOsascript(['-e', `application "${appName}" is running`]).then(({ code, stdout }) => {
    console.log(code, stdout)
    if (code === 0 && stdout === 'false') {
      return runOsascript(['-e', `tell application "${appName}" to activate`])
    } else if (code === 0 && stdout !== 'false') {
      return runOsascript(['-e', 'application (path to frontmost application as text)']).then(res => {
        console.log(res.stdout, appName)
        if (res.stdout === appName) {
          return runOsascript(['-e', 'tell application "Finder"', '-e', `set visible of process "${appName}" to false`, '-e', 'end tell'])
        } else {
          console.log('555')
          return runOsascript(['-e', `tell application "${appName}" to activate`])
        }
      })
    }

    return new Error('出错')
  })
}

export function runOsascript (args) {
  return new Command('run-osascript', args).execute()
}

// export function runOsascriptsingle (args) {
//   return new Command('run-osascript-single', args).execute()
// }

export function extractAppName(filePath) {
  var regex = /\/Applications\/(.+?)\.app/
  var matches = regex.exec(filePath)
  
  if (matches && matches.length > 1) {
    return matches[1]
  } else {
    return null // 如果无法提取应用程序名称，则返回null或其他适当的值
  }
}
