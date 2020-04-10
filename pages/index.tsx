import { GetServerSideProps } from 'next'
import { LoadPageChunkData } from '../types/notion'
import fetch from 'isomorphic-unfetch'

interface NotionProps {
  blockMap: LoadPageChunkData["recordMap"]["block"]
  currentID: string
}

const NotionRenderer: React.FC<NotionProps> = (props) => {
  const currentBlock = props.blockMap[props.currentID]

  if (currentBlock.value.type === "page") {
    return <div>
      <h1 className="font-serif text-black text-5xl font-bold text-center">{currentBlock.value.properties.title[0][0]}</h1>
      {currentBlock.value.content?.map((item, index) => {
        return <NotionRenderer key={index} blockMap={props.blockMap} currentID={item}/>
      })}
    </div>
  }

  if (currentBlock.value.type === "text") {
    return <div className="mx-5">
      <p className="text-black text-md font-sans"><>{currentBlock.value.properties.title.map((item, index) => {
        let newItem: any = item[0]
        if (item.length > 1) {
          item[1].forEach((item: any) => {
            if (item[0] === "b") newItem = <b key={index}>{newItem}</b>
            if (item[0] === "i") newItem = <em key={index}>{newItem}</em>
            if (item[0] === "s") newItem = <s key={index}>{newItem}</s>
          })
        }
        return newItem
      })}</></p>
      {currentBlock.value.content?.map((item, index) => {
        return <NotionRenderer blockMap={props.blockMap} key={index} currentID={item}/>
      })}
    </div>
  }

  return <div/>
}

interface HomeProps {
  blocks: LoadPageChunkData["recordMap"]["block"]
}


const Home: React.FC<HomeProps> = (props) => {
  return <>
  <NotionRenderer blockMap={props.blocks} currentID="ef28925f-6389-4c1d-962d-a11c86879897"/>
    {/* <div className="py-24 flex flex-col items-center">
      <img src={props.doc.data.profile_image.url}/>
      <h1 className="font-serif text-black text-5xl font-bold mt-5 text-center">{RichText.asText(props.doc.data.name)}</h1>
      <p className="max-w-lg text-black text-xl font-serif text-center">{RichText.asText(props.doc.data.tagline)}</p>
    </div> */}
  </>
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