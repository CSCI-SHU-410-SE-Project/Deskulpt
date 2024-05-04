import Logo from "/deskulpt.svg";

/**
 * The about tab in the manager.
 */

export default function AboutTab() {
  return (
    <div style={{ display: "flex", alignItems: "center", justifyContent: "center" }}>
      <div style={{ marginRight: "20px" }}>
        <img
          src={Logo}
          alt="Software Logo"
          style={{ width: "150px", height: "auto" }}
        />
      </div>
      <div style={{ textAlign: "left" }}>
        <h1 style={{ fontSize: "24px", marginBottom: "10px" }}>Deskulpt v1.0.0</h1>
        <p style={{ fontSize: "16px", marginBottom: "10px" }}>
          Created by: Xinyu Li, Yao Xiao, Yuchen Zhou, Runkai Zhu
        </p>
        <p style={{ fontSize: "16px", marginBottom: "10px" }}>
          A Cross-platform Desktop Customization Tool
        </p>
        <a
          href={"https://csci-shu-410-se-project.github.io/Deskulpt/"}
          target="_blank"
          rel="noreferrer"
          style={{ fontSize: "16px", textDecoration: "none", color: "#007bff" }}
        >
          Website Link not working
        </a>
      </div>
    </div>
  );
}
