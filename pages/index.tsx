import { GetServerSideProps } from 'next'

interface HomeProps {
  random: number
}

const Home: React.FC<HomeProps> = (props) => <h1>Random number: {props.random}</h1>

export const getServerSideProps: GetServerSideProps = async (context) => {
  context.res.setHeader("Cache-Control", "s-maxage=1, stale-while-revalidate")

  return {
    props: {
      random: Math.random()
    }
  }
}

export default Home