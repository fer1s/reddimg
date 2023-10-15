import { appWindow } from '@tauri-apps/api/window'
import { useNavigate } from 'react-router-dom'
import '../styles/AppBar.scss'

import { FiX, FiMinimize2, FiSettings } from 'react-icons/fi'

const AppBar = () => {
   const navigate = useNavigate()

   const closeWindow = () => {
      appWindow.close()
   }

   const minimizeWindow = () => {
      appWindow.minimize()
   }

   const openSettings = () => {
      navigate('/settings')
   }

   return (
      <div data-tauri-drag-region className="app_bar">
         <div className="app_bar__title">reddimg</div>
         <div className="app_bar__buttons">
            <button className="app_bar__button" onClick={openSettings}>
                <FiSettings />
            </button>
            <button className="app_bar__button" onClick={minimizeWindow}>
                <FiMinimize2 />
            </button>
            <button className="app_bar__button" onClick={closeWindow}>
                <FiX />
            </button>
         </div>
      </div>
   )
}

export default AppBar
