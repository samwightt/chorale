import { GetServerSideProps } from 'next'
import { LoadPageChunkData } from '../types/notion'
import fetch from 'isomorphic-unfetch'
import { NotionRenderer } from '../components/notion'

interface HomeProps {
  blocks: LoadPageChunkData["recordMap"]["block"]
}

const Home: React.FC<HomeProps> = (props) => {
  return <div className="container mx-auto">
    <NotionRenderer level={0} blockMap={props.blocks} currentID="ef28925f-6389-4c1d-962d-a11c86879897"/>
  </div>
}

export const getServerSideProps: GetServerSideProps = async (context) => {
  context.res.setHeader("Cache-Control", "s-maxage=1, stale-while-revalidate");

  var raw = JSON.stringify({"pageId":"ef28925f-6389-4c1d-962d-a11c86879897","limit":200,"cursor":{"stack":[[{"table":"block","id":"ef28925f-6389-4c1d-962d-a11c86879897","index":0}]]},"chunkNumber":0,"verticalColumns":false});

  var requestOptions = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: raw,
  };
  const res = await fetch('https://www.notion.so/api/v3/loadPageChunk', requestOptions)


  const data: LoadPageChunkData = await res.json();

  return {
    props: {
      blocks: data.recordMap.block
    }
  }
}

export default Home