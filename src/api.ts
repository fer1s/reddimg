import { invoke } from "@tauri-apps/api";

export const getSubreddits = async () => {
    const res: string = await invoke("get_subreddits");
    return JSON.parse(res);
}

export const addSubreddit = async (subreddit: string) => {
    const res: boolean = await invoke("add_subreddit", { subreddit });
    return res;
}

export const removeSubreddit = async (subreddit: string) => {
    const res: boolean = await invoke("remove_subreddit", { subreddit });
    return res;
}

export const getImages = async (quantity: number) => {
    const res: string = await invoke("get_images", { quantity });
    return JSON.parse(res);
}