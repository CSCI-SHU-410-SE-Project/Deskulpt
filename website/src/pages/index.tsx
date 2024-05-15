import clsx from "clsx";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import Heading from "@theme/Heading";
import Link from "@docusaurus/Link";
import styles from "./index.module.scss";
import DeskulptLogo from "@site/static/img/logo-wide.svg";
import { FaGithub } from "react-icons/fa";
import { MdOutlineDashboardCustomize } from "react-icons/md";

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<"svg">>;
  description: React.ReactNode;
};

const features: FeatureItem[] = [
  {
    title: "Lightweight, Fast, Secure",
    Svg: require("@site/static/img/tauri.svg").default,
    description: (
      <>
        Deskulpt is built with <a href="https://v2.tauri.app/">Tauri v2</a> with
        TypeScript frontend and Rust backend, which is cross-platform, fast,
        lightweight, and secure.
      </>
    ),
  },
  {
    title: "Highly Customizable",
    Svg: MdOutlineDashboardCustomize,
    description: (
      <>
        Deskulpt allows writing desktop widgets in{" "}
        <a href="https://react.dev/">React</a>, unlocking infinite possibilities for
        customization.
      </>
    ),
  },
];

function Feature(props: FeatureItem) {
  const { title, Svg, description } = props;

  return (
    <div className="col col--4">
      <p className="text--center">
        <Svg
          role="img"
          width={100}
          height={100}
          color="var(--ifm-color-primary-dark)"
        />
      </p>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();

  return (
    <header className={clsx("hero", styles.homepageHeader)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          <DeskulptLogo className="invert-on-dark" style={{ height: "120px" }} />
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div>
          <Link
            className="button button--secondary"
            to="/guides/getting-started/installation"
          >
            Download Now
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home() {
  return (
    <Layout title="Home">
      <HomepageHeader />
      <main>
        <section className={styles.homepageFeaturesSection}>
          <div className="container">
            <div className="row">
              {features.map((props, index) => (
                <Feature key={index} {...props} />
              ))}
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
