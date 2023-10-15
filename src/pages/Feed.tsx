import { useAppContext } from "../AppContext"

import '../styles/PFeed.scss'

const Feed = () => {

  const { images } = useAppContext()

  return (
    <>
        <h1>Your Feed</h1>
        <div className="images">
          {images.map((image, index) => (
            <div className="image" key={index}>
              <img src={image.image_url} alt={image.title} />
              <div className="image_footer">
                <span className="image_title">{image.title}</span>
                <span className="image_subreddit">r/{image.subreddit}</span>
              </div>
            </div>
          ))}
        </div>
    </>
  )
}

export default Feed