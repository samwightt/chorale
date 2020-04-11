import { LoadPageChunkData } from '../types/notion'

interface NotionProps {
  blockMap: LoadPageChunkData["recordMap"]["block"]
  currentID: string
}

export const decorationsApplyer = (properties: any[]) => {
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

export const BlockRenderer: React.FC<BlockRenderer> = (props) => {
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
    case "numbered_list":
      return <li className="text-black text-md font-sans whitespace-pre-wrap"><>{decorationsApplyer(block.value.properties.title)}</></li>
    default:
      return <div/>
  }
}

interface ChildRendererProps {
  blockMap: LoadPageChunkData["recordMap"]["block"]
  ids: string[]
}

export const ChildRenderer: React.FC<ChildRendererProps> = (props) => {
  const { ids, blockMap } = props

  let idArray = []
  let bulletArray = []
  let orderedArray = []

  for (let i = 0; i < ids.length; i++) {
    const currentId = ids[i]
    if (blockMap[currentId].value.type === "bulleted_list") {
      bulletArray.push(<NotionRenderer currentID={ids[i]} blockMap={blockMap}/>)
    }
    else if (blockMap[currentId].value.type === "numbered_list") {
      orderedArray.push(<NotionRenderer currentID={ids[i]} blockMap={blockMap}/>)
    }
    else if (blockMap[currentId].value.type === "column_list") {
      idArray.push(<div className="flex my-2 flex-wrap"><NotionRenderer currentID={currentId} blockMap={blockMap}/></div>)
    }
    else if (blockMap[currentId].value.type === "column") {
      let width = '100%';
      if (blockMap[currentId].value.format?.column_ratio) {
        width = `${blockMap[currentId].value.format?.column_ratio * 100}%`
      }
      idArray.push(<div style={{minWidth: '200px', width}}><NotionRenderer currentID={ids[i]} blockMap={blockMap}/></div>)
    }
    else {
      if (bulletArray.length > 0) {
        idArray.push(<ul className="mx-10 my-2 list-disc">{bulletArray}</ul>)
        bulletArray = []
      }
      if (orderedArray.length > 0) {
        idArray.push(<ol className="mx-10 my-2 list-decimal" key={i}>{orderedArray}</ol>)
        orderedArray = []
      }

      idArray.push(<div className="mx-5 my-2" key={i}><NotionRenderer currentID={ids[i]} blockMap={blockMap}/></div>)
    }
  }

  if (bulletArray.length > 0) {
    idArray.push(<ul className="mx-10 my-2 list-disc" key={idArray.length}>{bulletArray}</ul>)
  }
  if (orderedArray.length > 0) {
    idArray.push(<ol className="mx-10 my-2 list-decimal" key={idArray.length}>{bulletArray}</ol>)
  }
  return <>{idArray}</>
}

export const NotionRenderer: React.FC<NotionProps> = (props) => {
  const currentBlock = props.blockMap[props.currentID]
  if ((currentBlock.value.type === "header" || currentBlock.value.type === "text" || currentBlock.value.type === "bulleted_list") && !currentBlock.value.properties) return null

  return <>
    <BlockRenderer block={currentBlock}/>
    {currentBlock.value.content && <ChildRenderer ids={currentBlock.value.content} blockMap={props.blockMap} />}
  </>
}