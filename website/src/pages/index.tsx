import clsx from "clsx";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import Heading from "@theme/Heading";
import styles from "./index.module.scss";
import DeskulptLogo from "@site/static/img/logo-wide.svg";

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<"svg">>;
  description: React.ReactNode;
};

const features: FeatureItem[] = [
  {
    title: "Lightweight and Fast",
    Svg: require("@site/static/img/tauri.svg").default,
    description: (
      <>
        Deskulpt is built with <a href="https://tauri.app/">Tauri</a>, which is
        cross-platform, fast, lightweight, and secure.
      </>
    ),
  },
];

function Feature(props: FeatureItem) {
  const { title, Svg, description } = props;

  return (
    <div className="col col--4">
      <p className="text--center">
        <Svg role="img" />
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
