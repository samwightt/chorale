/**
 * Chorale: A blazing fast Notion page renderer.
 * Copyright (C) 2020 Sam Wight
 */
import { AppProps } from "next/app";
import "../styles/index.css";
import Layout from "../layout/main";

export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <Layout>
      <Component {...pageProps} />
    </Layout>
  );
}
