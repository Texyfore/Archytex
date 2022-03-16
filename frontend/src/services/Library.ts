import Environment from "../env";

interface AssetDb {
  textures: Texture[];
  props: Prop[];
}

interface Texture {
  id: number;
  name: string;
  categories: string[];
  emissive: string | null;
}

interface Prop {
  id: number;
  name: string;
  dependencies: string[];
  categories: string[];
}

export async function getAssets(): Promise<AssetDb> {
  const res = await fetch(Environment.asset_repo_url);
  const db: AssetDb = await res.json();
  return db;
}

export type { AssetDb };
export type { Texture };
export type { Prop };
