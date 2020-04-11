import { GetServerSideProps } from 'next'
import { LoadPageChunkData } from '../types/notion'
import fetch from 'isomorphic-unfetch'
import Head from 'next/head'

interface NotionProps {
  blockMap: LoadPageChunkData["recordMap"]["block"]
  currentID: string
}

const widthConverter = (width: number) => {
  switch(width) {
    case 0.5:
      return "w-1/2";
    case 0.4:
      return "w-2/5";
    case 0.3:
      return "w-1/3";
    case 0.2:
      return "w-1/5";
    case 0.1:
      return "w-1/12";
    case 0.6:
      return "w-3/5";
    case 0.7:
      return "w-2/3";
    case 0.8: 
      return "w-4/5";
    case 0.9:
      return "w-11/12";
  }
  return "w-auto";
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

interface BlockRenderer {
  block: LoadPageChunkData["recordMap"]["block"][""]
}

const BlockRenderer: React.FC<BlockRenderer> = (props) => {
  const { block } = props

  switch(block.value.type) {
    case "page":
      return <h1 className="font-serif text-black text-5xl font-bold text-center">{block.value.properties.title[0][0]}</h1>
    case "header":
      return <h1 className="font-serif text-black text-3xl font-bold"><>{decorationsApplyer(block.value.properties.title)}</></h1>
    case "column_list":
      return null;
    case "column":
      return null;
    case "text":
      return <p className="text-black text-md font-sans whitespace-pre-wrap"><>{decorationsApplyer(block.value.properties.title)}</></p>
    case "bulleted_list":
      return <li className="text-black text-md font-sans whitespace-pre-wrap"><>{decorationsApplyer(block.value.properties.title)}</></li>
    default:
      return <div/>
  }
}

interface ChildRendererProps {
  blockMap: LoadPageChunkData["recordMap"]["block"]
  ids: string[]
}

const ChildRenderer: React.FC<ChildRendererProps> = (props) => {
  const { ids, blockMap } = props

  let idArray = []
  let bulletArray = []
  let orderedArray = []

  for (let i = 0; i < ids.length; i++) {
    const currentId = ids[i]
    if (blockMap[currentId].value.type === "bulleted_list") {
      bulletArray.push(<NotionRenderer currentID={ids[i]} blockMap={blockMap}/>)
    }
    else if (blockMap[currentId].value.type === "column_list") {
      bulletArray.push(<div className="flex flex-wrap"><NotionRenderer currentID={currentId} blockMap={blockMap}/></div>)
    }
    else if (blockMap[currentId].value.type === "column") {
      let width = '100%';
      if (blockMap[currentId].value.format?.column_ratio) {
        width = `${blockMap[currentId].value.format?.column_ratio * 100}%`
      }
      bulletArray.push(<div style={{minWidth: '200px', width}}><NotionRenderer currentID={ids[i]} blockMap={blockMap}/></div>)
    }
    else {
      if (bulletArray.length > 0) {
        idArray.push(<ul className="mx-10 list-disc">{bulletArray}</ul>)
        bulletArray = []
      }

      idArray.push(<div className="mx-5"><NotionRenderer currentID={ids[i]} blockMap={blockMap}/></div>)
    }
  }

  if (bulletArray.length > 0) {
    idArray.push(<ul className="mx-10 list-disc">{bulletArray}</ul>)
  }
  return <>{idArray}</>
}

const NotionRenderer: React.FC<NotionProps> = (props) => {
  const currentBlock = props.blockMap[props.currentID]
  if ((currentBlock.value.type === "header" || currentBlock.value.type === "text" || currentBlock.value.type === "bulleted_list") && !currentBlock.value.properties) return null

  return <>
    <BlockRenderer block={currentBlock}/>
    {currentBlock.value.content && <ChildRenderer ids={currentBlock.value.content} blockMap={props.blockMap} />}
  </>
}

interface HomeProps {
  blocks: LoadPageChunkData["recordMap"]["block"]
}


const Home: React.FC<HomeProps> = (props) => {
  return <>
  <NotionRenderer blockMap={props.blocks} currentID="ef28925f-6389-4c1d-962d-a11c86879897"/>
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