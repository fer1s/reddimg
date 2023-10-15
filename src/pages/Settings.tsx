import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAppContext } from '../AppContext'

import { addSubreddit, removeSubreddit } from '../api'

import { FiArrowLeft, FiTrash } from 'react-icons/fi'
import '../styles/PSettings.scss'

const subredditRegex = /^([a-zA-Z0-9_]{3,21})$/

const Settings = () => {
   const navigate = useNavigate()

   const { subreddits, refreshSubreddits } = useAppContext()
   const [newSubreddit, setNewSubreddit] = useState('')

   const goBack = () => {
      navigate('/')
   }

   const handleEnter = (e: React.KeyboardEvent<HTMLInputElement>) => {
      if (e.key === 'Enter') {
         addNewSubreddit()
      }
   }

   const addNewSubreddit = () => {
      if (newSubreddit !== '') {
         if (!subredditRegex.test(newSubreddit)) return
         addSubreddit(newSubreddit).then(() => {
            refreshSubreddits()
            setNewSubreddit('')
         })
      }
   }

   const handleRemoveSubreddit = (subreddit: string) => {
      removeSubreddit(subreddit).then((res) => {
         if (res) refreshSubreddits()
      })
   }

   return (
      <div className="settings">
         <h1>
            <FiArrowLeft onClick={goBack} />
            <span>Settings</span>
         </h1>

         <section>
            <h2>Subreddits</h2>
            <p>Subreddits are the main way to customize your feed. You can choose any subreddit you want.</p>
            <hr />
            <div className="add_subreddit">
               <input type="text" placeholder="New subreddit" value={newSubreddit} onChange={(e) => setNewSubreddit(e.target.value)} onKeyDown={handleEnter} />
               <button onClick={addNewSubreddit}>Add</button>
            </div>
            <div className="subreddits">
               <h3>Your subreddits</h3>
               {subreddits.length > 0 ? (
                  subreddits.map((subreddit) => (
                     <div className="subreddit" key={subreddit}>
                        <span>{subreddit}</span>
                        <button
                           onClick={() => {
                              handleRemoveSubreddit(subreddit)
                           }}
                        >
                           <FiTrash />
                        </button>
                     </div>
                  ))
               ) : (
                  <p>You don't have any subreddits yet. Add one with the input above.</p>
               )}
            </div>
         </section>

         <section>
            <h2>Password protection</h2>
            <p>Protect your feed with a password. This will be asked every time you open the app.</p>
            <hr />
            <h2>TODO</h2>
         </section>
      </div>
   )
}

export default Settings
