import "./App.css";
import Editor, { Monaco } from "@monaco-editor/react";
import VerifyButton from "./VerifyButton";
import React, { useRef, MutableRefObject, useState, useEffect } from "react";
import { editor } from "monaco-editor";
import {
  Box,
  FormControl,
  InputLabel,
  ListSubheader,
  MenuItem,
  Select,
  SelectChangeEvent,
  Stack,
} from "@mui/material";
import ResultAlert from "./ResultAlert";
import api from "./api";
import FatalError from "./FatalError";
import { Group } from "@mui/icons-material";

interface IExamplesMap {
  [key: string]: { groupName: string; examples: api.IExample[] };
}

function App() {
  const value = `#[allow(unused)]

#[flux::sig(fn() -> i32[10])]
fn mk_ten() -> i32 {
    4 + 5
}
`;
  const [fatalError, setFatalError] = useState(undefined as string | undefined);

  const [status, setStatus] = useState(undefined as api.Status | undefined);
  const [verifying, setVerifying] = useState(false);

  const [selectedExample, setSelectedExample] = useState("");
  const [examples, setExamples] = useState({} as IExamplesMap);

  const options = { minimap: { enabled: false } };

  const monacoRef: MutableRefObject<Monaco | null> = useRef(null);
  const editorRef: MutableRefObject<editor.IStandaloneCodeEditor | null> = useRef(null);

  useEffect(() => {
    api.listExamples().then((response) => {
      if ("error" in response) {
        setFatalError(response.error);
      } else {
        const map: IExamplesMap = {};
        for (const group of response.groups) {
          map[group.id] = { groupName: group.displayName, examples: [] };
        }
        for (const example of response.examples) {
          map[example.groupId]?.examples.push(example);
        }
        setExamples(map);
      }
    });
  }, []);

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
          setStatusAndMarkers(response.status, response.markers);
        }
      });
    }
  };

  const setStatusAndMarkers = (status: api.Status | undefined, markers: editor.IMarkerData[]) => {
    const monaco = monacoRef.current;
    const model = editorRef.current?.getModel();
    if (monaco && model) {
      monaco.editor.setModelMarkers(model, "owner", markers);
      setStatus(status);
    }
  };

  const editorDidMount = (editor: editor.IStandaloneCodeEditor, monaco: Monaco) => {
    monacoRef.current = monaco;
    editorRef.current = editor;
    editor.getModel();
  };

  const closeFatalError = (_event?: React.SyntheticEvent | Event, _reason?: string) => {
    setFatalError(undefined);
  };

  const closeResult = (_event: React.SyntheticEvent) => {
    setStatusAndMarkers(undefined, []);
  };

  const selectDemo = (event: SelectChangeEvent<string>) => {
    if (verifying) return;

    console.log(event.target.value);
    setSelectedExample(event.target.value);
    api.getExampleCode(event.target.value).then((response) => {
      if ("error" in response) {
        setFatalError(response.error);
      } else {
        editorRef.current?.setValue(response.code);
        setStatusAndMarkers(undefined, []);
      }
    });
  };

  const demoItems = [];
  let key = 0;
  for (const group of Object.values(examples)) {
    demoItems.push(<ListSubheader key={key++}>{group.groupName}</ListSubheader>);
    for (const example of group.examples) {
      demoItems.push(
        <MenuItem key={key++} value={example.fileName}>
          {example.displayName}
        </MenuItem>
      );
    }
  }

  return (
    <div className="content">
      <Stack>
        <h1>Flux Playground</h1>
        <Box sx={{ maxWidth: 150 }}>
          <FormControl fullWidth>
            <InputLabel>Select Demo</InputLabel>
            <Select label="Select Demo" onChange={selectDemo} value={selectedExample}>
              {demoItems.map((item) => item)}
            </Select>
          </FormControl>
        </Box>
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
        <FatalError message={fatalError} onClose={closeFatalError}></FatalError>
      </Stack>
    </div>
  );
}

export default App;
