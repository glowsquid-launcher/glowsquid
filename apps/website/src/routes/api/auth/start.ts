import type { RequestHandler } from '@sveltejs/kit'

/**
 * redirects the user to the auth page. The port specified is the port of the glowsquid localhost server
 * @param request - request from sveltekit
 * @returns a redirect to the MS auth url
 */
export const get: RequestHandler = async ({ url }) => {
  const port = url.searchParams.get('port')
  if (!port) throw new Error('error.noState')
  const msUrl = new URL('https://login.live.com/oauth20_authorize.srf')
  msUrl.searchParams.append('client_id', import.meta.env.MICROSOFT_CLIENT_ID)
  msUrl.searchParams.append('response_type', 'code')
  msUrl.searchParams.append(
    'redirect_uri',
    `${url.origin}/api/auth/authenticate`
  )
  msUrl.searchParams.append('scope', 'XboxLive.signin offline_access')
  msUrl.searchParams.append('state', port)

  return {
    status: 302,
    headers: {
      location: msUrl.href,
    },
  }
}
