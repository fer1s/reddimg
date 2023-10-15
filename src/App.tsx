import { useState, useEffect, useRef } from 'react'
import { Routes, Route } from 'react-router-dom'
import AppContext, { Image } from './AppContext'
import { getSubreddits, getImages } from './api'

import AppBar from './components/AppBar'

import Feed from './pages/Feed'
import Settings from './pages/Settings'

function App() {
   const [ready, setReady] = useState(false)
   const [subreddits, setSubreddits] = useState<string[]>([])
   const [images, setImages] = useState<Image[]>([])

   const scrollRef = useRef<HTMLDivElement>(null)

   const refreshSubreddits = async () => {
      const subreddits = await getSubreddits()
      setSubreddits(subreddits)

      fetchImages()
   }

   const fetchImages = async () => {
      const images = await getImages(3)
      setImages(images)
   }

   const getNewImages = async () => {
      const newImgs = await getImages(3)
      setImages((prev) => [...prev, ...newImgs])
   }

   const handleScroll = (e: any) => {
      // Get actual route
      const route = window.location.pathname
      const { offsetHeight, scrollTop, scrollHeight } = e.target

      // If not on feed page, don't do anything
      if (route !== '/') return

      if (offsetHeight + scrollTop >= scrollHeight) {
         console.log('Reached bottom of feed')
         getNewImages()

         // Remove event listener
         e.target.removeEventListener('scroll', handleScroll)
      }
   }

   useEffect(() => {
      fetchImages()
      refreshSubreddits().then(() => setReady(true))
   }, [])

   useEffect(() => {
      if (scrollRef.current) {
         scrollRef.current.addEventListener('scroll', handleScroll)
      }
   }, [images])

   return (
      <AppContext.Provider
         value={{
            subreddits,
            setSubreddits,
            images,
            setImages,
            refreshSubreddits,
            getNewImages,
         }}
      >
         <div className="App">
            <AppBar />
            {ready ? (
               <div className="page" onScroll={handleScroll} ref={scrollRef}>
                  <Routes>
                     <Route path="/">
                        <Route index element={<Feed />} />
                        <Route path="settings" element={<Settings />} />
                     </Route>
                  </Routes>
               </div>
            ) : (
               <div className="loading">
                  <h1>Gathering your subreddits...</h1>
                  <span className="loader"></span>
               </div>
            )}
         </div>
      </AppContext.Provider>
   )
}

export default App
