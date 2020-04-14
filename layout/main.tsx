/**
 * Chorale: A blazing fast Notion page renderer.
 * Copyright (C) 2020 Sam Wight
 */
import React from "react";
import Head from "next/head";

const Layout: React.FC = (props) => {
  return (
    <>
      <Head>
        <script
          async
          src="https://www.googletagmanager.com/gtag/js?id=UA-163568155-1"
        ></script>
        <script>
          {`
        window.dataLayer = window.dataLayer || [];
        function gtag(){dataLayer.push(arguments);}
        gtag('js', new Date());

        gtag('config', 'UA-163568155-1');`}
        </script>
      </Head>
      {props.children}
    </>
  );
};

export default Layout;
