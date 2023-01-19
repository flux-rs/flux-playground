import Editor, { Monaco } from "@monaco-editor/react";
import VerifyButton from "./VerifyButton";
import React, { useRef, MutableRefObject, useState, useEffect } from "react";
import { editor } from "monaco-editor";
import Box from "@mui/material/Box";
import Container from "@mui/material/Container";
import ResultAlert from "./ResultAlert";
import api from "./api";
import FatalError from "./FatalError";
import EditorToolbar from "./EditorToolbar";
import { SelectChangeEvent } from "@mui/material/Select/SelectInput";
import ListSubheader from "@mui/material/ListSubheader";
import MenuItem from "@mui/material/MenuItem";
import Stack from "@mui/material/Stack";
import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select from "@mui/material/Select";

interface IExamplesMap {
  [key: string]: { groupName: string; examples: api.IExample[] };
}

function App() {
  const value = `#![allow(unused)]

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

  const monacoOptions: editor.IStandaloneEditorConstructionOptions = {
    minimap: { enabled: false },
  };

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
    window.addEventListener("resize", () => {
      editorRef.current?.layout({} as editor.IDimension);
    });

    return () => {
      window.removeEventListener("resize", () => {});
    };
  }, []);

  const doVerify = () => {
    if (verifying) return;

    const code = editorRef.current?.getValue();
    if (code) {
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

  const selectExample = (event: SelectChangeEvent<string>) => {
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

  const exampleItems = [];
  let key = 0;
  for (const group of Object.values(examples)) {
    exampleItems.push(<ListSubheader key={key++}>{group.groupName}</ListSubheader>);
    for (const example of group.examples) {
      exampleItems.push(
        <MenuItem key={key++} value={example.fileName}>
          {example.displayName}
        </MenuItem>
      );
    }
  }

  return (
    <Container className="content" maxWidth={false} sx={{ height: "100vh" }}>
      <Stack height="100%">
        <h1>Flux Playground</h1>
        <Box>
          <FormControl sx={{ minWidth: 160 }}>
            <InputLabel>Select Example</InputLabel>
            <Select label="Select Example" onChange={selectExample} value={selectedExample}>
              {exampleItems.map((item) => item)}
            </Select>
          </FormControl>
        </Box>
        <Stack direction="row" spacing={2} sx={{ padding: "1.25em 0", minHeight: "48px" }}>
          <VerifyButton onClick={doVerify} verifying={verifying} />
          <ResultAlert status={status} onClose={closeResult} />
        </Stack>
        <EditorToolbar />
        <Box
          sx={{
            border: (theme) => `1px solid ${theme.palette.divider}`,
            height: "100%",
          }}
        >
          <Editor
            className="editor"
            value={value}
            defaultLanguage="rust"
            options={monacoOptions}
            onMount={editorDidMount}
          />
        </Box>
        <FatalError message={fatalError} onClose={closeFatalError}></FatalError>
      </Stack>
    </Container>
  );
}

export default App;
