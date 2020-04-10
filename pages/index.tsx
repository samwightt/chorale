import { GetServerSideProps } from 'next'
import { LoadPageChunkData } from '../types/notion'
import fetch from 'isomorphic-unfetch'
import Head from 'next/head'

interface NotionProps {
  blockMap: LoadPageChunkData["recordMap"]["block"]
  currentID: string
}

const decorationsApplyer = (properties: any[]) => {
  return properties.map((item, index) => {
    let newItem: any = item[0]
    if (item.length > 1) {
      item[1].forEach((item: any) => {
        switch(item[0]) {
          case "b":
            newItem = <b key={index}>{newItem}</b>
            break;
          case "i":
            newItem = <em key={index}>{newItem}</em>
            break;
          case "s":
            newItem = <s key={index}>{newItem}</s>
            break;
          case "a":
            newItem = <a href={item[1]} key={index}>{newItem}</a>
            break;
          case "c":
            newItem = <code key={index}>{newItem}</code>
            break;
        }
      })
    }
    return newItem
  })
}

const NotionRenderer: React.FC<NotionProps> = (props) => {
  const currentBlock = props.blockMap[props.currentID]

  if (currentBlock.value.type === "page") {
    return <div>
      <Head><title>{currentBlock.value.properties.title[0][0]}</title></Head>
      <h1 className="font-serif text-black text-5xl font-bold text-center">{currentBlock.value.properties.title[0][0]}</h1>
      {currentBlock.value.content?.map((item, index) => {
        return <NotionRenderer key={index} blockMap={props.blockMap} currentID={item}/>
      })}
    </div>
  }
  
  if (currentBlock.value.type === "header") {
    if (!currentBlock.value.properties) return null
    return <div className="mx-5">
      <h1 className="font-serif text-black text-xl font-bold"><>{decorationsApplyer(currentBlock.value.properties.title)}</></h1>
      {
        currentBlock.value.content?.map((item, index) => {
          return <NotionRenderer key={index} blockMap={props.blockMap} currentID={item}/>
        })
      }
    </div>
  }

  if (currentBlock.value.type === "text") {
    if (!currentBlock.value.properties) return null
    return <div className="mx-5">
      <p className="text-black text-md font-sans"><>{decorationsApplyer(currentBlock.value.properties.title)}</></p>
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