import type { RequestHandler } from '@sveltejs/kit'
import { $fetch } from 'ohmyfetch'

/**
 * It takes an auth code, the original redirect URI, and a code type, and returns an object with an access token
 * and a refresh token
 * @param {string} authCode - The code you got from the first step
 * @param {string} redirectUri - The redirect URI you set in the Azure portal.
 * @param {'refresh_token' | 'code'} codeType - 'refresh_token' | 'code'
 * @returns An object with the following properties:
 *   - token: The token used to authenticate with the Xbox Live API
 *   - uhs: The user's Xbox Live User hash
 */
const getAuthToken = async (authCode: string, redirectUri: string, codeType: 'refresh_token' | 'code'): Promise<{
  accessToken: string,
  refreshToken: string
}> => {
  const url = new URL('https://login.live.com/oauth20_token.srf/')
  const body = new URLSearchParams()

  body.append('client_id', import.meta.env.MICROSOFT_CLIENT_ID)
  body.append('client_secret', import.meta.env.MICROSOFT_CLIENT_SECRET)
  body.append(codeType, authCode)
  body.append('grant_type', 'authorization_code')
  body.append('redirect_uri', redirectUri)

  const res = await $fetch(url.href, {
    method: 'POST',
    body
  })

  return {
    accessToken: res.access_token,
    refreshToken: res.refresh_token
  }
}

/**
 * It takes an access token and returns an XBL token and UHS
 * @param {string} accessToken - The access token you got from the first step
 * @returns An object with the token and uhs
 */
const getXBLToken = async (accessToken: string): Promise<{
  token: string,
  uhs: string
}> => {
  const res = await $fetch('https://user.auth.xboxlive.com/user/authenticate', {
    method: 'POST',
    body: {
      Properties: {
        AuthMethod: 'RPS',
        SiteName: 'user.auth.xboxlive.com',
        RpsTicket: `d=${accessToken}`
      },
      RelyingParty: 'http://auth.xboxlive.com',
      TokenType: 'JWT'
    }
  })

  return ({
    token: res.Token,
    uhs: res.DisplayClaims.xui[0].uhs
  })
}

/**
 * It takes an access token and returns a JWT token and a UHS
 * @param {string} accessToken - The access token you got from XBL
*/
const getXSTSToken = async (accessToken: string): Promise<{
  token: string,
  uhs: string}> => {
  const res = await $fetch('https://xsts.auth.xboxlive.com/xsts/authorize', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Accept: 'application/json'
    },
    body: {
      Properties: {
        SandboxId: 'RETAIL',
        UserTokens: [accessToken]
      },
      RelyingParty: 'rp://api.minecraftservices.com/',
      TokenType: 'JWT'
    }
  })

  return {
    token: res.Token,
    uhs: res.DisplayClaims.xui[0].uhs
  }
}

/**
 * It takes a XSTS token and a User hash, and returns a Minecraft username and access token
 * @param {string} xstsToken - The XSTS token you got from the previous step
 * @param {string} uhs - The User hash from the previous step.
 * @returns the accessToken
 */
const getMinecraftToken = async (xstsToken: string, uhs: string): Promise<string> => {
  const res = await $fetch('https://api.minecraftservices.com/authentication/login_with_xbox', {
    method: 'POST',
    body: {
      identityToken: `XBL3.0 x=${uhs};${xstsToken}`
    }
  })

  return res.access_token
}

/**
 * It takes a token and returns the Minecraft profile ID associated with it
 * @param {string} token - The token you got from the login request
 * @returns The id of the user's Minecraft profile
 */
const getMinecraftProfileId = async (token:string): Promise<string> => {
  const res = await $fetch('https://api.minecraftservices.com/minecraft/profile', {
    headers: {
      Authorization: `Bearer ${token}`
    }
  })

  return res.id
}

/**
 * authenticates a user with the Xbox Live API
 * @param request - request from sveltekit
 * @returns the minecraft uuid, token, and microsoft refresh token
 */
export const get: RequestHandler = async ({ url }) => {
  const code = url.searchParams.get('code')
  const refreshToken = url.searchParams.get('refreshToken')

  const redirectUri = new URL(url)
  redirectUri.searchParams.forEach((_v, k) => redirectUri.searchParams.delete(k))

  const codeType = code ? 'code' : 'refresh_token'
  const codeToSend = code || refreshToken
  if (!codeToSend) throw new Error('No code or refresh token provided')
  const oauthTokens = await getAuthToken(codeToSend, redirectUri.href, codeType)

  const XBLToken = await getXBLToken(oauthTokens.accessToken)
  const XSTSToken = await getXSTSToken(XBLToken.token).catch(err => {
    const data = err.data
    switch (data.XErr) {
      case 2148916233:
        throw new Error('error.noMicrosoftAccount')
      case 2148916235:
        throw new Error('error.microsoftAccountCountryBanned')
      case 2148916236:
      case 2148916237:
        throw new Error('error.microsoftAccountSouthKoreaAdultVerificationNeeded')
      case 2148916238:
        throw new Error('error.microsoftAccountUnder18')
    }
    throw new Error('error.microsoftUnknownError')
  })

  if (XBLToken.uhs !== XSTSToken.uhs) throw new Error('UHS mismatch. Please try again.')

  const minecraftToken = await getMinecraftToken(XSTSToken.token, XBLToken.uhs)
  const minecraftId = await getMinecraftProfileId(minecraftToken)

  return {
    status: 201,
    body: {
      minecraftId,
      minecraftToken,
      refreshToken: oauthTokens.refreshToken
    }
  }
}
