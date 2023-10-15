import { createContext, useContext } from "react";

export type Image = {
    subreddit: string;
    title: string;
    post_url: string;
    image_url: string;
}

type AppContextType = {
    subreddits: string[];
    setSubreddits: (subreddits: string[]) => void;
    images: Image[];
    setImages: (images: Image[]) => void;
    refreshSubreddits: () => void;
    getNewImages: () => void;
}

const AppContext = createContext<AppContextType>({
    subreddits: [],
    setSubreddits: () => {},
    images: [],
    setImages: () => {},
    refreshSubreddits: () => {},
    getNewImages: () => {},
});

export const useAppContext = () => useContext(AppContext);

export default AppContext;