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

import { GetServerSideProps } from "next";
import { LoadPageChunkData } from "../types/notion";
import fetch from "isomorphic-unfetch";
import { NotionRenderer, getTitle, getDescription } from "../components/notion";
import Head from "next/head";

interface HomeProps {
  blocks: LoadPageChunkData["recordMap"]["block"];
}

const Home: React.FC<HomeProps> = (props) => {
  const pageTitle = getTitle(
    props.blocks,
    "ef28925f-6389-4c1d-962d-a11c86879897"
  );
  const pageDescription = getDescription(
    props.blocks,
    "ef28925f-6389-4c1d-962d-a11c86879897",
    0
  );

  return (
    <>
      <Head>
        <title>{pageTitle}</title>
        <meta property="og:title" content={pageTitle} />
        <meta property="twitter:title" content={pageTitle} />
        <meta property="twitter:site" content="@samwightt" />
        <meta property="twitter:creator" content="@samwightt" />
        <meta property="twitter:card" content="summary" />
        {pageDescription && (
          <>
            <meta property="og:description" content={pageDescription} />
            <meta property="twitter:description" content={pageDescription} />
            <meta name="description" content={pageDescription} />
          </>
        )}
      </Head>
      <div className="container mx-auto max-w-4xl">
        <NotionRenderer
          level={0}
          blockMap={props.blocks}
          currentID="ef28925f-6389-4c1d-962d-a11c86879897"
        />
      </div>
    </>
  );
};

export const getServerSideProps: GetServerSideProps = async (context) => {
  context.res.setHeader("Cache-Control", "s-maxage=1, stale-while-revalidate");

  var raw = JSON.stringify({
    pageId: "ef28925f-6389-4c1d-962d-a11c86879897",
    limit: 200,
    cursor: {
      stack: [
        [
          {
            table: "block",
            id: "ef28925f-6389-4c1d-962d-a11c86879897",
            index: 0,
          },
        ],
      ],
    },
    chunkNumber: 0,
    verticalColumns: false,
  });

  var requestOptions = {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: raw,
  };
  const res = await fetch(
    "https://www.notion.so/api/v3/loadPageChunk",
    requestOptions
  );

  const data: LoadPageChunkData = await res.json();

  return {
    props: {
      blocks: data.recordMap.block,
    },
  };
};

export default Home;
