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
export const get: RequestHandler = async ({ url }) => {
  const refreshToken = url.searchParams.get('refreshToken')

  const redirectUri = url.protocol + '//' + url.host + url.pathname

  if (!refreshToken) throw new Error('error.noCode')
  const oauthTokens = await getAuthToken(
    refreshToken,
    redirectUri,
    'refresh_token'
  )
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

  return {
    status: 200,
    body: {
      minecraftId,
      minecraftToken,
      minecraftRefreshToken: oauthTokens.refreshToken,
      expiresIn: oauthTokens.expiresIn,
    },
  }
}
