const loginUrl = `${process.env.OAUTH_GITEA_URL}/login/oauth/authorize?client_id=${
    process.env.OAUTH_APP_ID
  }&redirect_uri=${encodeURI(
    `https://localhost:3001/api/auth/callback/gitea`
  )}&response_type=code`;
  

// export default function SignIn() {
//     return (<div>
//     <a href={loginUrl}>
//       <span className="block w-max text-sm font-semibold tracking-wide text-gray-700 transition duration-300 group-hover:text-blue-600 sm:text-base">
//         Continue with Gitea
//       </span>
//     </a>
//   </div>)
// }

export default function SignIn() {
  return (
<>
<div className="flex min-h-full flex-1 flex-col justify-center px-6 py-12 lg:px-8">
        <div className="sm:mx-auto sm:w-full sm:max-w-sm">
          <img
            className="mx-auto h-10 w-auto"
            src="./constructum-name-logo.svg"
            alt="Constructum Logo"
          />
          <h2 className="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
            Sign in to Constructum
          </h2>
        </div>

        <div className="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <div className="space-y-6">
              <a
                type="submit"
                className="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                href={loginUrl}
              >
                Sign in with Gitea
              </a>
            </div>
        </div>
      </div>
</>
  )
}