import type {LayoutLoad} from './$types';

/**
 * Loads all instances
 */
export const load: LayoutLoad = () => {
    return {
        instances: [
            "modpack-1",
            "all-of-fabric-5",
            "all-the-mods-3",
            "among-us",
        ]
    }
};
