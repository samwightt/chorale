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
