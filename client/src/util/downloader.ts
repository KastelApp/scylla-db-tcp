import { join } from "path";
const USER = "KastelApp";
const REPO = "scylla-db-tcp";

interface Release {
  url: string
  assets_url: string
  upload_url: string
  html_url: string
  id: number
  author: Author
  node_id: string
  tag_name: string
  target_commitish: string
  name: string
  draft: boolean
  prerelease: boolean
  created_at: string
  published_at: string
  assets: Asset[]
  tarball_url: string
  zipball_url: string
  body: string
}

interface Author {
  login: string
  id: number
  node_id: string
  avatar_url: string
  gravatar_id: string
  url: string
  html_url: string
  followers_url: string
  following_url: string
  gists_url: string
  starred_url: string
  subscriptions_url: string
  organizations_url: string
  repos_url: string
  events_url: string
  received_events_url: string
  type: string
  site_admin: boolean
}

interface Asset {
  url: string
  id: number
  node_id: string
  name: "scyllatcp-linux" | "scyllatcp-windows.exe"
  label: string | null
  uploader: Uploader
  content_type: string
  state: string
  size: number
  download_count: number
  created_at: string
  updated_at: string
  browser_download_url: string
}

interface Uploader {
  login: string
  id: number
  node_id: string
  avatar_url: string
  gravatar_id: string
  url: string
  html_url: string
  followers_url: string
  following_url: string
  gists_url: string
  starred_url: string
  subscriptions_url: string
  organizations_url: string
  repos_url: string
  events_url: string
  received_events_url: string
  type: string
  site_admin: boolean
}


export const getReleases = async (): Promise<Release[]> => {
    const res = await fetch(`https://api.github.com/repos/${USER}/${REPO}/releases`);
    return res.json() as Promise<Release[]>;
}

export const getLatestRelease = async (): Promise<Release> => {
    const releases = await getReleases();

    return releases[0];
}

export const downloadRelease = async () => {
    const release = await getLatestRelease();

    const type = process.platform === "win32" ? "windows" : "linux";

    const asset = release.assets.find((a) => a.name.includes(type));

    if (!asset) throw new Error("No asset found for this platform");

    const res = await fetch(asset.browser_download_url);

    const body = await res.arrayBuffer();

    const writer = Bun.file(join(import.meta.dirname, "..", "releases", "latest-" + asset.name)).writer()
    
    writer.write(new Uint8Array(body));

    writer.end();

    return;
}