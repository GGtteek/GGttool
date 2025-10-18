import { invoke } from "@tauri-apps/api/core";
import { message, Layout, Button, Typography } from "antd";
import "./App.css";

const { Header, Content, Footer } = Layout;
const { Title } = Typography;

function App() {
  const handleRunScript = async () => {
    try {
      await invoke("run_mas_script");
      message.success("✅ Script started successfully!");
    } catch {
      message.error("❌ Failed to run script");
    }
  };

  return (
    <Layout className="app-layout">
      <Header className="app-header">GGtteek</Header>

      <Content className="app-content">
        <Title level={3} style={{ color: "#fff" }}>
          Welcome GGttool
        </Title>
      </Content>

      <Footer className="app-footer">
        <Button type="primary" onClick={handleRunScript}>
          Run MAS Script
        </Button>
      </Footer>
    </Layout>
  );
}

export default App;
