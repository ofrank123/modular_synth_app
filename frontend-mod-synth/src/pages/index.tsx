import type { NextPage } from "next";
import Head from "next/head";
import styles from "../styles/Home.module.scss";
import { Oscilloscope } from "../components/Oscilloscope";
import { ModuleArea } from "../components/Modules";
import { Controls } from "../components/Controls";

const Home: NextPage = () => {
  return (
    <div className={styles.container}>
      <Head>
        <title>Modular Synth App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div style={{ display: "flex" }}>
        <Controls />
        <div style={{ marginLeft: "1rem", width: "100%" }}>
          <ModuleArea />
          <Oscilloscope />
        </div>
      </div>
    </div>
  );
};

export default Home;
