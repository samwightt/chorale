import '../styles/index.css'
import Layout from '../layout/main'

export default function MyApp({ Component, pageProps }) {
  return <Layout><Component {...pageProps} /></Layout>
}