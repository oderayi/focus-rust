import { process } from '@tauri-apps/api'
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from '@tauri-apps/api/window'
import React, { useState } from 'react'
import './App.css'

function App() {
  const [content, setContent] = useState('')

  const handleKeyDown = async (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      if (content.length) {
        await invoke('add_task', { content });
        setContent("");
      }
      await invoke('hide_window');
    }
  }

  return <input
    type="text"
    value={content}
    onChange={e => setContent(e.target.value)}
    onKeyDown={handleKeyDown}
    className='w-[800px] h-[80px] bg-[#222] text-2xl text-white px-6'
  />
}

export default App
