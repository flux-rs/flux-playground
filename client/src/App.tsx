import "./App.css";
import Editor, { Monaco, useMonaco } from "@monaco-editor/react";
import VerifyButton from "./VerifyButton";
import { useEffect, useRef, MutableRefObject } from "react";
import { editor } from "monaco-editor";

function App() {
  const options = { minimap: { enabled: false } };

  const monacoRef: MutableRefObject<Monaco | null> = useRef(null);

  const doVerify = () => {
    const monaco = monacoRef.current;
    if (monaco) {
      const markers = [];
      markers.push({
        message: "this is an error",
        severity: monaco.MarkerSeverity.Error,
        startLineNumber: 0,
        startColumn: 0,
        endLineNumber: 0,
        endColumn: 5,
      });
      const model = monaco.editor.getModels()[0];
      monaco.editor.setModelMarkers(model, "owner", markers);
    }
  };

  const editorDidMount = (editor: editor.IStandaloneCodeEditor, monaco: Monaco) => {
    monacoRef.current = monaco;
  };

  return (
    <div className="content">
      <div className="header">
        <VerifyButton onClick={doVerify} />
      </div>
      <div className="editor">
        <Editor height="90vh" defaultLanguage="rust" options={options} onMount={editorDidMount} />
      </div>
    </div>
  );
}

export default App;
