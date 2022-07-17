import {
  getAuthToken,
  getMinecraftProfileId,
  getMinecraftToken,
  getXBLToken,
  getXSTSToken,
} from '$lib/auth'
import type { RequestHandler } from '@sveltejs/kit'

/**
 * authenticates a user with the Xbox Live API
 * @param request - request from sveltekit
 * @returns the minecraft uuid, token, and microsoft refresh token
 */
export const GET: RequestHandler = async ({ url }) => {
  const code = url.searchParams.get('code')

  const redirectUri = url.protocol + '//' + url.host + url.pathname

  if (!code) throw new Error('error.noCode')
  const oauthTokens = await getAuthToken(code, redirectUri, 'code')
  const XBLToken = await getXBLToken(oauthTokens.accessToken)
  const XSTSToken = await getXSTSToken(XBLToken.token).catch((err) => {
    const data = err.data
    switch (data.XErr) {
    case 2148916233:
      throw new Error('error.noMicrosoftAccount')
    case 2148916235:
      throw new Error('error.microsoftAccountCountryBanned')
    case 2148916236:
    case 2148916237:
      throw new Error(
        'error.microsoftAccountSouthKoreaAdultVerificationNeeded'
      )
    case 2148916238:
      throw new Error('error.microsoftAccountUnder18')
    }
    throw new Error('error.microsoftUnknownError')
  })

  if (XBLToken.uhs !== XSTSToken.uhs) {
    throw new Error('UHS mismatch. Please try again.')
  }

  const minecraftToken = await getMinecraftToken(XSTSToken.token, XBLToken.uhs)
  const minecraftId = await getMinecraftProfileId(minecraftToken)

  const state = url.searchParams.get('state')
  if (!state) throw new Error('error.noState')
  const port = Number(state)

  const finalUrl = new URL(`http://localhost:${port}/cb`)
  finalUrl.searchParams.append('minecraftId', minecraftId)
  finalUrl.searchParams.append('minecraftToken', minecraftToken)
  finalUrl.searchParams.append(
    'microsoftRefreshToken',
    oauthTokens.refreshToken
  )
  finalUrl.searchParams.append(
    'microsoftExpiresIn',
    oauthTokens.expiresIn.toString()
  )

  return {
    status: 301,
    headers: {
      location: finalUrl.href,
    },
  }
}
