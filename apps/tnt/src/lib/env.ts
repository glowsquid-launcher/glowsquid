import {dev} from "$app/env"

export const serverURL = dev ? "http://localhost:8787/graphql" : "https://server.tnt-man-inc.workers.dev/graphql"
