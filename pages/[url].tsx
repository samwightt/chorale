import { GetServerSideProps } from 'next'
import { LoadPageChunkData } from '../types/notion'
import fetch from 'isomorphic-unfetch'
import { NotionRenderer } from '../components/notion'

interface HomeProps {
  blocks: LoadPageChunkData["recordMap"]["block"]
  id: string
}

const Home: React.FC<HomeProps> = (props) => {
  return <div className="container mx-auto">
    <NotionRenderer level={0} blockMap={props.blocks} currentID={props.id}/>
  </div>
}

export const getServerSideProps: GetServerSideProps = async (context) => {
  const id = context.params.url;
  let newId = "ef28925f-6389-4c1d-962d-a11c86879897"
  if (!Array.isArray(id)) {
    newId = id.split("-")[id.split("-").length - 1];
    newId = `${newId.slice(0, 8)}-${newId.slice(8, 12)}-${newId.slice(12, 16)}-${newId.slice(16, 20)}-${newId.slice(20, newId.length)}`
  }
  context.res.setHeader("Cache-Control", "s-maxage=1, stale-while-revalidate");

  var raw = JSON.stringify({"pageId":newId,"limit":200,"cursor":{"stack":[[{"table":"block","id":newId,"index":0}]]},"chunkNumber":0,"verticalColumns":false});

  var requestOptions = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: raw,
  };
  const res = await fetch('https://www.notion.so/api/v3/loadPageChunk', requestOptions)

  const data: LoadPageChunkData = await res.json();

  console.log(data)
  console.log(newId)

  return {
    props: {
      blocks: data.recordMap.block,
      id: newId
    }
  }
}

export default Home