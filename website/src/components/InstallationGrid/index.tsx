// @ts-nocheck

import Link from "@docusaurus/Link";
import Tabs from "@theme/Tabs";
import TabItem from "@theme/TabItem";
import usePlatform from "@site/src/hooks/usePlatform";
import IconTabLabel from "@site/src/components/IconTabLabel";
import { FaApple, FaLinux, FaWindows } from "react-icons/fa";

function DownloadOption(props: {
  version: string;
  artifact: string;
  children: JSX.Element;
}) {
  const { version, artifact, children } = props;

  return (
    <div
      className="container"
      style={{
        margin: "10px 0",
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
      }}
    >
      <span>{children}</span>
      <Link
        className="button button--secondary button--sm"
        href={`https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/releases/download/v${version}/${artifact}`}
        title={artifact}
      >
        Download
      </Link>
    </div>
  );
}

export default function InstallationGrid() {
  const platform = usePlatform("windows");

  return (
    <Tabs defaultValue={platform}>
      <TabItem
        value="windows"
        label={<IconTabLabel icon={<FaWindows />} title="Windows" />}
      >
        <DownloadOption version="0.0.1" artifact="Deskulpt_0.0.1_x64-setup.exe">
          <span>EXE installer (64-bit)</span>
        </DownloadOption>
        <DownloadOption version="0.0.1" artifact="Deskulpt_0.0.1_x64_en-US.msi">
          <span>MSI installer (64-bit)</span>
        </DownloadOption>
      </TabItem>
      <TabItem
        value="macos"
        label={<IconTabLabel icon={<FaApple />} title="MacOS" />}
      >
        <DownloadOption version="0.0.1" artifact="Deskulpt_0.0.1_aarch64.dmg">
          <span>Disk Image (64-bit, Apple Silicon)</span>
        </DownloadOption>
        <DownloadOption version="0.0.1" artifact="Deskulpt_aarch64.app.tar.gz">
          <span>Disk image (64-bit, Intel)</span>
        </DownloadOption>
        <DownloadOption version="0.0.1" artifact="Deskulpt_0.0.1_x64.dmg">
          <span>App bundle (64-bit, Apple Silicon)</span>
        </DownloadOption>
        <DownloadOption version="0.0.1" artifact="Deskulpt_x64.app.tar.gz">
          <span>App bundle (64-bit, Intel)</span>
        </DownloadOption>
      </TabItem>
      <TabItem
        value="linux"
        label={<IconTabLabel icon={<FaLinux />} title="Linux" />}
      >
        <DownloadOption
          version="0.0.1"
          artifact="deskulpt_0.0.1_amd64.AppImage"
        >
          <span>AppImage (64-bit, amd64)</span>
        </DownloadOption>
        <DownloadOption version="0.0.1" artifact="deskulpt-0.0.1-1.x86_64.rpm">
          <span>RPM installer (Red Hat, 64-bit, x86_64)</span>
        </DownloadOption>
        <DownloadOption version="0.0.1" artifact="deskulpt_0.0.1_amd64.deb">
          <span>DEB installer (Debian, 64-bit, amd64)</span>
        </DownloadOption>
      </TabItem>
    </Tabs>
  );
}
