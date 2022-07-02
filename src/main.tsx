import { register } from '@tauri-apps/api/globalShortcut'
import { invoke } from '@tauri-apps/api/tauri'
import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

register('Alt+Enter', async () => {
  await invoke('show_window')
})

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
