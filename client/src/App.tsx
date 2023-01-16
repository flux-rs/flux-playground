import "./App.css";
import Editor from "@monaco-editor/react";
import VerifyButton from "./VerifyButton";

function App() {
  const options = { minimap: { enabled: false } };

  return (
    <div className="content">
      <div className="header">
        <VerifyButton />
      </div>
      <div className="editor">
        <Editor height="90vh" defaultLanguage="json" options={options} />
      </div>
    </div>
  );
}

export default App;
