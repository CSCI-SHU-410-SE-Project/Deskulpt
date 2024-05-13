import Logo from "/deskulpt.svg";

/**
 * The about tab in the manager.
 */
export default function AboutTab() {
  return (
    <div
      css={{
        height: "300px",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <div css={{ flex: 1 }}>
        <div css={{ textAlign: "center" }}>
          <img src={Logo} alt="Deskulpt" width={150} />
        </div>
      </div>
      <div css={{ flex: 2 }}>
        <div css={{ fontWeight: "bold", fontSize: "1.2rem" }}>Deskulpt v1.0.0</div>
        <div>A Cross-platform Desktop Customization Tool</div>
        <p>
          <div>
            <strong>Version:</strong> 0.0.0 (Under Development)
          </div>
          <div>
            <strong>Authors:</strong> Xinyu Li, Yao Xiao, Yuchen Zhou, Runkai Zhu
          </div>
          <div>
            <strong>Repository:</strong>{" "}
            <a
              href="https://github.com/CSCI-SHU-410-SE-Project/Deskulpt"
              target="_blank"
              rel="noreferrer"
            >
              https://github.com/CSCI-SHU-410-SE-Project/Deskulpt
            </a>
          </div>
          <div>
            <strong>Documentation:</strong>{" "}
            <a
              href="https://csci-shu-410-se-project.github.io/Deskulpt/"
              target="_blank"
              rel="noreferrer"
            >
              https://csci-shu-410-se-project.github.io/Deskulpt/
            </a>
          </div>
        </p>
      </div>
    </div>
  );
}
