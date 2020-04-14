/**
 * Chorale: A blazing fast Notion page renderer.
 * Copyright (C) 2020 Sam Wight
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
import {
  LoadPageChunkData,
  DecorationType,
  ColorType,
  BaseTextValueType,
} from "../types/notion";
import Link from "next/link";

interface NotionProps {
  blockMap: LoadPageChunkData["recordMap"]["block"];
  currentID: string;
  level: number;
}

const getTextContent = (text: DecorationType[]) => {
  return text.reduce((prev, current) => prev + current[0], "");
};

export const getTitle = (
  blockMap: LoadPageChunkData["recordMap"]["block"],
  id: string
) => {
  const currentBlock = blockMap[id];
  if (currentBlock.value.type === "page" && currentBlock.value.properties) {
    return currentBlock.value.properties.title[0][0];
  } else return "";
};

export const getDescription = (
  blockMap: LoadPageChunkData["recordMap"]["block"],
  id: string,
  level: number
): string | false => {
  const currentBlock = blockMap[id];
  const type = currentBlock.value.type;
  if (
    type === "bulleted_list" ||
    type === "text" ||
    type === "quote" ||
    type === "numbered_list"
  ) {
    const value = currentBlock.value as BaseTextValueType;
    if (value.properties?.title) return getTextContent(value.properties.title);
  }

  if (
    currentBlock.value.content &&
    currentBlock.value.content.length > 0 &&
    !(currentBlock.value.type === "page" && level > 0)
  ) {
    for (let i = 0; i < currentBlock.value.content.length; i++) {
      const value = getDescription(
        blockMap,
        currentBlock.value.content[i],
        level + 1
      );
      if (value) return value;
    }
  }

  return false;
};

const colorConverter = (color: ColorType, isBlock: boolean) => {
  const bgStyle = isBlock
    ? "text-black py-1 px-2 rounded"
    : "text-black py-1 px-1 rounded";

  switch (color) {
    case "blue":
      return "text-blue-600 border-blue-600";
    case "brown":
      return "text-orange-900 border-orange-900";
    case "gray":
      return "text-gray-500 border-gray-500";
    case "orange":
      return "text-orange-600 border-orange-600";
    case "pink":
      return "text-pink-600 border-pink-600";
    case "purple":
      return "text-purple-600 border-purple-600";
    case "red":
      return "text-red-600 border-red-600";
    case "teal":
      return "text-green-600 border-teal-600";
    case "yellow":
      return "text-yellow-600 border-yellow-600";
    case "blue_background":
      return "bg-blue-300 border-blue-600 " + bgStyle;
    case "brown_background":
      return "";
    case "gray_background":
      return "bg-gray-300 border-gray-500 " + bgStyle;
    case "orange_background":
      return "bg-orange-300 border-orange-600 " + bgStyle;
    case "pink_background":
      return "bg-pink-300 border-pink-600 " + bgStyle;
    case "purple_background":
      return "bg-purple-300 border-purple-600 " + bgStyle;
    case "red_background":
      return "bg-red-300 border-red-600 " + bgStyle;
    case "teal_background":
      return "bg-green-300 border-teal-600 " + bgStyle;
    case "yellow_background":
      return "bg-yellow-300 border-yellow-600 " + bgStyle;
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
              <span className={colorConverter(item[1], false)}>{newItem}</span>
            );
        }
      });
    }
    return newItem;
  });
};

interface BlockRenderer {
  block: LoadPageChunkData["recordMap"]["block"][""];
  level: number;
}

export const BlockRenderer: React.FC<BlockRenderer> = (props) => {
  const { block } = props;

  switch (block.value.type) {
    case "page":
      if (!block.value.properties) return null;
      if (props.level > 0)
        return (
          <>
            <Link
              href="/[url]"
              as={`/${props.block.value.id.replace(/-/g, "")}`}
            >
              <a className="text-black border-b-2 border-gray-300 font-bold font-sans py-1 px-3 bg-white hover:bg-gray-200 hover:border-gray-200 transition-all duration-150">
                {block.value.properties.title[0][0]}
              </a>
            </Link>
          </>
        );
      return (
        <>
          <h1 className="font-sans text-black text-4xl font-extrabold mx-4 mt-20 mb-10">
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
            colorConverter(block.value.format.block_color, true)
          }`}
        >
          <>{decorationsApplyer(block.value.properties.title)}</>
        </h1>
      );
    case "sub_header":
      if (!block.value.properties) return null;
      return (
        <h2
          className={`font-sans text-black text-2xl pt-4 font-medium font-bold ${
            block.value.format?.block_color &&
            colorConverter(block.value.format.block_color, true)
          }`}
        >
          <>{decorationsApplyer(block.value.properties.title)}</>
        </h2>
      );
    case "sub_sub_header":
      if (!block.value.properties) return null;
      return (
        <h3
          className={`font-sans text-black text-xl font-medium font-bold pt-3 ${
            block.value.format?.block_color &&
            colorConverter(block.value.format.block_color, true)
          }`}
        >
          <>{decorationsApplyer(block.value.properties.title)}</>
        </h3>
      );
    case "column_list":
      return null;
    case "quote":
      if (!block.value.properties) return null;
      return (
        <blockquote
          className={`font-sans text-black text-xl font-medium border-l-2 border-black pl-2 whitespace-pre-wrap ${
            block.value.format?.block_color &&
            colorConverter(block.value.format.block_color, true)
          }`}
        >
          <>{decorationsApplyer(block.value.properties.title)}</>
        </blockquote>
      );
    case "column":
      return null;
    case "divider":
      return <div className="w-full border-b border-black"></div>;
    case "text":
      if (!block.value.properties) {
        return <p className="h-4"> </p>;
      }
      return (
        <p
          className={`text-md leading-normal text-black text-md font-sans whitespace-pre-wrap ${
            block.value.format?.block_color &&
            colorConverter(block.value.format.block_color, true)
          }`}
        >
          <>{decorationsApplyer(block.value.properties.title)}</>
        </p>
      );
    case "bulleted_list":
    case "numbered_list":
      if (!block.value.properties) return null;
      return (
        <li
          className={`mt-2 leading-normal text-black text-md font-sans whitespace-pre-wrap ${
            block.value.format?.block_color &&
            colorConverter(block.value.format.block_color, true)
          }`}
        >
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
    if (!(currentId in blockMap)) continue;
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
  // if (
  //   (currentBlock.value.type === "header" ||
  //     currentBlock.value.type === "text" ||
  //     currentBlock.value.type === "bulleted_list") &&
  //   !currentBlock.value.properties
  // )
  //   return null;

  const renderChildren = !(
    currentBlock.value.type === "page" && props.level > 0
  );

  return (
    <>
      <BlockRenderer level={props.level} block={currentBlock} />
      {currentBlock.value.content && renderChildren && (
        <ChildRenderer
          level={props.level}
          ids={currentBlock.value.content}
          blockMap={props.blockMap}
        />
      )}
    </>
  );
};
