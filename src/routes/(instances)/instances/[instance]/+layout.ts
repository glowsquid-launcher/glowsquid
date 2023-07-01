import type {LayoutLoad} from './$types';

/**
 * Loads data for instance page
 * @param params the id of the instance
 */
export const load: LayoutLoad = ({params}) => {
    return {
        id: params.instance
    };
};
