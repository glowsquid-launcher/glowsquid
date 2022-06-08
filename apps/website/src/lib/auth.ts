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
export const getAuthToken = async (
  authCode: string,
  redirectUri: string,
  codeType: 'refresh_token' | 'code'
): Promise<{
  accessToken: string
  refreshToken: string
  expiresIn: number
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
    body,
  })

  return {
    accessToken: res.access_token,
    refreshToken: res.refresh_token,
    expiresIn: res.expires_in,
  }
}

/**
 * It takes an access token and returns an XBL token and UHS
 * @param {string} accessToken - The access token you got from the first step
 * @returns An object with the token and uhs
 */
export const getXBLToken = async (
  accessToken: string
): Promise<{
  token: string
  uhs: string
}> => {
  const res = await $fetch('https://user.auth.xboxlive.com/user/authenticate', {
    method: 'POST',
    body: {
      Properties: {
        AuthMethod: 'RPS',
        SiteName: 'user.auth.xboxlive.com',
        RpsTicket: `d=${accessToken}`,
      },
      RelyingParty: 'http://auth.xboxlive.com',
      TokenType: 'JWT',
    },
  })

  return {
    token: res.Token,
    uhs: res.DisplayClaims.xui[0].uhs,
  }
}

/**
 * It takes an access token and returns a JWT token and a UHS
 * @param {string} accessToken - The access token you got from XBL
 */
export const getXSTSToken = async (
  accessToken: string
): Promise<{
  token: string
  uhs: string
}> => {
  const res = await $fetch('https://xsts.auth.xboxlive.com/xsts/authorize', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    },
    body: {
      Properties: {
        SandboxId: 'RETAIL',
        UserTokens: [accessToken],
      },
      RelyingParty: 'rp://api.minecraftservices.com/',
      TokenType: 'JWT',
    },
  })

  return {
    token: res.Token,
    uhs: res.DisplayClaims.xui[0].uhs,
  }
}

/**
 * It takes a XSTS token and a User hash, and returns a Minecraft username and access token
 * @param {string} xstsToken - The XSTS token you got from the previous step
 * @param {string} uhs - The User hash from the previous step.
 * @returns the accessToken
 */
export const getMinecraftToken = async (
  xstsToken: string,
  uhs: string
): Promise<string> => {
  const res = await $fetch(
    'https://api.minecraftservices.com/authentication/login_with_xbox',
    {
      method: 'POST',
      body: {
        identityToken: `XBL3.0 x=${uhs};${xstsToken}`,
      },
    }
  )

  return res.access_token
}

/**
 * It takes a token and returns the Minecraft profile ID associated with it
 * @param {string} token - The token you got from the login request
 * @returns The id of the user's Minecraft profile
 */
export const getMinecraftProfileId = async (token: string): Promise<string> => {
  const res = await $fetch(
    'https://api.minecraftservices.com/minecraft/profile',
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  )

  return res.id
}
