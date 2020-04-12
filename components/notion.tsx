import { LoadPageChunkData, DecorationType, ColorType } from "../types/notion";

interface NotionProps {
  blockMap: LoadPageChunkData["recordMap"]["block"];
  currentID: string;
  level: number;
}

const colorConverter = (color: ColorType) => {
  switch (color) {
    case "blue":
      return "text-blue-600";
    case "brown":
      return "text-orange-900";
    case "gray":
      return "text-gray-600";
    case "orange":
      return "text-orange-600";
    case "pink":
      return "text-pink-600";
    case "purple":
      return "text-purple-600";
    case "red":
      return "text-red-600";
    case "teal":
      return "text-teal-600";
    case "yellow":
      return "text-yellow-600";
    default:
      return "";
  }
};

export const decorationsApplyer = (properties: DecorationType[]) => {
  return properties.map((item, index) => {
    let newItem: JSX.Element = <>{item[0]}</>;
    if (item.length !== 1) {
      item[1].forEach((item) => {
        switch (item[0]) {
          case "b":
            newItem = <b key={index}>{newItem}</b>;
            break;
          case "i":
            newItem = <em key={index}>{newItem}</em>;
            break;
          case "s":
            newItem = <s key={index}>{newItem}</s>;
            break;
          case "a":
            newItem = (
              <a href={item[1]} key={index}>
                {newItem}
              </a>
            );
            break;
          case "c":
            newItem = (
              <code
                className="bg-gray-300 text-gray-800 rounded text-sm py-1 px-1 font-mono"
                key={index}
              >
                {newItem}
              </code>
            );
            break;
          case "h":
            newItem = (
              <span className={colorConverter(item[1])}>{newItem}</span>
            );
        }
      });
    }
    return newItem;
  });
};

interface BlockRenderer {
  block: LoadPageChunkData["recordMap"]["block"][""];
}

export const BlockRenderer: React.FC<BlockRenderer> = (props) => {
  const { block } = props;

  switch (block.value.type) {
    case "page":
      if (!block.value.properties) return null;
      return (
        <>
          <h1 className="font-sans text-black text-5xl font-extrabold text-center">
            {block.value.properties.title[0][0]}
          </h1>
        </>
      );
    case "header":
      if (!block.value.properties) return null;
      return (
        <h1
          className={`font-sans text-black text-3xl pt-5 font-bold ${
            block.value.format?.block_color &&
            colorConverter(block.value.format.block_color)
          }`}
        >
          <>{decorationsApplyer(block.value.properties.title)}</>
        </h1>
      );
    case "sub_header":
      if (!block.value.properties) return null;
      return (
        <h2 className="font-sans text-black text-2xl pt-4 font-medium font-bold">
          <>{decorationsApplyer(block.value.properties.title)}</>
        </h2>
      );
    case "sub_sub_header":
      if (!block.value.properties) return null;
      return (
        <h3 className="font-sans text-black text-xl font-medium font-bold pt-3">
          <>{decorationsApplyer(block.value.properties.title)}</>
        </h3>
      );
    case "column_list":
      return null;
    case "quote":
      if (!block.value.properties) return null;
      return (
        <blockquote className="font-sans text-black text-xl font-medium border-l-2 border-black pl-2 whitespace-pre-wrap">
          <>{decorationsApplyer(block.value.properties.title)}</>
        </blockquote>
      );
    case "column":
      return null;
    case "divider":
      return <div className="w-full border-b border-black"></div>;
    case "text":
      if (!block.value.properties) return null;
      return (
        <p className="text-md leading-normal text-black text-md font-sans whitespace-pre-wrap">
          <>{decorationsApplyer(block.value.properties.title)}</>
        </p>
      );
    case "bulleted_list":
    case "numbered_list":
      if (!block.value.properties) return null;
      return (
        <li className="mt-2 leading-normal text-black text-md font-sans whitespace-pre-wrap">
          <>{decorationsApplyer(block.value.properties.title)}</>
        </li>
      );
    case "image":
      return (
        <img
          src={`https://notion.so/image/${encodeURIComponent(
            block.value.properties.source[0][0]
          )}`}
        />
      );
    default:
      return <div />;
  }
};

interface ChildRendererProps {
  blockMap: LoadPageChunkData["recordMap"]["block"];
  level: number;
  ids: string[];
}

export const ChildRenderer: React.FC<ChildRendererProps> = (props) => {
  const { ids, blockMap } = props;

  let idArray = [];
  let bulletArray = [];
  let orderedArray = [];

  for (let i = 0; i < ids.length; i++) {
    const currentId = ids[i];
    const currentBlock = blockMap[currentId];
    if (currentBlock.value.type === "bulleted_list") {
      bulletArray.push(
        <NotionRenderer
          level={props.level + 1}
          currentID={ids[i]}
          blockMap={blockMap}
        />
      );
    } else if (currentBlock.value.type === "numbered_list") {
      orderedArray.push(
        <NotionRenderer
          level={props.level + 1}
          currentID={ids[i]}
          blockMap={blockMap}
        />
      );
    } else if (currentBlock.value.type === "column_list") {
      idArray.push(
        <div className="flex my-2 flex-wrap">
          <NotionRenderer
            level={props.level + 1}
            currentID={currentId}
            blockMap={blockMap}
          />
        </div>
      );
    } else if (currentBlock.value.type === "column") {
      const width = `${currentBlock.value.format.column_ratio * 100}%`;
      idArray.push(
        <div style={{ minWidth: "200px", width }}>
          <NotionRenderer
            level={props.level + 1}
            currentID={ids[i]}
            blockMap={blockMap}
          />
        </div>
      );
    } else {
      if (bulletArray.length > 0) {
        idArray.push(<ul className="mx-12 my-2 list-disc">{bulletArray}</ul>);
        bulletArray = [];
      }
      if (orderedArray.length > 0) {
        idArray.push(
          <ol className="mx-12 my-2 list-decimal" key={i}>
            {orderedArray}
          </ol>
        );
        orderedArray = [];
      }

      idArray.push(
        <div className="mx-5 my-2" key={i}>
          <NotionRenderer
            level={props.level + 1}
            currentID={ids[i]}
            blockMap={blockMap}
          />
        </div>
      );
    }
  }

  if (bulletArray.length > 0) {
    idArray.push(
      <ul className="mx-10 my-2 list-disc" key={idArray.length}>
        {bulletArray}
      </ul>
    );
  }
  if (orderedArray.length > 0) {
    idArray.push(
      <ol className="mx-10 my-2 list-decimal" key={idArray.length}>
        {orderedArray}
      </ol>
    );
  }
  return <>{idArray}</>;
};

export const NotionRenderer: React.FC<NotionProps> = (props) => {
  const currentBlock = props.blockMap[props.currentID];
  if (
    (currentBlock.value.type === "header" ||
      currentBlock.value.type === "text" ||
      currentBlock.value.type === "bulleted_list") &&
    !currentBlock.value.properties
  )
    return null;
  if (props.level > 0 && currentBlock.value.type === "page") return null;

  return (
    <>
      <BlockRenderer block={currentBlock} />
      {currentBlock.value.content && (
        <ChildRenderer
          level={props.level}
          ids={currentBlock.value.content}
          blockMap={props.blockMap}
        />
      )}
    </>
  );
};
