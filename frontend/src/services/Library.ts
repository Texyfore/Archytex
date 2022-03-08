import Environment from "../env";

interface AssetDb {
    textures: Texture[];
    props: Prop[];
}

interface Texture {
    id: number,
    url: string,
    public: boolean,
}

interface Prop {
    id: number,
    url: string,
}

export async function getAssets(): Promise<AssetDb> {
    const res = await fetch(Environment.asset_db_url);
    const db: AssetDb = await res.json();
    return db;
}

export type { AssetDb };
export type { Texture };
export type { Prop };