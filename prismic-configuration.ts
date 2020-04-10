import { GetServerSideProps } from 'next'
import Prismic from 'prismic-javascript'

export const apiEndpoint = 'https://peronsal-site.cdn.prismic.io/api/v2'
export const accessToken = 'MC5Yam5haEJNQUFDQUFnUXc1.77-9MGTvv73vv73vv70aayTvv71w77-977-977-9EQJtHnNr77-9Nm_vv73vv73vv73vv73vv70177-977-977-9'

type Request = Parameters<GetServerSideProps>[0]['req']

export const Client = () => (
  Prismic.client(apiEndpoint, {})
)

// const createClientOptions = (req: Request = null, prismicAccessToken: string = "") => {
//   const reqOption = req ? { req } : {}
//   const accessTokenOption = prismicAccessToken ? { accessToken: prismicAccessToken } : {}
//   return {
//     ...reqOption,
//     ...accessTokenOption
//   }
// }