import NextAuth, { NextAuthOptions } from "next-auth";

export const authOptions: NextAuthOptions = {
  providers: [{
    id: "gitea",
    name: "Gitea",
    type: "oauth",
    wellKnown: `${process.env.OAUTH_GITEA_URL}/.well-known/openid-configuration`,
    authorization: { params: { scope: "openid profile email read:user repo:status" } },
    idToken: false,
    checks: [],
    clientId: process.env.OAUTH_APP_ID,
    clientSecret: process.env.OAUTH_CLIENT_SECRET,
    profile(profile) {
      console.log(profile)
      return {
        id: profile.sub,
        name: profile.name || profile.preferred_username,
        email: profile.email,
        image: profile.picture,
      }
    },
  }],
}

export default NextAuth(authOptions);