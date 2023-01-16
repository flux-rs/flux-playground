import "./App.css";
import Editor, { Monaco } from "@monaco-editor/react";
import VerifyButton from "./VerifyButton";
import React, { useRef, MutableRefObject, useState } from "react";
import { editor } from "monaco-editor";
import { Alert, AlertTitle, Snackbar, SnackbarOrigin, Stack } from "@mui/material";
import ResultAlert from "./ResultAlert";
import api from "./api";

function App() {
  const [status, setStatus] = useState(undefined as api.Status | undefined);
  const [fatalError, setFatalError] = useState(undefined as string | undefined);
  const [verifying, setVerifying] = useState(false);

  const options = { minimap: { enabled: false } };

  const monacoRef: MutableRefObject<Monaco | null> = useRef(null);
  const editorRef: MutableRefObject<editor.IStandaloneCodeEditor | null> = useRef(null);

  const alertAnchor: SnackbarOrigin = { vertical: "bottom", horizontal: "left" };

  const value = `#[allow(unused)]

#[flux::sig(fn() -> i32[10])]
fn mk_ten() -> i32 {
    4 + 5
}
`;

  const doVerify = () => {
    if (verifying) return;

    const monaco = monacoRef.current;
    const editor = editorRef.current;
    const code = editor?.getValue();
    if (monaco && editor && code) {
      setVerifying(true);
      api.verify(code).then((response) => {
        setVerifying(false);
        if ("error" in response) {
          setFatalError(response.error);
        } else {
          const model = editor.getModel();
          model && monaco.editor.setModelMarkers(model, "owner", response.markers || []);
          setStatus(response.status);
        }
      });
    }
  };

  const editorDidMount = (editor: editor.IStandaloneCodeEditor, monaco: Monaco) => {
    monacoRef.current = monaco;
    editorRef.current = editor;
    editor.getModel();
  };

  const closeFatalError = (event?: React.SyntheticEvent | Event, reason?: string) => {
    setFatalError(undefined);
  };

  const closeResult = (event: React.SyntheticEvent) => {
    const monaco = monacoRef.current;
    const editor = monacoRef.current;
    if (monaco && editor) {
      const model = monaco.editor.getModels()[0];
      monaco.editor.setModelMarkers(model, "owner", []);
      setStatus(undefined);
    }
  };

  return (
    <div className="content">
      <Stack direction="row" className="header" spacing={2}>
        <VerifyButton onClick={doVerify} verifying={verifying} />
        <ResultAlert status={status} onClose={closeResult} />
      </Stack>
      <div className="editor">
        <Editor
          value={value}
          height="90vh"
          defaultLanguage="rust"
          options={options}
          onMount={editorDidMount}
        />
      </div>
      <Snackbar
        open={fatalError !== undefined}
        anchorOrigin={alertAnchor}
        autoHideDuration={6000}
        onClose={closeFatalError}
      >
        <Alert severity="error" onClose={closeFatalError} sx={{ width: "100%" }} variant="filled">
          <AlertTitle>Internal Server Error</AlertTitle>
          {fatalError}
        </Alert>
      </Snackbar>
    </div>
  );
}

export default App;
